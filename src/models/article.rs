use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Article {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub title: String,
    pub author: String,
    pub description: String,
    pub guid: String,
    // Rename this to content, in databases like postgre we can save it directly in a column, in
    // SQLite we rely on the fs
    pub path: Option<String>,
    pub read: bool,
    pub last_updated: DateTime<Utc>,
}
