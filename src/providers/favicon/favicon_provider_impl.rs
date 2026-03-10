use super::{FaviconProvider, Result, FaviconProviderError};
use crate::config::Config;
use axum::async_trait;
use axum::body::Bytes;
use scraper::{Html, Selector};
use std::sync::Arc;

pub struct FaviconProviderImpl {
    config: Arc<Config>,
    favicon_router_path: &'static str,
}

impl FaviconProviderImpl {
    pub fn new(config: Arc<Config>, favicon_router_path: &'static str) -> Self {
        Self {
            config,
            favicon_router_path,
        }
    }

    fn get_favicon_dir(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.config.data_path).join("favicons")
    }

    fn get_favicon_file_path(&self, feed_id: &str) -> std::path::PathBuf {
        self.get_favicon_dir().join(format!("{}.png", feed_id))
    }

    fn extract_base_url(&self, feed_link: &str) -> String {
        if let Ok(url) = reqwest::Url::parse(feed_link) {
            let host = url.host_str().unwrap_or("");
            let scheme = url.scheme();
            
            let known_feed_prefixes = ["feeds.", "feed.", "rss.", "atom."];
            
            for prefix in known_feed_prefixes.iter() {
                if host.contains(prefix) {
                    let parts: Vec<&str> = host.split('.').collect();
                    if parts.len() > 2 {
                        let tld = parts.last().unwrap_or(&"");
                        let domain = parts.get(parts.len() - 2).unwrap_or(&"");
                        
                        return format!("{}://www.{}.{}/", scheme, domain, tld);
                    }
                }
            }
            
            return format!("{}://{}/", scheme, host);
        }
        feed_link.trim_end_matches('/').to_string()
    }

    async fn parse_favicon_from_html(&self, base_url: &str) -> Option<String> {
        // Fetch the HTML from the base URL
        let response = reqwest::get(base_url).await.ok()?;
        if !response.status().is_success() {
            return None;
        }

        let html = response.text().await.ok()?;
        let document = Html::parse_document(&html);

        // Try to find favicon link tags with various rel attributes
        // Check for: rel="icon", rel="shortcut icon", rel="icon shortcut", etc.
        let selectors = vec![
            r#"link[rel~="icon"]"#,
            r#"link[rel="shortcut icon"]"#,
            r#"link[rel="icon shortcut"]"#,
        ];

        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    if let Some(href) = element.value().attr("href") {
                        // Convert relative URLs to absolute
                        return Some(self.resolve_favicon_url(base_url, href));
                    }
                }
            }
        }

        None
    }

    fn resolve_favicon_url(&self, base_url: &str, href: &str) -> String {
        // If the href is already absolute, return it
        if href.starts_with("http://") || href.starts_with("https://") {
            return href.to_string();
        }

        // Parse the base URL
        if let Ok(mut url) = reqwest::Url::parse(base_url) {
            // Handle protocol-relative URLs
            if href.starts_with("//") {
                return format!("{}:{}", url.scheme(), href);
            }

            // Handle absolute paths
            if href.starts_with('/') {
                url.set_path(href);
                return url.to_string();
            }

            // Handle relative paths
            let base_path = url.path().trim_end_matches('/');
            url.set_path(&format!("{}/{}", base_path, href));
            return url.to_string();
        }

        // Fallback: just append the href to base_url
        format!("{}/{}", base_url.trim_end_matches('/'), href.trim_start_matches('/'))
    }

    async fn try_download_favicon(&self, url: &str) -> Option<Bytes> {
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                return Some(bytes);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
        None
    }

    async fn try_common_favicon_paths(&self, base_url: &str) -> Option<Bytes> {
        // Try common favicon locations as fallback
        let common_paths = vec![
            "/favicon.ico",
            "/favicon.png",
            "/apple-touch-icon.png",
        ];

        for path in common_paths {
            let url = format!("{}{}", base_url.trim_end_matches('/'), path);
            if let Some(bytes) = self.try_download_favicon(&url).await {
                return Some(bytes);
            }
        }

        None
    }
}

#[async_trait]
impl FaviconProvider for FaviconProviderImpl {
    async fn download_favicon(&self, feed_link: &str, feed_id: &str) -> Result<Option<String>> {
        let base_url = self.extract_base_url(feed_link);

        let favicon_dir = self.get_favicon_dir();
        tokio::fs::create_dir_all(&favicon_dir)
            .await
            .map_err(|e| FaviconProviderError::IoError(e.to_string()))?;

        let favicon_path = self.get_favicon_file_path(feed_id);

        // Step 1: Try to parse favicon URL from HTML
        if let Some(favicon_url) = self.parse_favicon_from_html(&base_url).await {
            if let Some(bytes) = self.try_download_favicon(&favicon_url).await {
                tokio::fs::write(&favicon_path, &bytes)
                    .await
                    .map_err(|e| FaviconProviderError::IoError(e.to_string()))?;

                let relative_path = format!("{}/{}.png", self.favicon_router_path, feed_id);
                return Ok(Some(relative_path));
            }
        }

        // Step 2: Fallback to common favicon paths
        if let Some(bytes) = self.try_common_favicon_paths(&base_url).await {
            tokio::fs::write(&favicon_path, &bytes)
                .await
                .map_err(|e| FaviconProviderError::IoError(e.to_string()))?;

            let relative_path = format!("{}/{}.png", self.favicon_router_path, feed_id);
            return Ok(Some(relative_path));
        }

        Ok(None)
    }
}
