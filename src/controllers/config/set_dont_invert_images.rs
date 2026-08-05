use crate::controllers::ApiError;
use crate::services::persisted_config::PersistedConfigService;
use crate::state::AppState;
use axum::extract::State;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DontInvertImagesData {
    pub dont_invert_images: bool,
}

pub async fn set_dont_invert_images<S>(
    State(state): State<S>,
    Form(dont_invert_images_data): Form<DontInvertImagesData>,
) -> Result<(), ApiError>
where
    S: AppState,
{
    state
        .persisted_config_service()
        .set_dont_invert_images(dont_invert_images_data.dont_invert_images)
        .await?;

    Ok(())
}
