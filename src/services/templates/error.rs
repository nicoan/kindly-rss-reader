#[derive(Debug, thiserror::Error)]
pub enum TemplateServiceError {
    #[error("there was an error reading the template {0}: {1:?}")]
    ReadingTemplate(String, #[source] anyhow::Error),

    #[error("there was an error registering the template {0} with path {1}: {2:?}")]
    RegisteringTemplate(String, String, #[source] minijinja::Error),

    #[error("there was an error getting the registered template {0}: {1:?}")]
    GettingTemplate(String, #[source] minijinja::Error),

    #[error("there was an error rendering the registered template {0}: {1:?}")]
    RenderingTemplate(String, #[source] minijinja::Error),
}
