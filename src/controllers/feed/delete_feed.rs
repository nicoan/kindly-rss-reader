// SPDX-FileCopyrightText: 2025-2026 Keheliya Gallaba
// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
//
// Original implementation by Keheliya Gallaba. Subsequent
// modifications by Nicolás Antinori (AGPL-3.0-only).
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
