mod error;
mod feed_parser_impl;

pub use error::FeedParserError;
pub use feed_parser_impl::FeedParserImpl;

use crate::models::feed::Feed;
use anyhow::Result;
use axum::async_trait;
use axum::body::Bytes;

#[async_trait]
pub trait FeedParser: Send + Sync {
    /// Parse feed content (RSS or Atom) and create a Feed model
    async fn parse_feed(&self, feed_url: &str, content: &Bytes) -> Result<Feed, FeedParserError>;
    
    /// Parse feed items (RSS items or Atom entries) and create Article models
    async fn parse_feed_items(
        &self, 
        content: &Bytes
    ) -> Result<Vec<(String, Option<String>)>, FeedParserError>;
    
    /// Get the article's GUID or ID for tracking
    fn get_item_identifier(&self, item: &(String, Option<String>)) -> String;
}