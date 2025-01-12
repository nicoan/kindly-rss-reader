mod error;
mod html_processor_impl;

use axum::async_trait;
use error::HtmlProcessorError;
pub use html_processor_impl::HtmlProcessorImpl;

use super::image_processor::ImageProcessor;

type Result<T> = std::result::Result<T, HtmlProcessorError>;

#[async_trait]
pub trait HtmlProcessor: Sync + Send {
    /// This function process an Html article. We call "html articles" those that are obtained by
    /// following the link of the RSS feed to an actual html file. In those articles, we only keep
    /// the <main> tag, discarding anything else
    fn process_html_article(&self, html: &str) -> Result<String>;

    /// Fixes the src tag of images.
    /// Usually blogs link images relatively. This function download the image to the cache and
    /// fixes the tag.
    async fn fix_img_src<P>(&self, html: &str, link: &str, image_processor: &P) -> Result<String>
    where
        P: ImageProcessor + ?Sized;

    /// Sanitizes the HTML
    /// Removes potentially harmful tags such as <iframe> and <script>
    fn sanitize(&self, html: &str) -> Result<String>;
}
