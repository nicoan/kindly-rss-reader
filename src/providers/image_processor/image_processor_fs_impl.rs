//! This implementation will download the images and place them in thh file system
use std::path::Path;

use super::Result;
use axum::async_trait;
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use super::{ImageProcessor, ImageProcessorError};

pub struct ImageProcessorFsImpl<P>
where
    P: AsRef<Path> + Sync + Send,
{
    article_router_path: P,
    article_fs_path: P,
}

impl<P> ImageProcessorFsImpl<P>
where
    P: AsRef<Path> + Sync + Send,
{
    pub fn new(article_router_path: P, article_fs_path: P) -> Self {
        Self {
            article_router_path,
            article_fs_path,
        }
    }
}

#[async_trait]
impl<P> ImageProcessor for ImageProcessorFsImpl<P>
where
    P: AsRef<Path> + Sync + Send,
{
    async fn process_image_url(&self, url: &str) -> Result<String> {
        let mut image_path = self.article_fs_path.as_ref().join("static/");
        fs::create_dir_all(&image_path)
            .await
            .map_err(|e| ImageProcessorError::UnableToProcess(e.into()))?;

        let image_data = reqwest::get(url)
            .await
            .map_err(|e| ImageProcessorError::UnableToDownload(url.to_owned(), e))?
            .bytes()
            .await
            .map_err(|e| ImageProcessorError::UnableToProcess(e.into()))?;

        let image_file_name = Uuid::new_v4().to_string();
        image_path.push(&image_file_name);

        // Create the file and write the content
        let mut file = fs::File::create(&image_path)
            .await
            .map_err(|e| ImageProcessorError::UnableToProcess(e.into()))?;

        fs::File::write(&mut file, &image_data)
            .await
            .map_err(|e| ImageProcessorError::UnableToProcess(e.into()))?;

        self.article_router_path
            .as_ref()
            .to_str()
            .map(|p| format!("{p}/static/{image_file_name}",))
            .ok_or(ImageProcessorError::UnableToProcess(anyhow::anyhow!({
                "unable to convert the path {image_path:?} to string"
            })))
    }
}
