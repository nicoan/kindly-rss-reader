use super::Result;
use crate::providers::persisted_config::PersistedConfigProvider;
use axum::async_trait;
use minijinja::context;
use minijinja::Environment;
use serde::Serialize;
use std::{fs, path::Path, sync::Arc};

use super::{error::TemplateServiceError, TemplateService};

#[derive(Clone)]
pub struct TemplateServiceImpl<'a, PCP: PersistedConfigProvider> {
    environment: Environment<'a>,
    persisted_config_provider: Arc<PCP>,
}

impl<PCP> TemplateServiceImpl<'_, PCP>
where
    PCP: PersistedConfigProvider,
{
    pub fn new(persisted_config_provider: Arc<PCP>) -> Self {
        Self {
            environment: Environment::new(),
            persisted_config_provider,
        }
    }
}

#[async_trait]
impl<'a, PCP> TemplateService<'a> for TemplateServiceImpl<'a, PCP>
where
    PCP: PersistedConfigProvider,
{
    fn register_template(&mut self, name: &'a str, path: impl AsRef<Path>) -> Result<()> {
        let template = fs::read_to_string(path.as_ref()).map_err(|e| {
            TemplateServiceError::Reading(path.as_ref().display().to_string(), e.into())
        })?;

        self.environment
            .add_template_owned(name, template)
            .map_err(|e| {
                TemplateServiceError::Registering(
                    name.to_owned(),
                    path.as_ref().display().to_string(),
                    e,
                )
            })?;

        Ok(())
    }

    async fn render_template(
        &self,
        name: &str,
        context: impl Serialize + Sync + Send,
    ) -> Result<String> {
        let config = self
            .persisted_config_provider
            .get_configuration()
            .await
            .unwrap();

        self.environment
            .get_template(name)
            .map_err(|e| TemplateServiceError::Getting(name.to_owned(), e))?
            .render(context! { version => env!("CARGO_PKG_VERSION"), config => config, context })
            .map_err(|e| TemplateServiceError::Rendering(name.to_owned(), e))
    }
}
