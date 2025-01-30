mod error;
mod feed_service_impl;

use axum::async_trait;

use error::FeedServiceError;
pub use feed_service_impl::FeedServiceImpl;
use reqwest::Url;
use uuid::Uuid;

use crate::models::{article::Article, feed::Feed};

pub(crate) type Result<T> = std::result::Result<T, FeedServiceError>;

#[async_trait]
pub(crate) trait FeedService: Sync + Send {
    async fn get_feed_list(&self) -> Result<Vec<Feed>>;

    async fn get_feed(&self, feed_id: Uuid) -> Result<Option<Feed>>;

    async fn add_feed(&self, feed_url: Url) -> Result<()>;

    async fn get_channel(&self, feed_id: Uuid) -> Result<(Feed, Vec<Article>)>;

    async fn get_item_content(&self, feed_id: Uuid, article_id: Uuid) -> Result<(Article, String)>;

    async fn mark_article_as_read(&self, feed_id: Uuid, article_id: Uuid) -> Result<()>;
}
