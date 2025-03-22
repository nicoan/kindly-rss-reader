use std::io::BufReader;

use atom_syndication::Feed;
use chrono::Utc;

use super::error::{FeedParserError, Result};
use super::feed_parser_trait::{FeedParser, ParsedFeed, ParsedItem};

pub struct AtomParserImpl;

impl AtomParserImpl {
    pub fn new() -> Self {
        Self
    }
}

impl FeedParser for AtomParserImpl {
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed> {
        let reader = BufReader::new(content);
        let feed = Feed::read_from(reader)?;

        // Find the alternate link (usually the website URL)
        let link = feed
            .links()
            .iter()
            .find(|link| link.rel() == "alternate" || link.rel() == "self")
            .map(|link| link.href().to_owned())
            .ok_or_else(|| FeedParserError::MissingField("link".to_owned()))?;

        let items = feed
            .entries()
            .iter()
            .map(|entry| {
                // Find the alternate link for the entry
                let entry_link = entry
                    .links()
                    .iter()
                    .find(|link| link.rel() == "alternate")
                    .map(|link| link.href().to_owned());

                // Get content or summary
                let content = entry
                    .content()
                    .and_then(|c| c.value.as_ref().map(|v| v.to_owned()))
                    .or_else(|| entry.summary().map(|s| s.value.to_owned()));

                // Get author name if available
                let author = entry.authors().first().map(|a| a.name().to_owned());

                ParsedItem {
                    title: entry.title().to_string(),
                    link: entry_link,
                    guid: Some(entry.id().to_owned()),
                    content,
                    author,
                    pub_date: Some(entry.updated().with_timezone(&Utc)),
                }
            })
            .collect();

        Ok(ParsedFeed {
            title: feed.title().to_string(),
            link,
            items,
        })
    }

    fn can_parse(&self, content: &[u8]) -> bool {
        // Check if content looks like Atom by trying to parse it
        let reader = BufReader::new(content);
        Feed::read_from(reader).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_valid_atom() {
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
        "#.as_bytes();

        let parser = AtomParserImpl::new();
        assert!(parser.can_parse(atom_content));
    }

    #[test]
    fn test_cannot_parse_rss() {
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
        "#.as_bytes();

        let parser = AtomParserImpl::new();
        assert!(!parser.can_parse(rss_content));
    }
}
