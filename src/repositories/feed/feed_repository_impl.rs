use crate::{
    models::article::Article,
    repositories::{feed::Feed, RepositoryError},
};
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
    async fn get_feed_list(&self) -> Result<Vec<Feed>, RepositoryError> {
        self.connection
            .prepare("SELECT * FROM feed")
            .unwrap()
            .into_iter()
            .flat_map(|r| r.map(Feed::try_from))
            .collect()
    }

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>, RepositoryError> {
        self.connection
            .prepare("SELECT * FROM feed WHERE id = ?")
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .into_iter()
            .bind((1, feed_id.to_string().as_str()))
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .nth(0)
            .map(|r| {
                r.map_err(|e| RepositoryError::Unexpcted(e.into()))
                    .and_then(Feed::try_from)
            })
            .transpose()
    }

    async fn add_feed(&self, feed: Feed) -> Result<(), RepositoryError> {
        let statement = r#"
                        INSERT INTO feed (id, title, url, link, last_updated)
                        VALUES (?, ?, ?, ?, ?);
                    "#;

        self.connection
            .prepare(statement)
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .into_iter()
            .bind((1, feed.id.as_str()))
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .bind((2, feed.title.as_str()))
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .bind((3, feed.url.as_str()))
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .bind((4, feed.link.as_str()))
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?
            .bind((5, feed.last_updated.to_rfc3339().as_str()))
            .map_err(|e| RepositoryError::Unexpcted(e.into()))?;

        Ok(())
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
                Article {
                    id: Uuid::from_str(row.read::<&str, _>("id")).unwrap(),
                    feed_id: Uuid::from_str(row.read::<&str, _>("feed_id")).unwrap(),
                    title: row.read::<&str, _>("title").into(),
                    author: row.read::<&str, _>("author").into(),
                    link: row.read::<&str, _>("link").into(),
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
                    guid: row.read::<&str, _>("guid").into(),
                    link: row.read::<&str, _>("link").into(),
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
                    article.title.replace("'", "''"),
                    article.author.replace("'", "''"),
                    article.guid,
                    article.link,
                    &now,
                    article.path.as_deref().unwrap_or("NULL"),
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        let query = format!(
            r#"
            INSERT INTO article (id, feed_id, title, author, guid, link, last_updated, path, read)
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

    async fn update_last_updated(&self, feed_id: Uuid, date: DateTime<Utc>) {
        self.connection
            .execute(format!(
                "UPDATE feed SET last_updated = '{}' WHERE id = '{feed_id}'",
                date.to_rfc3339()
            ))
            .unwrap();
    }
}
