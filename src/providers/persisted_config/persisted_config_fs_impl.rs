// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use axum::async_trait;
use tokio::sync::RwLock;

use crate::models::persisted_config::PersistedConfig;

use super::{PersistedConfigProvider, Result};

pub struct PersistedConfigProviderImpl {
    loaded_configuration: RwLock<PersistedConfig>,
}

impl PersistedConfigProviderImpl {
    pub fn new(config: PersistedConfig) -> Self {
        Self {
            loaded_configuration: RwLock::new(config),
        }
    }
}

#[async_trait]
impl PersistedConfigProvider for PersistedConfigProviderImpl {
    async fn get_configuration(&self) -> Result<PersistedConfig> {
        Ok((*self.loaded_configuration.read().await).clone())
    }

    async fn set_zoom(&self, value: f64) -> Result<PersistedConfig> {
        let mut config = self.loaded_configuration.write().await;
        config.zoom = value;
        Ok(config.clone())
    }

    async fn set_dark_theme(&self, value: bool) -> Result<PersistedConfig> {
        let mut config = self.loaded_configuration.write().await;
        config.dark_theme = value;
        Ok(config.clone())
    }

    async fn set_toolbar_position_left(&self, value: bool) -> Result<PersistedConfig> {
        let mut config = self.loaded_configuration.write().await;
        config.toolbar_position_left = value;
        Ok(config.clone())
    }

    async fn set_hide_article_header(&self, value: bool) -> Result<PersistedConfig> {
        let mut config = self.loaded_configuration.write().await;
        config.hide_article_header = value;
        Ok(config.clone())
    }

    async fn set_dont_invert_images(&self, value: bool) -> Result<PersistedConfig> {
        let mut config = self.loaded_configuration.write().await;
        config.dont_invert_images = value;
        Ok(config.clone())
    }
}
