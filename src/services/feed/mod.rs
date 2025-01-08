mod error;
mod feed_service_impl;

use std::error::Error;

use axum::async_trait;

pub use feed_service_impl::FeedServiceImpl;
use reqwest::Url;

use crate::models::feed::Feed;

#[async_trait]
pub(crate) trait FeedService: Sync + Send {
    async fn get_feed_list(&self) -> Vec<Feed>;

    async fn add_feed(&self, feed_url: Url) -> Result<(), Box<dyn Error>>;
}
