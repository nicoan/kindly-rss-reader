use crate::controllers::{ApiError, HtmlResponse};
use crate::services::templates::{TemplateService, TEMPLATE_NAME_FEED_ADD};
use crate::state::AppState;
use axum::extract::State;
use minijinja::context;

pub async fn add_new_feed_form<S>(State(state): State<S>) -> Result<HtmlResponse, ApiError>
where
    S: AppState,
{
    Ok(HtmlResponse::new(
        state
            .template_service()
            .render_template(TEMPLATE_NAME_FEED_ADD, context! {})
            .await?,
    ))
}
