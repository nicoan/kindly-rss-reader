use chrono::{DateTime, Utc};

use super::error::Result;

/// Common structure representing parsed feed data
#[derive(Debug, Clone)]
pub struct ParsedFeed {
    pub title: String,
    pub link: String,
    pub items: Vec<ParsedItem>,
}

/// Common structure representing a parsed feed item
#[derive(Debug, Clone)]
pub struct ParsedItem {
    pub title: String,
    pub link: Option<String>,
    pub guid: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
}

/// Trait defining the common interface for all feed parsers
pub trait FeedParser: Send + Sync {
    /// Parse feed content and return feed metadata
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed>;
}
