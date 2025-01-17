use crate::{
    models::article::Article,
    repositories::{feed::Feed, RepositoryError},
};
use axum::async_trait;
use chrono::{DateTime, Utc};
use sqlite::ConnectionThreadSafe;
use std::sync::Arc;
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
            .prepare("SELECT * FROM feed;")?
            .into_iter()
            .flat_map(|r| r.map(Feed::try_from))
            .collect()
    }

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>, RepositoryError> {
        self.connection
            .prepare("SELECT * FROM feed WHERE id = ?;")?
            .into_iter()
            .bind((1, feed_id.to_string().as_str()))?
            .nth(0)
            .map(|r| {
                r.map_err(|e| RepositoryError::Unexpcted(e.into()))
                    .and_then(Feed::try_from)
            })
            .transpose()
    }

    async fn add_feed(&self, feed: Feed) -> Result<(), RepositoryError> {
        self.connection.execute("BEGIN")?;
        let mut stmt = self.connection.prepare(
            r#"
                    INSERT INTO feed (id, title, url, link, last_updated)
                    VALUES (?, ?, ?, ?, ?);
                "#,
        )?;
        stmt.bind((1, feed.id.as_str()))?;
        stmt.bind((2, feed.title.as_str()))?;
        stmt.bind((3, feed.url.as_str()))?;
        stmt.bind((4, feed.link.as_str()))?;
        stmt.bind((5, feed.last_updated.to_rfc3339().as_str()))?;

        stmt.next()?;
        self.connection.execute("COMMIT")?;
        Ok(())
    }

    async fn get_feed_articles(&self, feed_id: Uuid) -> Result<Vec<Article>, RepositoryError> {
        self.connection
            .prepare("SELECT * FROM article WHERE feed_id = ?")?
            .into_iter()
            .bind((1, feed_id.to_string().as_str()))?
            .flat_map(|r| r.map(Article::try_from))
            .collect()
    }

    async fn get_article_description(
        &self,
        feed_id: Uuid,
        article_id: Uuid,
    ) -> Result<Option<Article>, RepositoryError> {
        self.connection
            .prepare("SELECT * FROM article WHERE id = ? AND feed_id = ?")?
            .into_iter()
            .bind((1, article_id.to_string().as_str()))?
            .bind((2, feed_id.to_string().as_str()))?
            .nth(0)
            .map(|r| {
                r.map_err(|e| RepositoryError::Unexpcted(e.into()))
                    .and_then(Article::try_from)
            })
            .transpose()
    }

    async fn add_articles(
        &self,
        feed_id: Uuid,
        articles: &[Article],
    ) -> Result<(), RepositoryError> {
        let now = Utc::now().to_rfc3339();
        let feed_id = feed_id.to_string();

        self.connection.execute("BEGIN")?;

        for article in articles {
            let mut stmt = self.connection.prepare(
                r#"
                    INSERT INTO article (id, feed_id, title, author, guid, link, last_updated, path, read)
                    VALUES (:id, :feed_id, :title, :author, :guid, :link, :last_updated, :path, 0)
                    ON CONFLICT(guid) DO NOTHING;
                )"#)?;
            stmt.bind((":id", Uuid::new_v4().to_string().as_str()))?;
            stmt.bind((":feed_id", feed_id.as_str()))?;
            stmt.bind((":title", article.title.as_str()))?;
            stmt.bind((":author", article.author.as_str()))?;
            stmt.bind((":guid", article.guid.as_str()))?;
            stmt.bind((":link", article.link.as_str()))?;
            stmt.bind((":last_updated", now.as_str()))?;
            stmt.bind((":path", article.path.as_deref().unwrap_or("NULL")))?;

            // Execute the statement
            stmt.next()?;
        }

        let mut stmt = self
            .connection
            .prepare("UPDATE feed SET last_updated = ? WHERE id = ?")?;
        stmt.bind((1, now.as_str()))?;
        stmt.bind((2, feed_id.as_str()))?;

        stmt.next()?;

        self.connection.execute("COMMIT")?;

        Ok(())
    }

    async fn get_article_content(
        &self,
        feed_id: Uuid,
        article_id: Uuid,
    ) -> Result<Option<String>, RepositoryError> {
        let file_path = self
            .connection
            .prepare("SELECT * FROM article WHERE id = ? AND feed_id = ?")?
            .into_iter()
            .bind((1, article_id.to_string().as_str()))?
            .bind((2, feed_id.to_string().as_str()))?
            .nth(0)
            .map(|r| {
                r.map_err(|e| RepositoryError::Unexpcted(e.into()))
                    .map(|row| row.read::<&str, _>("path").to_owned())
            })
            .transpose()?;

        if let Some(path) = file_path {
            Ok(Some(
                fs::read_to_string(path)
                    .await
                    .map_err(|e| RepositoryError::Unexpcted(e.into()))?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn update_last_updated(
        &self,
        feed_id: Uuid,
        date: DateTime<Utc>,
    ) -> Result<(), RepositoryError> {
        // TODO: Prepare statement
        self.connection.execute("BEGIN")?;
        let mut stmt = self
            .connection
            .prepare("UPDATE feed SET last_updated = ? WHERE id = ?")?;
        stmt.bind((1, date.to_rfc3339().as_str()))?;
        stmt.bind((2, feed_id.to_string().as_str()))?;

        // execute the statement
        stmt.next()?;
        self.connection.execute("COMMIT")?;

        Ok(())
    }
}
