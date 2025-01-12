use uuid::Uuid;

use crate::repositories::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum FeedServiceError {
    #[error("the content of article {0} of feed {1} was not found")]
    ArticleContentNotFound(Uuid, Uuid),

    #[error("the article {0} of feed {1} was not found")]
    ArticleNotFound(Uuid, Uuid),

    #[error("there was an error getting the article: {0:?}")]
    GettingArticle(#[source] reqwest::Error),

    #[error("there was an error getting the feed: {0:?}")]
    GettingFeed(#[source] reqwest::Error),

    #[error("the feed {0} was not found")]
    FeedNotFound(Uuid),

    #[error("there was an error parsing the date for {0}")]
    ParsingDate(String, #[source] chrono::ParseError),

    #[error("a repository error ocurred: {0:?}")]
    Repository(#[from] RepositoryError),

    #[error("unexpected error ocurred: {0:?}")]
    Unexpected(#[source] anyhow::Error),
}
