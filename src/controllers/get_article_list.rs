use crate::services::feed::FeedService;
use crate::services::templates::{TemplateService, TEMPLATE_NAME_ARTICLE_LIST};
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};
use minijinja::context;
use uuid::Uuid;

use super::ApiError;

pub async fn get_article_list<S>(
    State(state): State<S>,
    Path(feed_id): Path<Uuid>,
) -> Result<Html<String>, ApiError>
where
    S: AppState,
{
    let (feed, articles) = state.feed_service().get_channel(feed_id).await?;

    let rendered_html = state.template_service().render_template(
        TEMPLATE_NAME_ARTICLE_LIST,
        context! { feed => feed, articles => articles },
    )?;

    Ok(Html(rendered_html))
}
