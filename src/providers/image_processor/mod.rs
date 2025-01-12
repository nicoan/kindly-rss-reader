mod error;
mod image_processor_fs_impl;

use axum::async_trait;
pub use error::ImageProcessorError;
pub use image_processor_fs_impl::ImageProcessorFsImpl;

type Result<T> = std::result::Result<T, ImageProcessorError>;

#[async_trait]
pub trait ImageProcessor: Sync + Send {
    /// Process an image url and returns the result of the processing.
    ///
    /// For example:
    ///   - If we save it to the file system, we return the image path.
    ///   - We can also embed the image information in the src attribute of the img tag, in that
    ///   case it should return the encoded image information
    async fn process_image_url(&self, url: &str) -> Result<String>;
}
