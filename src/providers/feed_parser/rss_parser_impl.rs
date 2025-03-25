use super::{error::FeedParserError, FeedParser, Result};
use crate::models::parsed_feed::{ParsedFeed, ParsedItem};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use rss::Channel;
use std::io::BufReader;

pub struct RssParserImpl;

impl RssParserImpl {
    // TODO: This should be in other struct
    fn parse_date(&self, date_str: &str) -> Result<DateTime<Utc>> {
        DateTime::parse_from_rfc2822(date_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(FeedParserError::DateParseError)
    }
}

impl FeedParser for RssParserImpl {
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed> {
        let reader = BufReader::new(content);
        let channel =
            Channel::read_from(reader).map_err(|e| FeedParserError::ParseError(anyhow!(e)))?;

        let items = channel
            .items()
            .iter()
            .map(|item| {
                let pub_date = item.pub_date().and_then(|date| self.parse_date(date).ok());

                ParsedItem {
                    title: item.title().unwrap_or("Unknown title").to_owned(),
                    link: item.link().map(|s| s.to_owned()),
                    guid: item.guid().map(|g| g.value().to_owned()),
                    content: item.content().map(|s| s.to_owned()),
                    author: item.author().map(|s| s.to_owned()),
                    pub_date,
                }
            })
            .collect();

        let link = channel.link().to_owned();

        Ok(ParsedFeed {
            title: channel.title().to_owned(),
            link,
            items,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_valid_rss() {
        let rss_content = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
            <channel>
                <title>Test RSS Feed</title>
                <link>https://example.com/feed</link>
                <description>A test RSS feed</description>
                <item>
                    <title>Test Item</title>
                    <link>https://example.com/item1</link>
                    <guid>https://example.com/item1</guid>
                    <pubDate>Mon, 01 Jan 2023 12:00:00 GMT</pubDate>
                </item>
            </channel>
        </rss>
        "#
        .as_bytes();

        assert!(RssParserImpl.parse_feed(rss_content).is_ok());
    }

    #[test]
    fn test_cannot_parse_atom() {
        let atom_content = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
            <title>Test Atom Feed</title>
            <link href="https://example.com/feed"/>
            <updated>2023-01-01T12:00:00Z</updated>
            <id>https://example.com/feed</id>
            <entry>
                <title>Test Entry</title>
                <link href="https://example.com/entry1"/>
                <id>https://example.com/entry1</id>
                <updated>2023-01-01T12:00:00Z</updated>
            </entry>
        </feed>
        "#
        .as_bytes();

        assert!(RssParserImpl.parse_feed(atom_content).is_err());
    }
}
