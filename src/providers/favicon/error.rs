use thiserror::Error;

#[derive(Error, Debug)]
pub enum FaviconProviderError {
    #[error("Request error: {0}")]
    RequestError(String),

    #[error("IO error: {0}")]
    IoError(String),
}
