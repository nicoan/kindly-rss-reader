use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlite::Row;
use uuid::Uuid;

use crate::repositories::RepositoryError;

#[derive(Serialize, Debug)]
pub struct Article {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub title: String,
    pub author: Option<String>,
    pub guid: String,
    pub link: String,
    pub content: Option<String>,
    pub read: bool,
    pub html_parsed: bool,
    pub last_updated: DateTime<Utc>,
}

impl TryFrom<Row> for Article {
    type Error = RepositoryError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let id = Uuid::from_str(row.read::<&str, _>("id"))
            .map_err(|e| RepositoryError::Deserialization(e.into()))?;

        let feed_id = Uuid::from_str(row.read::<&str, _>("feed_id"))
            .map_err(|e| RepositoryError::Deserialization(e.into()))?;

        Ok(Article {
            id,
            feed_id,
            title: row.read::<&str, _>("title").into(),
            author: row.read::<Option<&str>, _>("author").map(|a| a.to_owned()),
            link: row.read::<&str, _>("link").into(),
            guid: row.read::<&str, _>("guid").into(),
            content: row.read::<Option<&str>, _>("content").map(|s| s.to_owned()),
            read: row.read::<i64, _>("read") != 0,
            html_parsed: row.read::<i64, _>("html_parsed") != 0,
            last_updated: DateTime::from_str(row.read::<&str, _>("last_updated"))
                .map_err(|e: chrono::ParseError| RepositoryError::Deserialization(e.into()))?,
        })
    }
}
