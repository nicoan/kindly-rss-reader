pub mod persisted_config_repository_impl;

use axum::async_trait;

use crate::models::persisted_config::PersistedConfig;

use super::RepositoryError;

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[async_trait]
pub trait PersistedConfigRepository: Sync + Send {
    async fn load_configuration(&self) -> PersistedConfig;

    async fn save_configuration(&self, config: PersistedConfig) -> Result<()>;
}
