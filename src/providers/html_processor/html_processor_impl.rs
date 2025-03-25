//! TODO: Implement a proper parser!!!
use crate::providers::image_processor::ImageProcessor;

use super::{error::HtmlProcessorError, Result};
use axum::async_trait;
use regex::Regex;

use super::HtmlProcessor;

#[derive(Clone)]
pub struct HtmlProcessorImpl {
    img_tag_regex: Regex,
    tag_removal_regex: Regex,
    attr_removal_regex: Regex,
    favicon_url: Regex,
}

impl HtmlProcessorImpl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            img_tag_regex: Regex::new(r#"<img[^>]*\bsrc\s*=\s*['"]([^'"]+)['"][^>]*>"#)
                .map_err(|e| HtmlProcessorError::Unexpected(e.into()))?,
            tag_removal_regex: Regex::new(
                r"(?si)<iframe.*?(</iframe>|/>)|<script.*?(</script>|/>)",
            )
            .map_err(|e| HtmlProcessorError::Unexpected(e.into()))?,
            attr_removal_regex: Regex::new(r#"(?i)\b(on\w+|javascript:)[^"'<>]*=['"][^"']*['"]"#)
                .map_err(|e| HtmlProcessorError::Unexpected(e.into()))?,
            favicon_url: regex::Regex::new(
                r#"(?i)<link[^>]*rel=["'][^"']*icon[^"']*["'][^>]*href=["']([^"']+)["']"#,
            )
            .map_err(|e| HtmlProcessorError::Unexpected(e.into()))?,
        })
    }

    fn extract_content_between_tag(html: &str, tag: &str) -> Result<Option<String>> {
        // let start_tag = "<main";
        let start_tag = Regex::new(&format!(r"(?i)<\s*{tag}[^>]*>"))
            .map_err(|e| HtmlProcessorError::Unexpected(e.into()))?;
        let end_tag = &format!("</{tag}>");

        let start_tag = if let Some(st) = start_tag.find(html) {
            st
        } else {
            return Ok(None);
        };

        // Locate the start and end positions of the <article> content
        if let Some(end_idx) = html.find(end_tag) {
            // Extract the content between the <main> tags
            let start = start_tag.end();
            let article_content = &html[start..end_idx];
            return Ok(Some(article_content.to_string()));
        }

        Ok(None)
    }
}

#[async_trait]
impl HtmlProcessor for HtmlProcessorImpl {
    fn process_html_article(&self, html: &str) -> Result<String> {
        let content = Self::extract_content_between_tag(html, "main")?;
        if let Some(content) = content {
            return Ok(content);
        }

        let content = Self::extract_content_between_tag(html, "article")?;
        if let Some(content) = content {
            return Ok(content);
        }

        Err(HtmlProcessorError::UnableToParse)
    }

    async fn fix_img_src<P>(&self, html: &str, link: &str, image_processor: &P) -> Result<String>
    where
        P: ImageProcessor + ?Sized,
    {
        // Regex to find <img> tags and capture the src attribute
        let mut fixed_html = String::new();
        let mut last_pos = 0;

        // Iterate over each match
        for cap in self.img_tag_regex.captures_iter(html) {
            // Append the text before the current match
            let mat = cap.get(0).ok_or_else(|| {
                HtmlProcessorError::Unexpected(anyhow::anyhow!(
                    "could not extract information from matched tag"
                ))
            })?;
            fixed_html.push_str(&html[last_pos..mat.start()]);

            // Get the original <img> tag and src value
            let full_tag = &cap[0];
            let src_value = &cap[1];

            if src_value.starts_with("data:image") || src_value.starts_with("/data:image") {
                fixed_html.push_str(full_tag);
                last_pos = mat.end();
                continue;
            }

            let image_url = if src_value.starts_with("http://") || src_value.starts_with("https://")
            {
                src_value.to_owned()
            } else {
                match (src_value.starts_with("/"), link.ends_with("/")) {
                    (true, true) => format!("{}{}", link, &src_value[1..]),
                    (true, false) | (false, true) => format!("{}{}", link, src_value),
                    (false, false) => format!("{}/{}", link, src_value),
                }
            };

            let processed_image = image_processor.process_image_url(&image_url).await;

            let image_path = match &processed_image {
                Ok(path) => path,
                Err(e) => {
                    tracing::error!("unable to process image: {e:?}");
                    // TODO: use a self property
                    "/static/error_processing_image.png"
                }
            };

            let corrected_tag = full_tag.replace(src_value, image_path);

            // Append the corrected <img> tag
            fixed_html.push_str(&corrected_tag);

            // Update the position after the current match
            last_pos = mat.end();
        }

        // Append the remaining part of the HTML
        fixed_html.push_str(&html[last_pos..]);

        Ok(fixed_html)
    }

    fn sanitize(&self, html: &str) -> Result<String> {
        // Step 1: Remove harmful tags like <script> or <iframe>
        let sanitized_html = self.tag_removal_regex.replace_all(html, "");

        // Step 2: Remove potentially harmful attributes
        let sanitized_html = self.attr_removal_regex.replace_all(&sanitized_html, "");

        // Return the sanitized HTML as a String
        Ok(sanitized_html.to_string())
    }

    // Function to extract favicon URL from the HTML
    fn get_favicon_url(&self, html: &str) -> Option<String> {
        if let Some(captures) = self.favicon_url.captures(html) {
            if let Some(href) = captures.get(1) {
                return Some(href.as_str().to_string());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_iframe() {
        let iframe = r#"
<html>
<body>
<iframe class="frame" title="Comments" scrolling="no" src="https://iframe.com" loading="lazy"></iframe>
</body>
</html>
"#;

        let expected = r#"
<html>
<body>

</body>
</html>
"#;
        let result = HtmlProcessorImpl::new().unwrap().sanitize(iframe);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn remove_scripts() {
        let iframe = r#"
<html>
<head>
<script defer="" src="some_script.js" nonce=""></script>
<script nonce>function hello_world() { console.log("hello, world!"); }</script>
</head>
<body>
</body>
</html>
"#;

        let expected = r#"
<html>
<head>


</head>
<body>
</body>
</html>
"#;
        let result = HtmlProcessorImpl::new().unwrap().sanitize(iframe);

        assert_eq!(expected, result.unwrap());
    }
}
