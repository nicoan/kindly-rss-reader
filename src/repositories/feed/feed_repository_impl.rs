use crate::{models::article::Article, repositories::feed::Feed};
use axum::async_trait;
use chrono::{DateTime, Utc};
use sqlite::ConnectionThreadSafe;
use std::{str::FromStr, sync::Arc};
use tokio::fs;
use uuid::Uuid;

use super::FeedRepository;

#[derive(Clone)]
pub struct FeedRepositoryImpl {
    connection: Arc<ConnectionThreadSafe>,
}

impl FeedRepositoryImpl {
    pub fn new(connection: Arc<ConnectionThreadSafe>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl FeedRepository for FeedRepositoryImpl {
    async fn get_feed_list(&self) -> Vec<Feed> {
        self.connection
            .prepare("SELECT * FROM feed")
            .unwrap()
            .into_iter()
            .map(|row| {
                let row = row.unwrap();
                Feed {
                    id: row.read::<&str, _>("id").into(),
                    title: row.read::<&str, _>("title").into(),
                    url: row.read::<&str, _>("url").into(),
                    favicon_url: row
                        .read::<Option<&str>, _>("favicon_path")
                        .map(|s| s.to_owned()),
                    last_updated: DateTime::from_str(row.read::<&str, _>("last_updated")).unwrap(),
                }
            })
            .collect()
    }

    async fn get_feed(&self, feed_id: Uuid) -> Option<Feed> {
        self.connection
            .prepare("SELECT * FROM feed WHERE id = ?")
            .unwrap()
            .into_iter()
            .bind((1, feed_id.to_string().as_str()))
            .unwrap()
            .nth(0)
            .map(|row| {
                let row = row.unwrap();
                Feed {
                    id: row.read::<&str, _>("id").into(),
                    title: row.read::<&str, _>("title").into(),
                    url: row.read::<&str, _>("url").into(),
                    favicon_url: row
                        .read::<Option<&str>, _>("favicon_path")
                        .map(|s| s.to_owned()),
                    last_updated: DateTime::from_str(row.read::<&str, _>("last_updated")).unwrap(),
                }
            })
    }

    async fn get_feed_articles(&self, feed_id: Uuid) -> Vec<Article> {
        self.connection
            .prepare("SELECT * FROM article WHERE feed_id = ?")
            .unwrap()
            .into_iter()
            .bind((1, feed_id.to_string().as_str()))
            .unwrap()
            .map(|row| {
                let row = row.unwrap();
                println!("{}", row.read::<&str, _>("last_updated"));
                Article {
                    id: Uuid::from_str(row.read::<&str, _>("id")).unwrap(),
                    feed_id: Uuid::from_str(row.read::<&str, _>("feed_id")).unwrap(),
                    title: row.read::<&str, _>("title").into(),
                    author: row.read::<&str, _>("author").into(),
                    description: row.read::<&str, _>("description").into(),
                    guid: row.read::<&str, _>("guid").into(),
                    path: row.read::<Option<&str>, _>("path").map(|s| s.to_owned()),
                    read: row.read::<i64, _>("read") != 0,
                    last_updated: DateTime::from_str(row.read::<&str, _>("last_updated")).unwrap(),
                }
            })
            .collect()
    }

    async fn get_article_description(&self, feed_id: Uuid, article_id: Uuid) -> Option<Article> {
        self.connection
            .prepare("SELECT * FROM article WHERE id = ? AND feed_id = ?")
            .unwrap()
            .into_iter()
            .bind((1, article_id.to_string().as_str()))
            .unwrap()
            .bind((2, feed_id.to_string().as_str()))
            .unwrap()
            .nth(0)
            .map(|row| {
                let row = row.unwrap();
                Article {
                    id: Uuid::from_str(row.read::<&str, _>("id")).unwrap(),
                    feed_id: Uuid::from_str(row.read::<&str, _>("feed_id")).unwrap(),
                    title: row.read::<&str, _>("title").into(),
                    author: row.read::<&str, _>("author").into(),
                    description: row.read::<&str, _>("description").into(),
                    guid: row.read::<&str, _>("guid").into(),
                    path: row.read::<Option<&str>, _>("path").map(|s| s.to_owned()),
                    read: row.read::<i64, _>("read") != 0,
                    last_updated: DateTime::from_str(row.read::<&str, _>("last_updated")).unwrap(),
                }
            })
    }

    async fn add_articles(&self, feed_id: Uuid, articles: Vec<Article>) {
        // TODO: Should not inject values directly... use prepare and ?
        let now = Utc::now().to_rfc3339();
        let values = articles
            .iter()
            .map(|article| {
                format!(
                    "('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', 0)",
                    Uuid::new_v4(),
                    feed_id,
                    article.title,
                    article.author,
                    article.description,
                    article.guid,
                    &now,
                    article.path.as_deref().unwrap_or("NULL"),
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        let query = format!(
            r#"
            INSERT INTO article (id, feed_id, title, author, description, guid, last_updated, path, read)
            VALUES {values}
            ON CONFLICT(guid) DO NOTHING;

            UPDATE feed SET last_updated = '{now}' WHERE id = '{feed_id}';
            "#,
        );

        self.connection.execute(query).unwrap();
    }

    async fn get_article_content(&self, feed_id: Uuid, article_id: Uuid) -> Option<String> {
        let file_path = self
            .connection
            .prepare("SELECT * FROM article WHERE id = ? AND feed_id = ?")
            .unwrap()
            .into_iter()
            .bind((1, article_id.to_string().as_str()))
            .unwrap()
            .bind((2, feed_id.to_string().as_str()))
            .unwrap()
            .nth(0)
            .and_then(|row| {
                let row = row.unwrap();
                row.read::<Option<&str>, _>("path").map(|s| s.to_owned())
            });

        if let Some(path) = file_path {
            fs::read_to_string(path).await.ok()
        } else {
            None
        }
    }
}
