// SPDX-FileCopyrightText: 2025-2026 Keheliya Gallaba
// SPDX-License-Identifier: AGPL-3.0-only
mod error;
mod favicon_provider_impl;

pub use error::FaviconProviderError;
pub use favicon_provider_impl::FaviconProviderImpl;

use axum::async_trait;

pub(crate) type Result<T> = std::result::Result<T, FaviconProviderError>;

#[async_trait]
pub trait FaviconProvider: Sync + Send {
    async fn download_favicon(&self, feed_link: &str, feed_id: &str) -> Result<Option<String>>;
}
