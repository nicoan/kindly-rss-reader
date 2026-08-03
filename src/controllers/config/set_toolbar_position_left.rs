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
