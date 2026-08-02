// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use crate::controllers::{ApiError, HtmlResponse};
use crate::services::templates::TEMPLATE_NAME_FEED_LIST;
use crate::services::{feed::FeedService, templates::TemplateService};
use crate::state::AppState;
use axum::extract::State;
use minijinja::context;

pub async fn get_feed_list<S>(State(state): State<S>) -> Result<HtmlResponse, ApiError>
where
    S: AppState,
{
    let feeds = state.feed_service().get_feed_list().await?;

    let rendered_html = state
        .template_service()
        .render_template(TEMPLATE_NAME_FEED_LIST, context! { feeds => feeds })
        .await?;

    Ok(HtmlResponse::new(rendered_html))
}
