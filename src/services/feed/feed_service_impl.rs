use super::error::FeedServiceError;
use super::FeedService;
use super::Result;
use crate::config::Config;
use crate::models::article::Article;
use crate::models::parsed_feed::ParsedFeed;
use crate::models::parsed_feed::ParsedItem;
use crate::providers::favicon::FaviconProvider;
use crate::providers::feed_parser::FeedParser;
use crate::providers::html_processor::HtmlProcessor;
use crate::providers::image_processor::ImageProcessor;
use crate::providers::image_processor::ImageProcessorFsImpl;
use crate::repositories::feed_content::FeedContentRepository;
use crate::{models::feed::Feed, repositories::feed::FeedRepository};
use axum::async_trait;
use axum::body::Bytes;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use reqwest::Url;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::task::JoinSet;
use uuid::Uuid;

type ArticleContent = String;

pub struct FeedServiceImpl<FR, FCR, HP, FRP, FAP, FVP>
where
    FR: FeedRepository,
    FCR: FeedContentRepository,
    HP: HtmlProcessor + 'static,
    FRP: FeedParser + 'static,
    FAP: FeedParser + 'static,
    FVP: FaviconProvider + 'static,
{
    feed_repository: Arc<FR>,
    feed_content_repository: Arc<FCR>,
    html_processor: Arc<HP>,
    atom_parser: Arc<FRP>,
    rss_parser: Arc<FAP>,
    favicon_provider: Arc<FVP>,
    config: Arc<Config>,
    articles_router_path: &'static str,
}

