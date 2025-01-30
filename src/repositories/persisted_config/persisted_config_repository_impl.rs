use axum::async_trait;
use tokio::fs;

use crate::{models::persisted_config::PersistedConfig, repositories::RepositoryError};

use super::{PersistedConfigRepository, Result};

pub struct PersistedConfigFsRepositoryImpl {
    config_path: String,
}

impl PersistedConfigFsRepositoryImpl {
    pub fn new(data_path: String) -> Self {
        Self {
            config_path: format!("{data_path}/config.json"),
        }
    }
}

#[async_trait]
impl PersistedConfigRepository for PersistedConfigFsRepositoryImpl {
    async fn load_configuration(&self) -> PersistedConfig {
        fn return_default_config(message: &str, error: impl std::error::Error) -> PersistedConfig {
            tracing::error!("{message}. error: {error}");
            PersistedConfig::default()
        }

        fs::read_to_string(&self.config_path)
            .await
            .map(|c| {
                serde_json::from_str::<PersistedConfig>(&c).unwrap_or_else(|e| {
                    return_default_config(
                        "unable to deserialize configuration file, using default configuration",
                        e,
                    )
                })
            })
            .unwrap_or_else(|e| {
                return_default_config(
                    "unable to load configuration, using default configuration",
                    e,
                )
            })
    }

    async fn save_configuration(&self, config: PersistedConfig) -> Result<()> {
        let str_config =
            serde_json::to_string(&config).map_err(|e| RepositoryError::Unexpected(e.into()))?;
        fs::write(&self.config_path, &str_config)
            .await
            .map_err(|e| RepositoryError::Unexpected(e.into()))?;

        Ok(())
    }
}
