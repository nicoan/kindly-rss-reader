use crate::{
    providers::persisted_config::PersistedConfigProviderError, repositories::RepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum PersistedConfigError {
    #[error("a repository error ocurred: {0:?}")]
    Repository(#[from] RepositoryError),

    #[error("a provider error ocurred: {0:?}")]
    Provider(#[from] PersistedConfigProviderError),
}
