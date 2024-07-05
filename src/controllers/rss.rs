use crate::services::{templates::TEMPLATE_NAME_ARTICLE, RssService, TemplateService};
use crate::state::AppState;
use axum::{extract::State, response::Html};
use minijinja::context;

pub async fn get_article<S>(State(state): State<S>) -> Html<String>
where
    S: AppState,
{
    let channel = state
        .rss_service()
        .get_channel("https://nicoan.github.io/index.xml")
        .await
        .unwrap();

    let content = state
        .rss_service()
        .get_item_content(&channel.items()[0])
        .await
        .expect("content error");

    let rendered_article = state
        .template_service()
        .render_template(TEMPLATE_NAME_ARTICLE, context! { article => content });

    Html(rendered_article)
}
