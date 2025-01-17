use crate::services::templates::TEMPLATE_NAME_FEED_LIST;
use crate::services::{feed::FeedService, templates::TemplateService};
use crate::state::AppState;
use axum::{extract::State, response::Html};
use minijinja::context;

pub async fn get_feed_list<S>(State(state): State<S>) -> Html<String>
where
    S: AppState,
{
    let feeds = state.feed_service().get_feed_list().await;

    let rendered_html = if let Ok(feeds) = feeds {
        state
            .template_service()
            .render_template(TEMPLATE_NAME_FEED_LIST, context! { feeds => feeds })
            .unwrap_or(
                "<h1> There was an error rendering the feed list. Please check the logs. </h1>"
                    .to_owned(),
            )
    } else {
        "<h1> There was an error getting the feed list. Please check the logs. </h1>".to_owned()
    };

    Html(rendered_html)
}
