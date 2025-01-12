#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("there was an error deserializating data retrieved: {0:?}")]
    Deserialization(#[source] anyhow::Error),

    #[error("unexpected error: {0:?}")]
    Unexpcted(#[from] anyhow::Error),
}
