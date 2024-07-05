use axum::async_trait;
use rss::Channel;

use super::RssService;

#[derive(Clone)]
pub struct RssServiceImpl;

impl RssServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RssService for RssServiceImpl {
    async fn get_channel(
        &self,
        url: impl AsRef<str> + Send,
    ) -> Result<Channel, Box<dyn std::error::Error>> {
        let content = reqwest::get(url.as_ref()).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    async fn get_item_content(&self, item: &rss::Item) -> Result<String, String> {
        if let Some(content) = item.content() {
            Ok(content.to_owned())
        } else if let Some(content_url) = item.link() {
            Ok(reqwest::get(content_url)
                .await
                .expect("unable to get content url")
                .text()
                .await
                .expect("unable to get content from content url"))
        } else {
            Err("<h1> Unable to get content <h1>".to_owned())
        }
    }
}
