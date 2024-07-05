use axum::async_trait;
use std::{fs, path::Path};

use minijinja::Environment;
use serde::Serialize;

use super::TemplateService;

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
    fn register_template(&mut self, name: &'a str, path: impl AsRef<Path>) {
        let template = fs::read_to_string(path.as_ref()).unwrap();
        self.environment.add_template_owned(name, template).unwrap();
    }

    fn render_template(&self, name: &str, context: impl Serialize) -> String {
        self.environment
            .get_template(name)
            .unwrap()
            .render(context)
            .unwrap()
    }
}
