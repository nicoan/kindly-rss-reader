use crate::services::feed::FeedService;
use crate::services::templates::TemplateService;
use crate::services::templates::TEMPLATE_NAME_ARTICLE;
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};
use minijinja::context;
use uuid::Uuid;

use super::ApiError;

pub async fn get_article<S>(
    State(state): State<S>,
    Path((feed_id, article_id)): Path<(Uuid, Uuid)>,
) -> Result<Html<String>, ApiError>
where
    S: AppState,
{
    let feed = state.feed_service().get_feed(feed_id).await?;

    let (article_data, content) = state
        .feed_service()
        .get_item_content(feed_id, article_id)
        .await?;

    let rendered_article = state.template_service().render_template(
        TEMPLATE_NAME_ARTICLE,
        context! { feed => feed, article => content, article_data => article_data },
    )?;

    Ok(Html(rendered_article))
}
