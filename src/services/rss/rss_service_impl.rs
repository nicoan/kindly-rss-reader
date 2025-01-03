use std::{path::Path, sync::Arc};

use super::RssService;
use crate::{
    models::{article::Article, feed::Feed},
    repositories::feed::FeedRepository,
};
use axum::async_trait;
use chrono::{DateTime, TimeDelta, Utc};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

#[derive(Clone)]
pub struct RssServiceImpl<FR>
where
    FR: FeedRepository,
{
    feed_repository: Arc<FR>,
}

impl<FR> RssServiceImpl<FR>
where
    FR: FeedRepository,
{
    pub fn new(feed_repository: Arc<FR>) -> Self {
        Self { feed_repository }
    }
}

#[async_trait]
impl<FR> RssService for RssServiceImpl<FR>
where
    FR: FeedRepository,
{
    async fn get_channel(
        &self,
        feed_id: Uuid,
    ) -> Result<(Feed, Vec<Article>), Box<dyn std::error::Error>> {
        let feed = self.feed_repository.get_feed(feed_id).await;
        if let Some(feed) = feed {
            if Utc::now() - feed.last_updated > TimeDelta::minutes(60) {
                let content = reqwest::get(&feed.url).await?.bytes().await?;
                let rss_channel = rss::Channel::read_from(&content[..])?;

                let mut articles = vec![];
                // Create the articles from the channel items
                for article in &rss_channel.items {
                    let article_id = Uuid::new_v4();
                    // If the items have content, we cache them in the fs
                    let file_path = if let Some(content) = article.content() {
                        let file_path = format!("articles/{feed_id}/{article_id}.html");

                        // Get the parent directory of the file
                        if let Some(parent) = Path::new(&file_path).parent() {
                            // Create the nested directories
                            fs::create_dir_all(parent).await?;
                        }

                        // Create the file and write the content
                        let mut file = fs::File::create(&file_path).await?;
                        fs::File::write(&mut file, content.as_bytes()).await?;

                        Some(file_path)
                    } else {
                        None
                    };

                    let date = if let Some(date) = &article.pub_date {
                        DateTime::parse_from_str(date, "%a, %d %b %Y %H:%M:%S %z")
                            .unwrap()
                            .with_timezone(&Utc)
                    } else {
                        Utc::now()
                    };

                    let article_description = Article {
                        id: article_id,
                        feed_id,
                        title: article
                            .title
                            .clone()
                            .unwrap_or_else(|| "Unknown title".to_owned()),
                        author: article.author.clone().unwrap_or_else(|| "".to_owned()),
                        description: article
                            .description
                            .clone()
                            .unwrap_or_else(|| "No description".to_owned()),
                        guid: article
                            .guid()
                            .map(|id| id.value.clone())
                            .unwrap_or_else(|| article_id.to_string()),
                        path: file_path,
                        read: false,
                        last_updated: date,
                    };

                    articles.push(article_description);
                }

                self.feed_repository.add_articles(feed_id, articles).await;
            }
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
                Ok(reqwest::get(article.guid)
                    .await
                    .expect("unable to get content url")
                    .text()
                    .await
                    .expect("unable to get content from content url"))
            } else {
                Err("Unable to get content ".into())
            }
        }
    }
}
