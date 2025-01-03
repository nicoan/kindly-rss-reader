use std::sync::Arc;

use super::FeedService;
use crate::{models::feed::Feed, repositories::feed::FeedRepository};
use axum::async_trait;

#[derive(Clone)]
pub struct FeedServiceImpl<FR: FeedRepository> {
    feed_repository: Arc<FR>,
}

impl<FR: FeedRepository> FeedServiceImpl<FR> {
    pub fn new(feed_repository: Arc<FR>) -> Self {
        Self { feed_repository }
    }
}

#[async_trait]
impl<FR: FeedRepository> FeedService for FeedServiceImpl<FR> {
    async fn get_feed_list(&self) -> Vec<Feed> {
        self.feed_repository.get_feed_list().await
    }
}
