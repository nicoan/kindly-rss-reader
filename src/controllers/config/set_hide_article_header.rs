use crate::controllers::ApiError;
use crate::services::persisted_config::PersistedConfigService;
use crate::state::AppState;
use axum::extract::State;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HideArticleHeaderData {
    pub hide_article_header: bool,
}

pub async fn set_hide_article_header<S>(
    State(state): State<S>,
    Form(hide_article_header_data): Form<HideArticleHeaderData>,
) -> Result<(), ApiError>
where
    S: AppState,
{
    state
        .persisted_config_service()
        .set_hide_article_header(hide_article_header_data.hide_article_header)
        .await?;

    Ok(())
}
