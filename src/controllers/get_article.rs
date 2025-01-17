use crate::services::feed::FeedService;
use crate::services::templates::TemplateService;
use crate::services::templates::TEMPLATE_NAME_ARTICLE;
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};
use minijinja::context;
use uuid::Uuid;

pub async fn get_article<S>(
    State(state): State<S>,
    Path((feed_id, article_id)): Path<(Uuid, Uuid)>,
) -> Result<Html<String>, Html<String>>
where
    S: AppState,
{
    let feed = state.feed_service().get_feed(feed_id).await.map_err(|e| {
        tracing::error!("{:?}", e);
        Html("<h1> Feed not found </h1>".to_owned())
    })?;

    let content = state
        .feed_service()
        .get_item_content(feed_id, article_id)
        .await
        .unwrap_or("<h1> Article not found </h1>".to_owned());

    let rendered_article = state
        .template_service()
        .render_template(
            TEMPLATE_NAME_ARTICLE,
            context! { feed => feed, article => content },
        )
        .unwrap_or(
            "<h1> There was an error rendering the article. Please check the logs. </h1>"
                .to_owned(),
        );

    Ok(Html(rendered_article))
}
