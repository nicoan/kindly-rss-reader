use crate::services::feed::FeedService;
use crate::services::{templates::TEMPLATE_NAME_ARTICLE, TemplateService};
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};
use minijinja::context;
use uuid::Uuid;

pub async fn get_article<S>(
    State(state): State<S>,
    Path((feed_id, article_id)): Path<(Uuid, Uuid)>,
) -> Html<String>
where
    S: AppState,
{
    let content = state
        .feed_service()
        .get_item_content(feed_id, article_id)
        .await
        .unwrap_or("<h1> Article not found </h1>".to_owned());

    let rendered_article = state
        .template_service()
        .render_template(TEMPLATE_NAME_ARTICLE, context! { article => content })
        .unwrap_or(
            "<h1> There was an error rendering the article. Please check the logs. </h1>"
                .to_owned(),
        );

    Html(rendered_article)
}
