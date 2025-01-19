use crate::services::templates::{TemplateService, TEMPLATE_NAME_FEED_ADD};
use crate::state::AppState;
use axum::{extract::State, response::Html};
use minijinja::context;

use super::ApiError;

pub async fn add_new_feed_form<S>(State(state): State<S>) -> Result<Html<String>, ApiError>
where
    S: AppState,
{
    Ok(Html(
        state
            .template_service()
            .render_template(TEMPLATE_NAME_FEED_ADD, context! {})?,
    ))
}
