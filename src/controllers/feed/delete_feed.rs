use crate::services::feed::FeedService;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use uuid::Uuid;

pub async fn delete_feed<S: AppState>(
    State(state): State<S>,
    Path(feed_id): Path<Uuid>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    state
        .feed_service()
        .delete_feed(feed_id)
        .await
        .map_err(|e| {
            tracing::error!("Error deleting feed: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete feed")
        })?;
    
    Ok(Redirect::to("/"))
}