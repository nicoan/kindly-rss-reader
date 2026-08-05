// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use crate::controllers::{ApiError, HtmlResponse};
use crate::services::feed::FeedService;
use crate::services::templates::{TemplateService, TEMPLATE_NAME_ARTICLE_LIST};
use crate::state::AppState;
use crate::view_models::article_list_item::ArticleListItem;
use axum::extract::Path;
use axum::extract::State;
use minijinja::context;
use uuid::Uuid;

pub async fn get_article_list<S>(
    State(state): State<S>,
    Path(feed_id): Path<Uuid>,
) -> Result<HtmlResponse, ApiError>
where
    S: AppState,
{
    let (feed, articles) = state.feed_service().get_channel(feed_id).await?;

    let articles: Vec<ArticleListItem> = articles.into_iter().map(ArticleListItem::from).collect();

    let rendered_html = state
        .template_service()
        .render_template(
            TEMPLATE_NAME_ARTICLE_LIST,
            context! { feed => feed, articles => articles },
        )
        .await?;

    Ok(HtmlResponse::new(rendered_html))
}
