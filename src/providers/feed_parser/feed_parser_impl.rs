use super::atom_parser_impl::AtomParserImpl;
use super::feed_parser_trait::{FeedParser, ParsedFeed};
use super::rss_parser_impl::RssParserImpl;
use super::Result;

pub struct FeedParserImpl {
    rss_parser: RssParserImpl,
    atom_parser: AtomParserImpl,
}

impl Default for FeedParserImpl {
    fn default() -> Self {
        Self {
            rss_parser: RssParserImpl,
            atom_parser: AtomParserImpl,
        }
    }
}

impl FeedParser for FeedParserImpl {
    fn parse_feed(&self, content: &[u8]) -> Result<ParsedFeed> {
        if let Ok(parsed_feed) = self.rss_parser.parse_feed(content) {
            Ok(parsed_feed)
        } else if let Ok(parsed_feed) = self.atom_parser.parse_feed(content) {
            Ok(parsed_feed)
        } else {
            Err(super::error::FeedParserError::UnsupportedFormat)
        }
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
        "#
        .as_bytes();

        let parser = FeedParserImpl::default();
        assert!(parser.parse_feed(rss_content).is_ok());
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
        "#
        .as_bytes();

        let parser = FeedParserImpl::default();
        assert!(parser.parse_feed(atom_content).is_ok());
    }

    #[test]
    fn test_cannot_parse_invalid_content() {
        let invalid_content = b"This is not a valid feed";
        let parser = FeedParserImpl::default();
        assert!(parser.parse_feed(invalid_content).is_err());
    }
}
