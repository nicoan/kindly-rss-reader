use std::{error::Error, sync::Arc};

use super::FeedService;
use crate::{models::feed::Feed, repositories::feed::FeedRepository};
use axum::async_trait;
use chrono::DateTime;
use reqwest::Url;
use uuid::Uuid;

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

    async fn add_feed(&self, feed_url: Url) -> Result<(), Box<dyn Error>> {
        let content = reqwest::get(feed_url.as_str()).await?.bytes().await?;
        let rss_channel = rss::Channel::read_from(&content[..])?;

        let feed = Feed {
            id: Uuid::new_v4().to_string(),
            title: rss_channel.title,
            link: feed_url.origin().unicode_serialization(),
            url: feed_url.into(),
            favicon_url: None,
            last_updated: DateTime::default(),
        };

        self.feed_repository.add_feed(feed).await;

        Ok(())
    }
}
