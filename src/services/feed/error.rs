use reqwest::StatusCode;
use uuid::Uuid;

use crate::{controllers::ApiError, repositories::RepositoryError};

#[derive(Debug, thiserror::Error)]
pub enum FeedServiceError {
    #[error("the content of article {0} of feed {1} was not found")]
    ArticleContentNotFound(Uuid, Uuid),

    #[error("there was an error getting the article: {0}")]
    GettingArticle(#[source] reqwest::Error),

    #[error("there was an error getting the feed: {0}")]
    GettingFeed(#[source] reqwest::Error),

    #[error("the feed {0} was not found")]
    FeedNotFound(Uuid),

    #[error("a repository error ocurred: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Unsupported feed format")]
    UnsupportedFormat,

    #[error("unexpected error ocurred: {0}")]
    Unexpected(#[source] anyhow::Error),
}

impl From<FeedServiceError> for ApiError {
    fn from(error: FeedServiceError) -> Self {
        match error {
            e @ FeedServiceError::ArticleContentNotFound(_, _) => Self {
                original_error: e.into(),
                status_code: StatusCode::NOT_FOUND,
            },

            e @ FeedServiceError::FeedNotFound(_) => Self {
                original_error: e.into(),
                status_code: StatusCode::NOT_FOUND,
            },

            e => Self {
                original_error: e.into(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}
