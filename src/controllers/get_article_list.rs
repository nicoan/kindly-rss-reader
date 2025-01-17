use crate::services::feed::FeedService;
use crate::services::templates::{TemplateService, TEMPLATE_NAME_ARTICLE_LIST};
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, response::Html};
use minijinja::context;
use uuid::Uuid;

pub async fn get_article_list<S>(State(state): State<S>, Path(feed_id): Path<Uuid>) -> Html<String>
where
    S: AppState,
{
    let article_list = state.feed_service().get_channel(feed_id).await;

    let rendered_html = if let Ok((feed, articles)) = article_list {
        state
            .template_service()
            .render_template(
                TEMPLATE_NAME_ARTICLE_LIST,
                context! { feed => feed, articles => articles },
            )
            .unwrap_or(
                "<h1> There was an error rendering the article list. Please check the logs. </h1>"
                    .to_owned(),
            )
    } else {
        "<h1> There was an error getting the article list. Please check the logs. </h1>".to_owned()
    };

    Html(rendered_html)
}
