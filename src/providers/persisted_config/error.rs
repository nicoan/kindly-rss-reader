use crate::repositories::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum PersistedConfigProviderError {
    #[error("a repository error ocurred: {0:?}")]
    Repository(#[from] RepositoryError),
}
