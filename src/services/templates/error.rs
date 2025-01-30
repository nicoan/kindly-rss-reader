#[derive(Debug, thiserror::Error)]
pub enum TemplateServiceError {
    #[error("there was an error reading the template {0}: {1:?}")]
    Reading(String, #[source] anyhow::Error),

    #[error("there was an error registering the template {0} with path {1}: {2:?}")]
    Registering(String, String, #[source] minijinja::Error),

    #[error("there was an error getting the registered template {0}: {1:?}")]
    Getting(String, #[source] minijinja::Error),

    #[error("there was an error rendering the registered template {0}: {1:?}")]
    Rendering(String, #[source] minijinja::Error),
}
