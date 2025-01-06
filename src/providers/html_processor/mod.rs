mod html_processor_impl;

use axum::async_trait;
pub use html_processor_impl::HtmlProcessorImpl;
use std::error::Error;
use std::path::Path;

#[async_trait]
pub trait HtmlProcessor: Sync + Send {
    /// This function process an Html article. We call "html articles" those that are obtained by
    /// following the link of the RSS feed to an actual html file. In those articles, we only keep
    /// the <main> tag, discarding anything else
    fn process_html_article(html: &str) -> Result<String, Box<dyn Error>>;

    /// Fixes the src tag of images.
    /// Usually blogs link images relatively. This function download the image to the cache and
    /// fixes the tag.
    async fn fix_img_src(html: &str, link: &str, article_path: impl AsRef<Path> + Send) -> String;

    fn sanitize(html: &str) -> String;
}
