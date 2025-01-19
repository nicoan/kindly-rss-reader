use crate::services::templates::TEMPLATE_NAME_FEED_LIST;
use crate::services::{feed::FeedService, templates::TemplateService};
use crate::state::AppState;
use axum::{extract::State, response::Html};
use minijinja::context;

use super::ApiError;

pub async fn get_feed_list<S>(State(state): State<S>) -> Result<Html<String>, ApiError>
where
    S: AppState,
{
    let feeds = state.feed_service().get_feed_list().await?;

    let rendered_html = state
        .template_service()
        .render_template(TEMPLATE_NAME_FEED_LIST, context! { feeds => feeds })?;

    Ok(Html(rendered_html))
}
