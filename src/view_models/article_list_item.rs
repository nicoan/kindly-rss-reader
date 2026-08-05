// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use serde::Serialize;
use uuid::Uuid;

use crate::models::article::Article;

#[derive(Serialize)]
pub struct ArticleListItem {
    id: Uuid,
    title: String,
    author: String,
    date: String,
    read: bool,
}

impl From<Article> for ArticleListItem {
    fn from(value: Article) -> Self {
        Self {
            id: value.id,
            title: value.title,
            author: value.author.unwrap_or_default(),
            date: value.last_updated.format("%B %d, %Y").to_string(),
            read: value.read,
        }
    }
}
