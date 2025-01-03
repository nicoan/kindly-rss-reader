use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon_url: Option<String>,
    pub last_updated: DateTime<Utc>,
}
