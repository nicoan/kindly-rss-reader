use crate::services::feed::FeedService;
use crate::state::AppState;
use axum::extract::State;
use axum::response::{Html, Redirect};
use axum::Form;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FeedAddForm {
    pub url: String,
}

// TODO: On fail redirect to the error page
pub async fn add_new_feed<S>(
    State(state): State<S>,
    Form(rss_url): Form<FeedAddForm>,
) -> Result<Redirect, Html<String>>
where
    S: AppState,
{
    let url = Url::try_from(rss_url.url.as_str()).map_err(|e| {
        tracing::error!("{:?}", e);
        Html("<h1> There was an error rendering adding the feed. Invalid url. </h1>".to_owned())
    })?;
    state.feed_service().add_feed(url).await.map_err(|e| {
        tracing::error!("{:?}", e);
        Html(
            "<h1> There was an error rendering adding the feed. Please check the logs. </h1>"
                .to_owned(),
        )
    })?;

    Ok(Redirect::to("/"))
}
