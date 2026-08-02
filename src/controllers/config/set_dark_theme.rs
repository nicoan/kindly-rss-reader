// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use crate::controllers::ApiError;
use crate::services::persisted_config::PersistedConfigService;
use crate::state::AppState;
use axum::extract::State;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DarkThemeData {
    pub dark_theme: bool,
}

pub async fn set_dark_theme<S>(
    State(state): State<S>,
    Form(dark_theme_data): Form<DarkThemeData>,
) -> Result<(), ApiError>
where
    S: AppState,
{
    state
        .persisted_config_service()
        .set_dark_theme(dark_theme_data.dark_theme)
        .await?;

    Ok(())
}
