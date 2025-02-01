use reqwest::StatusCode;

use crate::{
    controllers::ApiError, providers::persisted_config::PersistedConfigProviderError,
    repositories::RepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum PersistedConfigError {
    #[error("a repository error ocurred: {0:?}")]
    Repository(#[from] RepositoryError),

    #[error("a provider error ocurred: {0:?}")]
    Provider(#[from] PersistedConfigProviderError),
}

impl From<PersistedConfigError> for ApiError {
    fn from(error: PersistedConfigError) -> Self {
        Self {
            original_error: error.into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
