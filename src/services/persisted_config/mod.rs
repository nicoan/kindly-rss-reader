mod error;
mod persisted_config_service_impl;

use axum::async_trait;

use error::PersistedConfigError;

use crate::models::persisted_config::PersistedConfig;

pub use persisted_config_service_impl::PersistedConfigServiceImpl;

pub(crate) type Result<T> = std::result::Result<T, PersistedConfigError>;

#[async_trait]
pub trait PersistedConfigService: Sync + Send {
    async fn get_configuration(&self) -> Result<PersistedConfig>;

    async fn set_zoom(&self, value: f64) -> Result<()>;

    async fn set_dark_theme(&self, value: bool) -> Result<()>;
}
