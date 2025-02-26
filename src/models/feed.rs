use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::Row;
use uuid::Uuid;

use crate::repositories::RepositoryError;

#[derive(Serialize)]
pub struct Feed {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub link: String,
    pub favicon_url: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub unread_count: u16,
}

impl TryFrom<Row> for Feed {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let id = Uuid::from_str(row.read::<&str, _>("id"))
            .map_err(|e| RepositoryError::Deserialization(e.into()))?;

        Ok(Feed {
            id,
            title: row.read::<&str, _>("title").into(),
            url: row.read::<&str, _>("url").into(),
            link: row.read::<&str, _>("link").into(),
            favicon_url: row
                .read::<Option<&str>, _>("favicon_path")
                .map(|s| s.to_owned()),
            last_updated: DateTime::from_str(row.read::<&str, _>("last_updated"))
                .map_err(|e: chrono::ParseError| RepositoryError::Deserialization(e.into()))?,
            unread_count: row.read::<i64, _>("unread_count").clamp(0, u16::MAX as i64) as u16,
        })
    }
}
