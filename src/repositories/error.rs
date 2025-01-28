#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("there was an error deserializating data retrieved: {0:?}")]
    Deserialization(#[source] anyhow::Error),

    #[error("unexpected error: {0:?}")]
    Unexpected(#[from] anyhow::Error),
}

impl From<sqlite::Error> for RepositoryError {
    fn from(value: sqlite::Error) -> Self {
        RepositoryError::Unexpected(value.into())
    }
}
