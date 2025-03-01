mod feed_repository_impl;

use crate::models::{article::Article, feed::Feed};
use axum::async_trait;

use chrono::{DateTime, Utc};
pub use feed_repository_impl::FeedRepositoryImpl;
use uuid::Uuid;

use super::RepositoryError;

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[async_trait]
pub trait FeedRepository: Sync + Send {
    async fn add_feed(&self, url: Feed) -> Result<()>;

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>>;

    async fn get_feed_articles(&self, feed_id: Uuid) -> Result<Vec<Article>>;

    async fn get_feed_list(&self) -> Result<Vec<Feed>>;

    async fn add_articles(&self, feed_id: Uuid, articles: &[&Article]) -> Result<()>;

    async fn get_article_description(
        &self,
        feed_id: Uuid,
        article_id: Uuid,
    ) -> Result<Option<Article>>;

    async fn update_last_updated(&self, feed_id: Uuid, date: DateTime<Utc>) -> Result<()>;

    async fn mark_article_as_read(&self, feed_id: Uuid, article_id: Uuid) -> Result<()>;
    
    async fn delete_feed(&self, feed_id: Uuid) -> Result<()>;
}
