//! This provider acts as a memory cache for the persisted config
mod error;
mod persisted_config_fs_impl;

use crate::models::persisted_config::PersistedConfig;
use axum::async_trait;
pub use error::PersistedConfigProviderError;
pub use persisted_config_fs_impl::PersistedConfigProviderImpl;

pub(crate) type Result<T> = std::result::Result<T, PersistedConfigProviderError>;

#[async_trait]
pub trait PersistedConfigProvider: Sync + Send {
    async fn get_configuration(&self) -> Result<PersistedConfig>;

    async fn set_zoom(&self, value: f64) -> Result<PersistedConfig>;

    async fn set_dark_theme(&self, value: bool) -> Result<PersistedConfig>;
}
