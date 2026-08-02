// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
#[derive(Debug, thiserror::Error)]
pub enum HtmlProcessorError {
    #[error("unable to parse html, this usually means the article have no <main> and/or <article> tag(s)")]
    UnableToParse,

    #[error("unexpected error ocurred: {0:?}")]
    Unexpected(#[source] anyhow::Error),
}
