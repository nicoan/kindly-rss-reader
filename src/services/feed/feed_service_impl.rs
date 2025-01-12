use super::error::FeedServiceError;
use super::FeedService;
use super::Result;
use crate::models::article::Article;
use crate::providers::html_processor::HtmlProcessor;
use crate::providers::image_processor::ImageProcessorFsImpl;
use crate::{models::feed::Feed, repositories::feed::FeedRepository};
use axum::async_trait;
use axum::body::Bytes;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use reqwest::Url;
use rss::Item;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct FeedServiceImpl<FR, HP>
where
    FR: FeedRepository,
    HP: HtmlProcessor + 'static,
{
    feed_repository: Arc<FR>,
    html_processor: Arc<HP>,
}

impl<FR, HP> FeedServiceImpl<FR, HP>
where
    FR: FeedRepository,
    HP: HtmlProcessor + 'static,
{
    pub fn new(feed_repository: Arc<FR>, html_processor: Arc<HP>) -> Self {
        Self {
            feed_repository,
            html_processor,
        }
    }

    async fn download_feed_content(feed_url: &str) -> Result<Bytes> {
        reqwest::get(feed_url)
            .await
            .map_err(FeedServiceError::GettingFeed)?
            .bytes()
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }

    async fn download_html_article(article_url: &str) -> Result<String> {
        reqwest::get(article_url)
            .await
            .map_err(FeedServiceError::GettingArticle)?
            .text()
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }
}