impl<FR, FCR, HP, FRP, FAP, FVP> FeedServiceImpl<FR, FCR, HP, FRP, FAP, FVP>
where
    FR: FeedRepository,
    FCR: FeedContentRepository,
    HP: HtmlProcessor + 'static,
    FRP: FeedParser + 'static,
    FAP: FeedParser + 'static,
    FVP: FaviconProvider + 'static,
{
    pub fn new(
        feed_repository: Arc<FR>,
        feed_content_repository: Arc<FCR>,
        html_processor: Arc<HP>,
        atom_parser: Arc<FRP>,
        rss_parser: Arc<FAP>,
        favicon_provider: Arc<FVP>,
        config: Arc<Config>,
        articles_router_path: &'static str,
    ) -> Self {
        Self {
            feed_repository,
            feed_content_repository,
            html_processor,
            atom_parser,
            rss_parser,
            favicon_provider,
            config,
            articles_router_path,
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

    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed> {
        if let Ok(parsed_feed) = self.rss_parser.parse_feed(content) {
            Ok(parsed_feed)
        } else if let Ok(parsed_feed) = self.atom_parser.parse_feed(content) {
            Ok(parsed_feed)
        } else {
            Err(FeedServiceError::UnsupportedFormat)
        }
    }

    async fn process_html_content(
        content: &str,
        html_processor: Arc<HP>,
        image_processor: Arc<impl ImageProcessor>,
        feed_link: Arc<String>,
    ) -> Result<String> {
        let content = html_processor
            .process_html_article(content)
            .map_err(|e| FeedServiceError::Unexpected(anyhow::anyhow!(e)))?
            .to_owned();

        // Fix img src in contents
        let content = html_processor
            .fix_img_src(&content, &feed_link, &*image_processor)
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

        html_processor
            .sanitize(&content)
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }

    async fn process_rss_content(
        content: &str,
        html_processor: Arc<HP>,
        image_processor: Arc<impl ImageProcessor>,
        feed_link: Arc<String>,
    ) -> Result<String> {
        // Fix img src in contents
        let content = html_processor
            .fix_img_src(content, &feed_link, &*image_processor)
            .await
            .map_err(|e| FeedServiceError::Unexpected(e.into()))?;

        html_processor
            .sanitize(&content)
            .map_err(|e| FeedServiceError::Unexpected(e.into()))
    }

    /// This function processes a feed item and adds it to the feed's article list.
    ///
    /// If `download_content` is `true`, then the article is downloaded and processed, otherwise,
    /// it is only added to the list
    async fn process_parsed_item(
        download_content: bool,
        image_processor: Arc<impl ImageProcessor>,
        html_processor: Arc<HP>,
        feed_id: Uuid,
        feed_link: Arc<String>,
        item: ParsedItem,
    ) -> Result<(Article, Option<ArticleContent>)> {
        let article_id = Uuid::new_v4();

        let article_link = item.link.clone().ok_or_else(|| {
            FeedServiceError::Unexpected(anyhow::anyhow!(
                "an article for the feed {} does not have a link",
                feed_id
            ))
        })?;

        // If the items have content, we cache it in the fs
        let (html_parsed, content) = if let Some(content) = item.content {
            (false, Some(content))
        }
        // Otherwise we follow the link and download the html
        else if download_content {
            let article = Self::download_html_article(&article_link).await?;
            (true, Some(article))
        } else {
            (true, None)
        };

        let content = if let Some(content) = content {
            if html_parsed {
                Some(
                    Self::process_html_content(
                        &content,
                        html_processor,
                        image_processor,
                        feed_link,
                    )
                    .await?,
                )
            } else {
                Some(
                    Self::process_rss_content(&content, html_processor, image_processor, feed_link)
                        .await?,
                )
            }
        } else {
            None
        };

        let date = item.pub_date.unwrap_or_else(Utc::now);

        Ok((
            Article {
                id: article_id,
                feed_id,
                title: item.title,
                link: article_link,
                guid: item.guid.ok_or_else(|| {
                    FeedServiceError::Unexpected(anyhow::anyhow!(
                        "an article for the feed {} does not have a guid",
                        feed_id
                    ))
                })?,
                author: item.author,
                html_parsed,
                content: None,
                read: false,
                last_updated: date,
            },
            content,
        ))
    }

    // TODO: File paths are not responsibility of this service, they should be managed by
    // ImageProcessorFsImpl and FeedContentFsRepositoryImpl
    fn get_article_router_path(&self, feed_id: Uuid) -> String {
        format!("{}/{feed_id}", self.articles_router_path)
    }

    // TODO: File paths are not responsibility of this service, they should be managed by
    // ImageProcessorFsImpl and FeedContentFsRepositoryImpl
    fn get_article_file_path(&self, feed_id: Uuid) -> String {
        format!("{}/articles/{feed_id}", self.config.data_path)
    }
}

#[async_trait]
impl<FR, FCR, HP, FRP, FAP, FVP> FeedService for FeedServiceImpl<FR, FCR, HP, FRP, FAP, FVP>
where
    FR: FeedRepository + 'static,
    FCR: FeedContentRepository + 'static,
    HP: HtmlProcessor,
    FRP: FeedParser + 'static,
    FAP: FeedParser + 'static,
    FVP: FaviconProvider + 'static,
{
    async fn get_feed_list(&self) -> Result<Vec<Feed>> {
        Ok(self.feed_repository.get_feed_list().await?)
    }

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>> {
        Ok(self.feed_repository.get_feed(feed_id).await?)
    }

    async fn add_feed(&self, feed_url: Url) -> Result<()> {
        let content = Self::download_feed_content(feed_url.as_str()).await?;

        let parsed_feed = self.parse_feed(&content)?;

        let link_path = feed_url.path_segments();
        let link = if let Some(link_path) = link_path {
            let mut final_path = link_path.collect::<Vec<&str>>();
            final_path.remove(final_path.len() - 1);
            let final_path = final_path.join("/");
            let mut link = feed_url.origin().unicode_serialization();
            link.push('/');
            link.push_str(&final_path);
            link
        } else {
            feed_url.origin().unicode_serialization()
        };

        let feed_id = Uuid::new_v4();
        
        let favicon_url = self.favicon_provider
            .download_favicon(&link, feed_id.to_string().as_str())
            .await
            .ok()
            .flatten();

        let feed = Feed {
            id: feed_id,
            title: parsed_feed.title,
            link,
            url: feed_url.into(),
            favicon_url,
            last_updated: DateTime::default(),
            unread_count: 0,
        };

        self.feed_repository.add_feed(feed).await?;

        Ok(())
    }

    async fn get_channel(&self, feed_id: Uuid) -> Result<(Feed, Vec<Article>)> {
        let feed = self.feed_repository.get_feed(feed_id).await?;
        if let Some(feed) = feed {
            let mut articles = if Utc::now() - feed.last_updated
                > TimeDelta::minutes(self.config.minutes_to_check_for_updates.into())
            {
                let content = Self::download_feed_content(feed.url.as_str()).await?;
                let parsed_feed = self.parse_feed(&content)?;

                // First we check with the channel and the database if any of the articles is new
                let saved_articles = self.feed_repository.get_feed_articles(feed_id).await?;

                let articles_guid: HashSet<&str> = saved_articles
                    .iter()
                    .map(|article| &*article.guid)
                    .collect();

                let new_items: Vec<ParsedItem> = parsed_feed
                    .items
                    .into_iter()
                    .filter(|item| {
                        if let Some(guid) = &item.guid {
                            !articles_guid.contains(guid.as_str())
                        // We filter by link because if guid is not found, we identify the article
                        // by its link
                        } else if let Some(link) = &item.link {
                            !articles_guid.contains(link.as_str())
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

                if new_items.is_empty() {
                    self.feed_repository
                        .update_last_updated(feed_id, Utc::now())
                        .await?;
                    return Ok((feed, saved_articles));
                }

                let mut join_set: JoinSet<Result<(Article, Option<ArticleContent>)>> =
                    JoinSet::new();
                let feed_link = Arc::new(feed.link.clone());
                let router_path = self.get_article_router_path(feed_id);
                let file_path = self.get_article_file_path(feed_id);
                let image_processor = Arc::new(ImageProcessorFsImpl::new(router_path, file_path));

                // TODO: We are assuming the channel items are sorted by pub date desc

                // Create the articles from the parsed items
                let mut processed_html_articles = 0;
                for item in new_items {
                    let feed_link = feed_link.clone();
                    let img_processor = image_processor.clone();
                    let html_processor = self.html_processor.clone();

                    // If it is an HTML article, we add one to the counter
                    // If content exist then we process it anyway because we already donwloaded
                    // it... otherwise we only process it if we processed less than config.max_articles_qty_to_download
                    if item.content.is_none() {
                        processed_html_articles += 1;
                    }

                    let download_content =
                        if let Some(qty) = self.config.max_articles_qty_to_download {
                            processed_html_articles <= qty
                        } else {
                            true
                        };

                    //  Start a task to process the article
                    join_set.spawn(Self::process_parsed_item(
                        download_content,
                        img_processor,
                        html_processor,
                        feed_id,
                        feed_link,
                        item,
                    ));
                }

                let mut processed_articles = vec![];
                while let Some(Ok(article)) = join_set.join_next().await {
                    match article {
                        Ok((article, content)) => processed_articles.push((article, content)),
                        Err(e) => {
                            tracing::error!("there was an error processing an article: {e:?}")
                        }
                    }
                }

                let articles: Vec<&Article> = processed_articles.iter().map(|(a, _)| a).collect();
                // Add the articles
                self.feed_repository
                    .add_articles(feed_id, &articles)
                    .await?;

                let articles_contents: Vec<(&Article, &ArticleContent)> = processed_articles
                    .iter()
                    .filter_map(|(a, c)| c.as_ref().map(|c| (a, c)))
                    .collect();

                if let Err(e) = self
                    .feed_content_repository
                    .save_article_content(&articles_contents)
                    .await
                {
                    tracing::error!("there was an error saving articles content: {e:?}")
                }

                let mut articles: Vec<Article> =
                    processed_articles.into_iter().map(|(a, _)| a).collect();

                articles.extend(saved_articles);

                articles
            } else {
                self.feed_repository.get_feed_articles(feed_id).await?
            };

            articles.sort_by(|a1, a2| a2.last_updated.cmp(&a1.last_updated));

            Ok((feed, articles))
        } else {
            Err(FeedServiceError::FeedNotFound(feed_id))
        }
    }

    async fn get_item_content(&self, feed_id: Uuid, article_id: Uuid) -> Result<(Article, String)> {
        let content = self
            .feed_content_repository
            .get_article_content(feed_id, article_id)
            .await?;

        let article_data = self
            .feed_repository
            .get_article_description(feed_id, article_id)
            .await?;

        if let Some(article_data) = article_data {
            // If the content exist we serve it
            if let Some(content) = content {
                Ok((article_data, content))
            }
            // Otherwise, we download it on demand
            else {
                let feed = self.feed_repository.get_feed(feed_id).await?.ok_or(
                    FeedServiceError::Unexpected(anyhow::anyhow!(
                        "feed with id {feed_id} not found"
                    )),
                )?;
                let content = Self::download_html_article(&article_data.link).await?;

                let router_path = self.get_article_router_path(feed_id);
                let file_path = self.get_article_file_path(feed_id);
                let image_processor = Arc::new(ImageProcessorFsImpl::new(router_path, file_path));

                let processed_article = Self::process_html_content(
                    &content,
                    self.html_processor.clone(),
                    image_processor,
                    feed.link.into(),
                )
                .await?;

                self.feed_content_repository
                    .save_article_content(&[(&article_data, &processed_article)])
                    .await?;

                Ok((article_data, processed_article))
            }
        } else {
            Err(FeedServiceError::ArticleContentNotFound(
                article_id, feed_id,
            ))
        }
    }

    async fn mark_article_as_read(&self, feed_id: Uuid, article_id: Uuid) -> Result<()> {
        Ok(self
            .feed_repository
            .mark_article_as_read(feed_id, article_id)
            .await?)
    }

    async fn delete_feed(&self, feed_id: Uuid) -> Result<()> {
        // First verify that the feed exists
        let feed = self.feed_repository.get_feed(feed_id).await?;
        if feed.is_none() {
            return Err(FeedServiceError::FeedNotFound(feed_id));
        }

        // Delete the feed content files
        self.feed_content_repository
            .delete_feed_content(feed_id)
            .await?;

        // Delete the feed and its articles from the database
        Ok(self.feed_repository.delete_feed(feed_id).await?)
    }
}
