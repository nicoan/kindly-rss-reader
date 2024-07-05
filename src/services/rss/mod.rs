mod error;
mod rss_service_impl;

use axum::async_trait;
use rss::{Channel, Item};
use std::error::Error;

pub use rss_service_impl::RssServiceImpl;

#[async_trait]
pub(crate) trait RssService: Sync + Send {
    async fn get_channel(&self, url: impl AsRef<str> + Send) -> Result<Channel, Box<dyn Error>>;

    async fn get_item_content(&self, item: &Item) -> Result<String, String>;
}
