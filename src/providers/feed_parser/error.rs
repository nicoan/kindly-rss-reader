// SPDX-FileCopyrightText: 2025-2026 Keheliya Gallaba
// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
//
// Original implementation by Keheliya Gallaba. Subsequent
// modifications by Nicolás Antinori (AGPL-3.0-only).
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FeedParserError {
    #[error("Failed to parse feed: {0}")]
    ParseError(#[source] anyhow::Error),

    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("Failed to parse date: {0}")]
    DateParseError(#[from] chrono::ParseError),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),
}
