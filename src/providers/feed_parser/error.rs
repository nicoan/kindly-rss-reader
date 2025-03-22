use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedParserError {
    #[error("Failed to parse RSS feed: {0}")]
    RssParseError(#[from] rss::Error),

    #[error("Failed to parse Atom feed: {0}")]
    AtomParseError(#[from] atom_syndication::Error),

    #[error("Unsupported feed format")]
    UnsupportedFormat,

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Failed to parse date: {0}")]
    DateParseError(#[from] chrono::ParseError),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, FeedParserError>;
