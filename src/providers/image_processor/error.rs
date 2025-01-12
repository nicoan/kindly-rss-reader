#[derive(Debug, thiserror::Error)]
pub enum ImageProcessorError {
    #[error("unable to download image from {0}: {1:?}")]
    UnableToDownload(String, #[source] reqwest::Error),

    #[error("unable to process image content{0}")]
    UnableToProcess(#[source] anyhow::Error),
}
