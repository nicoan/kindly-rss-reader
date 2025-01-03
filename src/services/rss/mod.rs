mod error;
mod rss_service_impl;

use crate::models::{article::Article, feed::Feed};
use axum::async_trait;
use std::error::Error;
use uuid::Uuid;

pub use rss_service_impl::RssServiceImpl;

#[async_trait]
pub(crate) trait RssService: Sync + Send {
    async fn get_channel(&self, feed_id: Uuid) -> Result<(Feed, Vec<Article>), Box<dyn Error>>;

    async fn get_item_content(
        &self,
        feed_id: Uuid,
        article_id: Uuid,
    ) -> Result<String, Box<dyn Error>>;
}
