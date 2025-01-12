use super::Result;
use axum::async_trait;
use minijinja::Environment;
use serde::Serialize;
use std::{fs, path::Path};

use super::{error::TemplateServiceError, TemplateService};

#[derive(Clone)]
pub struct TemplateServiceImpl<'a> {
    environment: Environment<'a>,
}

impl Default for TemplateServiceImpl<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateServiceImpl<'_> {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
}

#[async_trait]
impl<'a> TemplateService<'a> for TemplateServiceImpl<'a> {
    fn register_template(&mut self, name: &'a str, path: impl AsRef<Path>) -> Result<()> {
        let template = fs::read_to_string(path.as_ref()).map_err(|e| {
            TemplateServiceError::ReadingTemplate(path.as_ref().display().to_string(), e.into())
        })?;

        self.environment
            .add_template_owned(name, template)
            .map_err(|e| {
                TemplateServiceError::RegisteringTemplate(
                    name.to_owned(),
                    path.as_ref().display().to_string(),
                    e,
                )
            })?;

        Ok(())
    }

    fn render_template(&self, name: &str, context: impl Serialize) -> Result<String> {
        self.environment
            .get_template(name)
            .map_err(|e| TemplateServiceError::GettingTemplate(name.to_owned(), e))?
            .render(context)
            .map_err(|e| TemplateServiceError::RenderingTemplate(name.to_owned(), e))
    }
}
