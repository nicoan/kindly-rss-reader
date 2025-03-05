//! This repository manages the saving and retrieving of the feed articles
mod feed_content_fs_impl;

use axum::async_trait;

pub use feed_content_fs_impl::FeedContentFsRepositoryImpl;
use uuid::Uuid;

use crate::models::article::Article;

use super::RepositoryError;

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[async_trait]
pub trait FeedContentRepository: Sync + Send {
    async fn get_article_content(&self, feed_id: Uuid, article_id: Uuid) -> Result<Option<String>>;

    async fn save_article_content(&self, articles: &[(&Article, &String)]) -> Result<()>;
    
    async fn delete_feed_content(&self, feed_id: Uuid) -> Result<()>;
}
