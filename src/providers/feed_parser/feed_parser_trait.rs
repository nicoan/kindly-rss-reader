use crate::models::parsed_feed::ParsedFeed;

use super::Result;

/// Trait defining the common interface for all feed parsers
pub trait FeedParser: Send + Sync {
    /// Parse feed content and return feed metadata
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed>;
}
