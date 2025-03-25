use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedParserError {
    #[error("Failed to parse feed: {0}")]
    ParseError(#[source] anyhow::Error),

    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("Failed to parse date: {0}")]
    DateParseError(#[from] chrono::ParseError),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}
