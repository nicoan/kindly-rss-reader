use std::sync::Arc;

use axum::async_trait;

use crate::{
    models::persisted_config::PersistedConfig,
    providers::persisted_config::PersistedConfigProvider,
    repositories::persisted_config::PersistedConfigRepository,
};

use super::{PersistedConfigService, Result};

pub struct PersistedConfigServiceImpl<PCR, PCP>
where
    PCR: PersistedConfigRepository,
    PCP: PersistedConfigProvider,
{
    persisted_config_repository: Arc<PCR>,
    persisted_config_provider: Arc<PCP>,
}

impl<PCR, PCP> PersistedConfigServiceImpl<PCR, PCP>
where
    PCR: PersistedConfigRepository,
    PCP: PersistedConfigProvider,
{
    pub fn new(persisted_config_repository: Arc<PCR>, persisted_config_provider: Arc<PCP>) -> Self {
        Self {
            persisted_config_repository,
            persisted_config_provider,
        }
    }
}

#[async_trait]
impl<PCR, PCP> PersistedConfigService for PersistedConfigServiceImpl<PCR, PCP>
where
    PCR: PersistedConfigRepository,
    PCP: PersistedConfigProvider,
{
    async fn get_configuration(&self) -> Result<PersistedConfig> {
        Ok(self.persisted_config_provider.get_configuration().await?)
    }

    async fn set_zoom(&self, value: f64) -> Result<()> {
        let config = self.persisted_config_provider.set_zoom(value).await?;
        Ok(self
            .persisted_config_repository
            .save_configuration(config)
            .await?)
    }

    async fn set_dark_theme(&self, value: bool) -> Result<()> {
        let config = self.persisted_config_provider.set_dark_theme(value).await?;
        Ok(self
            .persisted_config_repository
            .save_configuration(config)
            .await?)
    }
}
