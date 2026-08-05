// SPDX-FileCopyrightText: 2025-2026 Keheliya Gallaba
// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
//
// Original implementation by Keheliya Gallaba. Subsequent
// modifications by Nicolás Antinori (AGPL-3.0-only).
mod atom_parser_impl;
mod error;
mod rss_parser_impl;

pub use atom_parser_impl::AtomParserImpl;
pub use error::FeedParserError;
pub use rss_parser_impl::RssParserImpl;

use crate::models::parsed_feed::ParsedFeed;

pub(crate) type Result<T> = std::result::Result<T, FeedParserError>;

/// Trait defining the common interface for all feed parsers
pub trait FeedParser: Send + Sync {
    /// Parse feed content and return feed metadata
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed>;
}
