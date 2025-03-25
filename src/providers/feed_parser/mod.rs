mod atom_parser_impl;
mod error;
mod feed_parser_impl;
mod feed_parser_trait;
mod rss_parser_impl;

pub use atom_parser_impl::AtomParserImpl;
pub use error::FeedParserError;
pub use feed_parser_impl::FeedParserImpl;
pub use feed_parser_trait::{FeedParser, ParsedFeed, ParsedItem};
pub use rss_parser_impl::RssParserImpl;

pub(crate) type Result<T> = std::result::Result<T, FeedParserError>;
