use crate::controllers::ApiError;
use crate::services::templates::{TemplateService, TEMPLATE_NAME_NOT_FOUND};
use crate::state::AppState;
use axum::extract::State;
use minijinja::context;

use super::HtmlResponse;

pub async fn not_found<S>(State(state): State<S>) -> Result<HtmlResponse, ApiError>
where
    S: AppState,
{
    let rendered_html = state
        .template_service()
        .render_template(TEMPLATE_NAME_NOT_FOUND, context! {})
        .await?;

    Ok(HtmlResponse::new(rendered_html))
}
