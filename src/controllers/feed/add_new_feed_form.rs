// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use crate::controllers::{ApiError, HtmlResponse};
use crate::services::templates::{TemplateService, TEMPLATE_NAME_FEED_ADD};
use crate::state::AppState;
use axum::extract::State;
use minijinja::context;

pub async fn add_new_feed_form<S>(State(state): State<S>) -> Result<HtmlResponse, ApiError>
where
    S: AppState,
{
    Ok(HtmlResponse::new(
        state
            .template_service()
            .render_template(TEMPLATE_NAME_FEED_ADD, context! {})
            .await?,
    ))
}