#[async_trait]
impl<FR, HP> FeedService for FeedServiceImpl<FR, HP>
where
    FR: FeedRepository,
    HP: HtmlProcessor,
{
    async fn get_feed_list(&self) -> Result<Vec<Feed>> {
        Ok(self.feed_repository.get_feed_list().await?)
    }

    async fn add_feed(&self, feed_url: Url) -> Result<()> {
        let content = Self::download_feed_content(feed_url.as_str()).await?;

        let rss_channel = rss::Channel::read_from(&content[..])
            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

        let feed = Feed {
            id: Uuid::new_v4().to_string(),
            title: rss_channel.title,
            link: feed_url.origin().unicode_serialization(),
            url: feed_url.into(),
            favicon_url: None,
            last_updated: DateTime::default(),
        };

        self.feed_repository.add_feed(feed).await?;

        Ok(())
    }

    // TODO: Compare GUID and last_updated of entries to not re-download not updated entries
    async fn get_channel(&self, feed_id: Uuid) -> Result<(Feed, Vec<Article>)> {
        let feed = self.feed_repository.get_feed(feed_id).await?;
        if let Some(feed) = feed {
            // TODO: 120 must be a config
            if Utc::now() - feed.last_updated > TimeDelta::minutes(120) {
                let content = Self::download_feed_content(feed.url.as_str()).await?;
                let rss_channel = rss::Channel::read_from(&content[..])
                    .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

                // First we check with the channel and the database if any of the articles is new
                let saved_articles = self.feed_repository.get_feed_articles(feed_id).await;

                let articles_guid: HashSet<&str> = saved_articles
                    .iter()
                    .map(|article| &*article.guid)
                    .collect();

                let channel_items: Vec<Item> = rss_channel
                    .items
                    .into_iter()
                    .filter(|item| {
                        if let Some(guid) = &item.guid().map(|g| g.value()) {
                            !articles_guid.contains(guid)
                        // We filter by link because if guid is not found, we identify the article
                        // by its link
                        } else if let Some(link) = &item.link() {
                            !articles_guid.contains(link)
                        } else {
                            tracing::warn!(
                                r#"found an article for feed {} with id "{}" with no guid or link"#,
                                feed.title,
                                feed_id
                            );
                            false
                        }
                    })
                    .collect();

                if channel_items.is_empty() {
                    self.feed_repository
                        .update_last_updated(feed_id, Utc::now())
                        .await;
                    return Ok((feed, saved_articles));
                }

                let mut join_set: JoinSet<Result<Article>> = JoinSet::new();
                let feed_link = Arc::new(feed.link.clone());
                let file_path = format!("articles/{feed_id}");
                let image_processor = Arc::new(ImageProcessorFsImpl::new(file_path));
                // Create the articles from the channel items
                for article in channel_items {
                    let feed_link = feed_link.clone();
                    let img_processor = image_processor.clone();
                    let html_processor = self.html_processor.clone();
                    join_set.spawn(async move {
                        let article_id = Uuid::new_v4();

                        let file_path = format!("articles/{feed_id}/{article_id}.html");
                        let file_directory = Path::new(&file_path).parent().ok_or_else(|| {
                            FeedServiceError::Unexpected(anyhow::anyhow!(format!(
                                "there was an error getting the parent path for {file_path}"
                            )))
                        })?;
                        fs::create_dir_all(file_directory)
                            .await
                            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

                        let article_link =
                            article.link().map(|link| link.to_owned()).ok_or_else(|| {
                                FeedServiceError::Unexpected(anyhow::anyhow!(
                                    "an article for the feed {feed_id} does not have a link"
                                ))
                            })?;

                        // If the items have content, we cache it in the fs
                        let content = if let Some(content) = article.content() {
                            content
                        }
                        // Otherwise we follow the linka and download the html
                        else {
                            let article = Self::download_html_article(&article_link).await?;

                            // TODO: Create an HTML processor error
                            &html_processor
                                .process_html_article(&article)
                                .map_err(|e| FeedServiceError::Unexpected(anyhow::anyhow!(e)))?
                                .to_owned()
                        };

                        // Fix img src in contents
                        let content = html_processor
                            .fix_img_src(content, &feed_link, &*img_processor)
                            .await
                            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

                        let content = &html_processor
                            .sanitize(&content)
                            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

                        // Create the file and write the content
                        let mut file = fs::File::create(&file_path)
                            .await
                            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;
                        fs::File::write(&mut file, content.as_bytes())
                            .await
                            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

                        let date = if let Some(date) = &article.pub_date {
                            DateTime::parse_from_rfc2822(date)
                                .map_err(|e| {
                                    FeedServiceError::ParsingDate(article_link.clone(), e)
                                })?
                                .with_timezone(&Utc)
                        } else {
                            Utc::now()
                        };

                        Ok(Article {
                            id: article_id,
                            feed_id,
                            title: article
                                .title
                                .clone()
                                .unwrap_or_else(|| "Unknown title".to_owned()),
                            author: article.author.clone().unwrap_or_else(|| "".to_owned()),
                            link: article_link,
                            guid: article.guid().map(|id| id.value.clone()).ok_or_else(|| {
                                FeedServiceError::Unexpected(anyhow::anyhow!(
                                    "an article for the feed {feed_id} does not have a guid"
                                ))
                            })?,
                            path: Some(file_path),
                            read: false,
                            last_updated: date,
                        })
                    });
                }

                let mut articles = vec![];
                while let Some(Ok(article)) = join_set.join_next().await {
                    match article {
                        Ok(article) => articles.push(article),
                        Err(e) => {
                            tracing::error!("there was an error processing an article: {e:?}")
                        }
                    }
                }

                self.feed_repository.add_articles(feed_id, articles).await;
            }

            // TODO: Just prepend the new articles to saved_articles
            let articles = self.feed_repository.get_feed_articles(feed_id).await;

            Ok((feed, articles))
        } else {
            Err(FeedServiceError::FeedNotFound(feed_id))
        }
    }

    async fn get_item_content(&self, feed_id: Uuid, article_id: Uuid) -> Result<String> {
        self.feed_repository
            .get_article_content(feed_id, article_id)
            .await
            .ok_or(FeedServiceError::ArticleNotFound(article_id, feed_id))
    }
}
