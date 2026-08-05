// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
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
