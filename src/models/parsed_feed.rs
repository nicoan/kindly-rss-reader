// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use chrono::{DateTime, Utc};

pub struct ParsedFeed {
    pub title: String,
    pub link: String,
    pub items: Vec<ParsedItem>,
}

pub struct ParsedItem {
    pub title: String,
    pub link: Option<String>,
    pub guid: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
}
