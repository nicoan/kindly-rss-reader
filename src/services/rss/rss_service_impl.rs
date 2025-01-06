use std::{collections::HashSet, error::Error, path::Path, sync::Arc};

use super::RssService;
use crate::{
    models::{article::Article, feed::Feed},
    providers::html_processor::HtmlProcessor,
    repositories::feed::FeedRepository,
};
use axum::async_trait;
use chrono::{DateTime, TimeDelta, Utc};
use rss::Item;
use tokio::{fs, io::AsyncWriteExt, task::JoinSet};
use uuid::Uuid;

#[derive(Clone)]
pub struct RssServiceImpl<FR, HP>
where
    FR: FeedRepository,
    HP: HtmlProcessor,
{
    feed_repository: Arc<FR>,
    html_processor: HP,
}

impl<FR, HP> RssServiceImpl<FR, HP>
where
    FR: FeedRepository,
    HP: HtmlProcessor,
{
    pub fn new(feed_repository: Arc<FR>, html_processor: HP) -> Self {
        Self {
            feed_repository,
            html_processor,
        }
    }
}

#[async_trait]
impl<FR, HP> RssService for RssServiceImpl<FR, HP>
where
    FR: FeedRepository,
    HP: HtmlProcessor,
{
    // TODO: Compare GUID and last_updated of entries to not re-download not updated entries
    async fn get_channel(
        &self,
        feed_id: Uuid,
    ) -> Result<(Feed, Vec<Article>), Box<dyn std::error::Error>> {
        let feed = self.feed_repository.get_feed(feed_id).await;
        if let Some(feed) = feed {
            // TODO: 120 must be a config
            if Utc::now() - feed.last_updated > TimeDelta::minutes(120) {
                let content = reqwest::get(&feed.url).await?.bytes().await?;
                let rss_channel = rss::Channel::read_from(&content[..])?;

                // First we check with the channel and the database if any of the articles is new
                let saved_articles = self.feed_repository.get_feed_articles(feed_id).await;

                let articles_guid: HashSet<&str> = saved_articles
                    .iter()
                    .map(|article| &*article.guid)
                    .collect();

                let channel_items: Vec<Item> = rss_channel
                    .items
                    .into_iter()
                    .filter(|item| !articles_guid.contains(&item.guid().unwrap().value()))
                    .collect();

                if channel_items.is_empty() {
                    self.feed_repository
                        .update_last_updated(feed_id, Utc::now())
                        .await;
                    return Ok((feed, saved_articles));
                }

                let mut join_set: JoinSet<Result<Article, Box<dyn Error + Sync + Send>>> =
                    JoinSet::new();
                let feed_link = Arc::new(feed.link.clone());
                // Create the articles from the channel items
                for article in channel_items {
                    let feed_link = feed_link.clone();
                    join_set.spawn(async move {
                        let article_id = Uuid::new_v4();

                        let file_path = format!("articles/{feed_id}/{article_id}.html");
                        let file_directory = Path::new(&file_path).parent().unwrap();
                        fs::create_dir_all(file_directory).await?;

                        // If the items have content, we cache it in the fs
                        let content = if let Some(content) = article.content() {
                            content
                        }
                        // Otherwise we follow the linka and download the html
                        else {
                            let article = reqwest::get(article.link().unwrap())
                                .await
                                .expect("unable to get content url")
                                .text()
                                .await
                                .expect("unable to get content from content url");

                            &HP::process_html_article(&article).expect("unable to extract article")
                        };

                        // Fix img src in contents
                        let content = &HP::fix_img_src(content, &feed_link, file_directory).await;
                        let content = &HP::sanitize(content);

                        // Create the file and write the content
                        let mut file = fs::File::create(&file_path).await?;
                        fs::File::write(&mut file, content.as_bytes()).await?;

                        let date = if let Some(date) = &article.pub_date {
                            DateTime::parse_from_rfc2822(date)
                                .unwrap()
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
                            link: article
                                .link()
                                .map(|link| link.to_owned())
                                .unwrap_or_else(|| "TODO: Fail, link is mandatory".to_owned()),
                            guid: article
                                .guid()
                                .map(|id| id.value.clone())
                                .unwrap_or_else(|| article_id.to_string()),
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
            Err("feed not found".into())
        }
    }

    async fn get_item_content(
        &self,
        feed_id: Uuid,
        article_id: Uuid,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(article_content) = self
            .feed_repository
            .get_article_content(feed_id, article_id)
            .await
        {
            Ok(article_content)
        } else {
            let article_description = self
                .feed_repository
                .get_article_description(feed_id, article_id)
                .await;

            if let Some(article) = article_description {
                let article = reqwest::get(article.guid)
                    .await
                    .expect("unable to get content url")
                    .text()
                    .await
                    .expect("unable to get content from content url");

                Ok(HP::process_html_article(&article).expect("unable to extract article"))
            } else {
                Err("Unable to get content ".into())
            }
        }
    }
}
