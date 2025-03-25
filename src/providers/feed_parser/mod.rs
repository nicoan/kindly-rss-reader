mod atom_parser_impl;
mod error;
mod rss_parser_impl;

pub use atom_parser_impl::AtomParserImpl;
pub use error::FeedParserError;
pub use rss_parser_impl::RssParserImpl;

use crate::models::parsed_feed::ParsedFeed;

pub(crate) type Result<T> = std::result::Result<T, FeedParserError>;

/// Trait defining the common interface for all feed parsers
pub trait FeedParser: Send + Sync {
    /// Parse feed content and return feed metadata
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed>;
}
