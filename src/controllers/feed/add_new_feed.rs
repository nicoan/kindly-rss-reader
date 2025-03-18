use crate::controllers::ApiError;
use crate::services::feed::FeedService;
use crate::state::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::Form;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FeedAddForm {
    pub url: String,
}

pub async fn add_new_feed<S>(
    State(state): State<S>,
    Form(rss_url): Form<FeedAddForm>,
) -> Result<Redirect, ApiError>
where
    S: AppState,
{
    let url = Url::try_from(rss_url.url.as_str()).map_err(|e| anyhow::anyhow!("{e}"))?;
    state.feed_service().add_feed(url).await?;
    Ok(Redirect::to("/"))
}
