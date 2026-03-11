use crate::{
    models::article::Article,
    repositories::{feed::Feed, RepositoryError},
    transaction,
};
use axum::async_trait;
use chrono::{DateTime, Utc};
use sqlite::ConnectionThreadSafe;
use std::sync::Arc;
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
            .prepare(
                "SELECT f.*, 
                (SELECT COUNT(*) FROM article a WHERE a.feed_id = f.id AND a.read = 0) as unread_count 
                FROM feed f;"
            )?
            .into_iter()
            .flat_map(|r| r.map(Feed::try_from))
            .collect()
    }

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>, RepositoryError> {
        self.connection
            .prepare(
                "SELECT f.*, 
                (SELECT COUNT(*) FROM article a WHERE a.feed_id = f.id AND a.read = 0) as unread_count 
                FROM feed f WHERE f.id = ?;"
            )?
            .into_iter()
            .bind((1, feed_id.to_string().as_str()))?
            .nth(0)
            .map(|r| {
                r.map_err(|e| RepositoryError::Unexpected(e.into()))
                    .and_then(Feed::try_from)
            })
            .transpose()
    }

    async fn add_feed(&self, feed: Feed) -> Result<(), RepositoryError> {
        // self.connection.execute("BEGIN")?;
        transaction!(self, {
            let mut stmt = self.connection.prepare(
                r#"
                    INSERT INTO feed (id, title, url, link, favicon_path, last_updated)
                    VALUES (?, ?, ?, ?, ?, ?);
                "#,
            )?;
            stmt.bind((1, feed.id.to_string().as_str()))?;
            stmt.bind((2, feed.title.as_str()))?;
            stmt.bind((3, feed.url.as_str()))?;
            stmt.bind((4, feed.link.as_str()))?;
            stmt.bind((5, feed.favicon_url.as_deref()))?;
            stmt.bind((6, feed.last_updated.to_rfc3339().as_str()))?;

            // Execute the statement
            stmt.next()?;
            stmt.reset()?;
            drop(stmt);

            Ok(())
        })
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
                r.map_err(|e| RepositoryError::Unexpected(e.into()))
                    .and_then(Article::try_from)
            })
            .transpose()
    }

    async fn add_articles(
        &self,
        feed_id: Uuid,
        articles: &[&Article],
    ) -> Result<(), RepositoryError> {
        let feed_id = feed_id.to_string();
        transaction!(self, {
            for article in articles {
                let mut stmt = self.connection.prepare(
                r#"
                    INSERT INTO article (id, feed_id, title, author, guid, link, last_updated, html_parsed, content, read)
                    VALUES (:id, :feed_id, :title, :author, :guid, :link, :last_updated, :html_parsed, :content, 0)
                    ON CONFLICT(guid) DO NOTHING;
                )"#)?;
                stmt.bind((":id", article.id.to_string().as_str()))?;
                stmt.bind((":feed_id", feed_id.as_str()))?;
                stmt.bind((":title", article.title.as_str()))?;
                stmt.bind((":author", article.author.as_deref()))?;
                stmt.bind((":guid", article.guid.as_str()))?;
                stmt.bind((":link", article.link.as_str()))?;
                stmt.bind((":last_updated", article.last_updated.to_rfc3339().as_str()))?;
                stmt.bind((":html_parsed", if article.html_parsed { 1 } else { 0 }))?;
                stmt.bind((":content", article.content.as_deref()))?;

                // Execute the statement
                stmt.next()?;
                stmt.reset()?;
                drop(stmt);
            }

            let mut stmt = self
                .connection
                .prepare("UPDATE feed SET last_updated = ? WHERE id = ?")?;
            stmt.bind((1, Utc::now().to_rfc3339().as_str()))?;
            stmt.bind((2, feed_id.as_str()))?;

            stmt.next()?;
            stmt.reset()?;
            drop(stmt);

            Ok(())
        })
    }

    async fn update_last_updated(
        &self,
        feed_id: Uuid,
        date: DateTime<Utc>,
    ) -> Result<(), RepositoryError> {
        transaction!(self, {
            let mut stmt = self
                .connection
                .prepare("UPDATE feed SET last_updated = ? WHERE id = ?")?;
            stmt.bind((1, date.to_rfc3339().as_str()))?;
            stmt.bind((2, feed_id.to_string().as_str()))?;

            // execute the statement
            stmt.next()?;
            stmt.reset()?;
            drop(stmt);

            Ok(())
        })
    }

    async fn mark_article_as_read(
        &self,
        feed_id: Uuid,
        article_id: Uuid,
    ) -> Result<(), RepositoryError> {
        transaction!(self, {
            let mut stmt = self
                .connection
                .prepare("UPDATE article SET read = 1 WHERE feed_id = ? and id = ?")?;
            stmt.bind((1, feed_id.to_string().as_str()))?;
            stmt.bind((2, article_id.to_string().as_str()))?;

            // execute the statement
            stmt.next()?;
            stmt.reset()?;
            drop(stmt);

            Ok(())
        })
    }
    
    async fn delete_feed(&self, feed_id: Uuid) -> Result<(), RepositoryError> {
        transaction!(self, {
            // First delete all articles related to this feed
            let mut stmt = self
                .connection
                .prepare("DELETE FROM article WHERE feed_id = ?")?;
            stmt.bind((1, feed_id.to_string().as_str()))?;
            stmt.next()?;
            stmt.reset()?;
            drop(stmt);
            
            // Then delete the feed itself
            let mut stmt = self
                .connection
                .prepare("DELETE FROM feed WHERE id = ?")?;
            stmt.bind((1, feed_id.to_string().as_str()))?;
            stmt.next()?;
            stmt.reset()?;
            drop(stmt);
            
            Ok(())
        })
    }
}
