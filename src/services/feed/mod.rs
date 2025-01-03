mod error;
mod feed_service_impl;

use axum::async_trait;

pub use feed_service_impl::FeedServiceImpl;

use crate::models::feed::Feed;

#[async_trait]
pub(crate) trait FeedService: Sync + Send {
    async fn get_feed_list(&self) -> Vec<Feed>;
}
