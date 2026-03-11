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
