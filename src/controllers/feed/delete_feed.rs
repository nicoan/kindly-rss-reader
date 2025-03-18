use crate::state::AppState;
use crate::{controllers::ApiError, services::feed::FeedService};
use axum::{
    extract::{Path, State},
    response::Redirect,
};
use uuid::Uuid;

pub async fn delete_feed<S: AppState>(
    State(state): State<S>,
    Path(feed_id): Path<Uuid>,
) -> Result<Redirect, ApiError> {
    state.feed_service().delete_feed(feed_id).await?;

    Ok(Redirect::to("/"))
}
