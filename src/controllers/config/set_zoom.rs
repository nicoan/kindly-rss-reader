use crate::controllers::ApiError;
use crate::services::persisted_config::PersistedConfigService;
use crate::state::AppState;
use axum::extract::State;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ZoomData {
    pub zoom: f64,
}

pub async fn set_zoom<S>(
    State(state): State<S>,
    Form(zoom_data): Form<ZoomData>,
) -> Result<(), ApiError>
where
    S: AppState,
{
    state
        .persisted_config_service()
        .set_zoom(zoom_data.zoom)
        .await?;

    Ok(())
}
