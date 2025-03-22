use std::sync::Arc;

use super::atom_parser_impl::AtomParserImpl;
use super::error::Result;
use super::feed_parser_trait::{FeedParser, ParsedFeed};
use super::rss_parser_impl::RssParserImpl;

/// Factory implementation that selects the appropriate parser based on the feed content
pub struct FeedParserImpl {
    rss_parser: Arc<RssParserImpl>,
    atom_parser: Arc<AtomParserImpl>,
}

impl FeedParserImpl {
    pub fn new() -> Self {
        Self {
            rss_parser: Arc::new(RssParserImpl::new()),
            atom_parser: Arc::new(AtomParserImpl::new()),
        }
    }
}

impl FeedParser for FeedParserImpl {
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed> {
        if self.rss_parser.can_parse(content) {
            self.rss_parser.parse_feed(content)
        } else if self.atom_parser.can_parse(content) {
            self.atom_parser.parse_feed(content)
        } else {
            Err(super::error::FeedParserError::UnsupportedFormat)
        }
    }

    fn can_parse(&self, content: &[u8]) -> bool {
        self.rss_parser.can_parse(content) || self.atom_parser.can_parse(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_rss() {
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

        let parser = FeedParserImpl::new();
        assert!(parser.can_parse(rss_content));
    }

    #[test]
    fn test_can_parse_atom() {
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

        let parser = FeedParserImpl::new();
        assert!(parser.can_parse(atom_content));
    }

    #[test]
    fn test_cannot_parse_invalid_content() {
        let invalid_content = b"This is not a valid feed";
        let parser = FeedParserImpl::new();
        assert!(!parser.can_parse(invalid_content));
    }
}
