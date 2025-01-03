mod feed_repository_impl;

use crate::models::{article::Article, feed::Feed};
use axum::async_trait;

pub use feed_repository_impl::FeedRepositoryImpl;
use uuid::Uuid;

#[async_trait]
pub trait FeedRepository: Sync + Send {
    async fn get_feed_list(&self) -> Vec<Feed>;

    async fn get_feed(&self, feed_id: Uuid) -> Option<Feed>;

    async fn get_feed_articles(&self, feed_id: Uuid) -> Vec<Article>;

    async fn add_articles(&self, feed_id: Uuid, articles: Vec<Article>);

    async fn get_article_description(&self, feed_id: Uuid, article_id: Uuid) -> Option<Article>;

    async fn get_article_content(&self, feed_id: Uuid, article_id: Uuid) -> Option<String>;
}
