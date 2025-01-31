//! This implementation of FeedContentRepository saves the content as html files in the filesystem
use crate::{config::Config, models::article::Article, repositories::RepositoryError};
use axum::async_trait;
use sqlite::ConnectionThreadSafe;
use std::{path::Path, sync::Arc};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use super::{FeedContentRepository, Result};

pub struct FeedContentFsRepositoryImpl {
    connection: Arc<ConnectionThreadSafe>,
    config: Arc<Config>,
}

impl FeedContentFsRepositoryImpl {
    pub fn new(connection: Arc<ConnectionThreadSafe>, config: Arc<Config>) -> Self {
        Self { connection, config }
    }
}

#[async_trait]
impl FeedContentRepository for FeedContentFsRepositoryImpl {
    async fn get_article_content(&self, feed_id: Uuid, article_id: Uuid) -> Result<Option<String>> {
        let file_path = self
            .connection
            .prepare("SELECT content FROM article WHERE id = ? AND feed_id = ?")?
            .into_iter()
            .bind((1, article_id.to_string().as_str()))?
            .bind((2, feed_id.to_string().as_str()))?
            .nth(0)
            .map(|r| {
                r.map_err(|e| RepositoryError::Unexpected(e.into()))
                    .map(|row| row.read::<Option<&str>, _>("content").map(|p| p.to_owned()))
            })
            .transpose()?
            .flatten();

        println!("{file_path:?}");

        if let Some(path) = file_path {
            Ok(Some(
                fs::read_to_string(path)
                    .await
                    .map_err(|e| RepositoryError::Unexpected(e.into()))?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn save_article_content(&self, articles: &[(&Article, &String)]) -> Result<()> {
        self.connection.execute("BEGIN")?;

        for (article, content) in articles {
            let file_path = format!(
                "{}/articles/{}/{}.html",
                self.config.data_path, article.feed_id, article.id
            );

            let file_directory = Path::new(&file_path).parent().ok_or_else(|| {
                RepositoryError::Unexpected(anyhow::anyhow!(format!(
                    "there was an error getting the parent path for {file_path}"
                )))
            })?;

            fs::create_dir_all(file_directory)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.into()))?;

            // Create the file and write the content
            let mut file = fs::File::create(&file_path)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.into()))?;

            fs::File::write(&mut file, content.as_bytes())
                .await
                .map_err(|e| RepositoryError::Unexpected(e.into()))?;

            let mut stmt = self
                .connection
                .prepare("UPDATE article SET content = ? WHERE id = ? AND feed_id = ?")?;
            stmt.bind((1, file_path.as_str()))?;
            stmt.bind((2, article.id.to_string().as_str()))?;
            stmt.bind((3, article.feed_id.to_string().as_str()))?;

            stmt.next()?;
            stmt.reset()?;
            drop(stmt);
        }
        self.connection.execute("END")?;

        Ok(())
    }
}
