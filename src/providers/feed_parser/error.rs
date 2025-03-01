use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedParserError {
    #[error("Error parsing RSS feed: {0}")]
    RssParsingError(#[from] rss::Error),
    
    #[error("Error parsing Atom feed: {0}")]
    AtomParsingError(#[from] atom_syndication::Error),
    
    #[error("Error parsing date: {0}")]
    DateParsingError(#[from] chrono::ParseError),
    
    #[error("Invalid feed format")]
    InvalidFeedFormat,
    
    #[error("Feed item missing required fields: {0}")]
    MissingFields(String),
    
    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}