use crate::services::feed::FeedService;
use crate::services::templates::{
    TEMPLATE_NAME_ARTICLE_LIST, TEMPLATE_NAME_FEED_ADD, TEMPLATE_NAME_FEED_LIST,
};
use crate::services::{templates::TEMPLATE_NAME_ARTICLE, RssService, TemplateService};
use crate::state::AppState;
use axum::extract::Path;
use axum::response::Redirect;
use axum::Form;
use axum::{extract::State, response::Html};
use minijinja::context;
use reqwest::Url;
use serde::Deserialize;
use uuid::Uuid;

pub async fn get_article<S>(
    State(state): State<S>,
    Path((feed_id, article_id)): Path<(Uuid, Uuid)>,
) -> Html<String>
where
    S: AppState,
{
    let article_content = state
        .rss_service()
        .get_item_content(feed_id, article_id)
        .await;

    if let Ok(content) = article_content {
        let rendered_article = state
            .template_service()
            .render_template(TEMPLATE_NAME_ARTICLE, context! { article => content });

        Html(rendered_article)
    } else {
        Html("<h1> Article not found <h1>".to_owned())
    }
}

pub async fn get_article_list<S>(State(state): State<S>, Path(feed_id): Path<Uuid>) -> Html<String>
where
    S: AppState,
{
    let (feed, articles) = state.rss_service().get_channel(feed_id).await.unwrap();

    let rendered_article = state.template_service().render_template(
        TEMPLATE_NAME_ARTICLE_LIST,
        context! { feed => feed, articles => articles },
    );

    Html(rendered_article)
}

pub async fn get_feed_list<S>(State(state): State<S>) -> Html<String>
where
    S: AppState,
{
    let feeds = state.feed_service().get_feed_list().await;

    let rendered_article = state
        .template_service()
        .render_template(TEMPLATE_NAME_FEED_LIST, context! { feeds => feeds });

    Html(rendered_article)
}

pub async fn add_new_feed_form<S>(State(state): State<S>) -> Html<String>
where
    S: AppState,
{
    Html(
        state
            .template_service()
            .render_template(TEMPLATE_NAME_FEED_ADD, context! {}),
    )
}

#[derive(Deserialize, Debug)]
pub struct FeedAddForm {
    pub url: String,
}

pub async fn add_new_feed<S>(State(state): State<S>, Form(rss_url): Form<FeedAddForm>) -> Redirect
where
    S: AppState,
{
    // TODO: On fail redirect to the error page
    let url = Url::try_from(rss_url.url.as_str()).expect("Invalid url");
    state.feed_service().add_feed(url).await;
    Redirect::to("/")
}
