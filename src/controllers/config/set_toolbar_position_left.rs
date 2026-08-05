// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use crate::controllers::ApiError;
use crate::services::persisted_config::PersistedConfigService;
use crate::state::AppState;
use axum::extract::State;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ToolbarPositionLeftData {
    pub toolbar_position_left: bool,
}

pub async fn set_toolbar_position_left<S>(
    State(state): State<S>,
    Form(toolbar_position_left_data): Form<ToolbarPositionLeftData>,
) -> Result<(), ApiError>
where
    S: AppState,
{
    state
        .persisted_config_service()
        .set_toolbar_position_left(toolbar_position_left_data.toolbar_position_left)
        .await?;

    Ok(())
}
