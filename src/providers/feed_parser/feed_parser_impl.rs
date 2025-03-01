use super::FeedParser;
use super::FeedParserError;
use crate::models::feed::Feed;
use anyhow::anyhow;
use atom_syndication::Feed as AtomFeed;
use axum::async_trait;
use axum::body::Bytes;
use chrono::{DateTime, Utc};
use reqwest::Url;
use rss::Channel;
use uuid::Uuid;

pub struct FeedParserImpl;

impl FeedParserImpl {
    pub fn new() -> Self {
        Self {}
    }

    fn try_parse_rss(&self, content: &Bytes) -> Result<Option<Channel>, FeedParserError> {
        match Channel::read_from(&content[..]) {
            Ok(channel) => Ok(Some(channel)),
            Err(_) => Ok(None), // Not RSS format, could be Atom
        }
    }

    fn try_parse_atom(&self, content: &Bytes) -> Result<Option<AtomFeed>, FeedParserError> {
        match AtomFeed::read_from(&content[..]) {
            Ok(feed) => Ok(Some(feed)),
            Err(_) => Ok(None), // Not Atom format, could be RSS
        }
    }

    fn extract_link_from_url(&self, feed_url: &str) -> Result<String, FeedParserError> {
        let feed_url = Url::parse(feed_url)
            .map_err(|e| FeedParserError::Unexpected(anyhow!(e)))?;
        
        let link_path = feed_url.path_segments();
        let link = if let Some(link_path) = link_path {
            let mut final_path = link_path.collect::<Vec<&str>>();
            if !final_path.is_empty() {
                final_path.remove(final_path.len() - 1);
            }
            let final_path = final_path.join("/");
            let mut link = feed_url.origin().unicode_serialization();
            link.push('/');
            link.push_str(&final_path);
            link
        } else {
            feed_url.origin().unicode_serialization()
        };
        
        Ok(link)
    }

    fn parse_rss_date(&self, date_str: &str) -> Result<DateTime<Utc>, FeedParserError> {
        DateTime::parse_from_rfc2822(date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(FeedParserError::DateParsingError)
    }

    fn parse_atom_date(&self, date_str: &str) -> Result<DateTime<Utc>, FeedParserError> {
        // First try standard RFC 3339 format
        match DateTime::parse_from_rfc3339(date_str) {
            Ok(dt) => Ok(dt.with_timezone(&Utc)),
            Err(_) => {
                // Try to clean the string and parse again
                let cleaned_date = date_str
                    .trim()
                    .replace(['\n', '\r', '\t'], "");
                
                DateTime::parse_from_rfc3339(&cleaned_date)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(FeedParserError::DateParsingError)
            }
        }
    }
}

#[async_trait]
impl FeedParser for FeedParserImpl {
    async fn parse_feed(&self, feed_url: &str, content: &Bytes) -> Result<Feed, FeedParserError> {
        // Try to parse as RSS first
        if let Some(channel) = self.try_parse_rss(content)? {
            let link = self.extract_link_from_url(feed_url)?;
            
            return Ok(Feed {
                id: Uuid::new_v4(),
                title: channel.title,
                link,
                url: feed_url.to_string(),
                favicon_url: None,
                last_updated: DateTime::default(),
                unread_count: 0,
            });
        }
        
        // Try to parse as Atom
        if let Some(atom_feed) = self.try_parse_atom(content)? {
            let link = if let Some(link) = atom_feed.links().iter().find(|l| l.rel() == "alternate") {
                link.href().to_string()
            } else {
                self.extract_link_from_url(feed_url)?
            };
            
            return Ok(Feed {
                id: Uuid::new_v4(),
                title: atom_feed.title().to_string(),
                link,
                url: feed_url.to_string(),
                favicon_url: None,
                last_updated: DateTime::default(),
                unread_count: 0,
            });
        }
        
        // Neither RSS nor Atom
        Err(FeedParserError::InvalidFeedFormat)
    }
    
    async fn parse_feed_items(
        &self, 
        feed_id: Uuid, 
        feed_link: String,
        content: &Bytes
    ) -> Result<Vec<(String, Option<String>)>, FeedParserError> {
        let mut items = Vec::new();
        
        // Try to parse as RSS first
        if let Some(channel) = self.try_parse_rss(content)? {
            for item in channel.items() {
                let guid = if let Some(guid) = item.guid() {
                    guid.value().to_string()
                } else if let Some(link) = item.link() {
                    link.to_string()
                } else {
                    return Err(FeedParserError::MissingFields(
                        "RSS item missing both guid and link".to_string()
                    ));
                };
                
                let title = item.title()
                    .unwrap_or("Unknown title")
                    .to_string();
                
                let link = if let Some(link) = item.link() {
                    link.to_string()
                } else {
                    return Err(FeedParserError::MissingFields(
                        "RSS item missing link".to_string()
                    ));
                };
                
                let pub_date = if let Some(date) = item.pub_date() {
                    self.parse_rss_date(date)?
                } else {
                    Utc::now()
                };
                
                let author = item.author().map(String::from);
                
                let content = item.content().map(String::from);
                
                // For RSS, we store the item information as JSON to match the Article structure
                let item_json = serde_json::json!({
                    "title": title,
                    "link": link,
                    "guid": guid,
                    "author": author,
                    "date": pub_date.to_rfc3339(),
                    "html_parsed": false
                });
                
                items.push((item_json.to_string(), content));
            }
            
            return Ok(items);
        }
        
        // Try to parse as Atom
        if let Some(atom_feed) = self.try_parse_atom(content)? {
            for entry in atom_feed.entries() {
                let id = entry.id().to_string();
                
                let title = entry.title().to_string();
                
                let link = if let Some(link) = entry.links().iter().find(|l| l.rel() == "alternate") {
                    link.href().to_string()
                } else {
                    return Err(FeedParserError::MissingFields(
                        "Atom entry missing alternate link".to_string()
                    ));
                };
                
                // Parse the date directly - convert DateTime to string first
                let pub_date = if let Some(published) = entry.published() {
                    // Convert DateTime to RFC3339 string and then parse it
                    let date_str = published.to_rfc3339();
                    self.parse_atom_date(&date_str)?
                } else {
                    // Convert DateTime to RFC3339 string and then parse it
                    let date_str = entry.updated().to_rfc3339();
                    self.parse_atom_date(&date_str)?
                };
                
                let author = entry.authors().first().map(|a| a.name().to_string());
                
                let content = entry.content().and_then(|c| c.value().map(String::from));
                
                // For Atom, we store the entry information as JSON to match the Article structure
                let item_json = serde_json::json!({
                    "title": title,
                    "link": link,
                    "guid": id,
                    "author": author,
                    "date": pub_date.to_rfc3339(),
                    "html_parsed": false
                });
                
                items.push((item_json.to_string(), content));
            }
            
            return Ok(items);
        }
        
        // Neither RSS nor Atom
        Err(FeedParserError::InvalidFeedFormat)
    }
    
    fn get_item_identifier(&self, item: &(String, Option<String>)) -> String {
        // Parse the JSON string to extract the guid
        let parsed: serde_json::Value = serde_json::from_str(&item.0)
            .unwrap_or_else(|_| serde_json::json!({}));
        
        if let Some(guid) = parsed.get("guid").and_then(|g| g.as_str()) {
            guid.to_string()
        } else if let Some(link) = parsed.get("link").and_then(|l| l.as_str()) {
            link.to_string()
        } else {
            // Fallback
            item.0.clone()
        }
    }
}
