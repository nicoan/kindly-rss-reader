use crate::controllers::ApiError;
use crate::services::templates::{TemplateService, TEMPLATE_NAME_ERROR};
use crate::state::AppState;
use crate::view_models::error::Error;
use axum::extract::State;
use minijinja::context;
use reqwest::StatusCode;

use super::HtmlResponse;

pub async fn not_found<S>(State(state): State<S>) -> Result<HtmlResponse, ApiError>
where
    S: AppState,
{
    let error = Error::not_found();
    let rendered_html = state
        .template_service()
        .render_template(TEMPLATE_NAME_ERROR, context! { error => error})
        .await?;

    Ok(HtmlResponse::new(rendered_html).with_status_code(StatusCode::NOT_FOUND))
}
