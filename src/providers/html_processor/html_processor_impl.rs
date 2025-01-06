//! TODO: Implement a proper parser!!! ATM we are not sanitizing anything
use std::path::Path;

use axum::async_trait;
use regex::Regex;
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use super::HtmlProcessor;

#[derive(Clone)]
pub struct HtmlProcessorImpl;

impl HtmlProcessorImpl {}

#[async_trait]
impl HtmlProcessor for HtmlProcessorImpl {
    fn process_html_article(html: &str) -> Result<String, Box<dyn std::error::Error>> {
        // let start_tag = "<main";
        let start_tag = Regex::new(r"(?i)<\s*main[^>]*>").unwrap();
        let end_tag = "</main>";

        let start_tag = start_tag.find(html).ok_or("unable to find <main> tag")?;

        // Locate the start and end positions of the <article> content
        if let Some(end_idx) = html.find(end_tag) {
            // Extract the content between the <main> tags
            let start = start_tag.end();
            let article_content = &html[start..end_idx];
            return Ok(article_content.to_string());
        }
        Ok("".to_owned())
    }

    // TODO: We need to make this flexible in case we don't want to save the images to the fs
    async fn fix_img_src(html: &str, link: &str, article_path: impl AsRef<Path> + Send) -> String {
        let path = article_path.as_ref().join("static/");
        fs::create_dir_all(&path)
            .await
            .expect("failed to create static dir");

        // Regex to find <img> tags and capture the src attribute
        let img_tag_regex = Regex::new(r#"<img[^>]*\bsrc\s*=\s*['"]([^'"]+)['"][^>]*>"#).unwrap();
        let mut fixed_html = String::new();
        let mut last_pos = 0;

        // Iterate over each match
        for cap in img_tag_regex.captures_iter(html) {
            // Append the text before the current match
            let mat = cap.get(0).unwrap();
            fixed_html.push_str(&html[last_pos..mat.start()]);

            // Get the original <img> tag and src value
            let full_tag = &cap[0];
            let src_value = &cap[1];

            let image_url = if src_value.starts_with("http://") || src_value.starts_with("https://")
            {
                src_value.to_owned()
            } else {
                // Prepend the base link to the src value
                format!("{}{}", link, src_value)
            };

            // TODO: If we can't get the image replace it with a placeholder
            let image_data = reqwest::get(image_url)
                .await
                .expect("unable to get content url")
                .bytes()
                .await
                .expect("unable to get content from content url");

            let mut image_path = path.clone();

            image_path.push(Uuid::new_v4().to_string());

            // Create the file and write the content
            let mut file = fs::File::create(&image_path)
                .await
                .expect("unable to create image file");

            fs::File::write(&mut file, &image_data)
                .await
                .expect("unable to write image file");

            let corrected_tag = full_tag.replace(src_value, &format!("/{}", image_path.display())); // Replace the src value

            // Append the corrected <img> tag
            fixed_html.push_str(&corrected_tag);

            // Update the position after the current match
            last_pos = mat.end();
        }

        // Append the remaining part of the HTML
        fixed_html.push_str(&html[last_pos..]);

        fixed_html
    }

    fn sanitize(html: &str) -> String {
        // Regex to remove script and iframe tags entirely
        let tag_removal_regex =
            Regex::new(r"(?si)<iframe.*?(</iframe>|/>)|<script.*?(</script>|/>)").unwrap();

        // Regex to remove potentially harmful attributes
        let attr_removal_regex =
            Regex::new(r#"(?i)\b(on\w+|javascript:|data:)[^"'<>]*=['"][^"']*['"]"#).unwrap();

        // Step 1: Remove harmful tags like <script> or <iframe>
        let sanitized_html = tag_removal_regex.replace_all(html, "");

        // Step 2: Remove potentially harmful attributes
        let sanitized_html = attr_removal_regex.replace_all(&sanitized_html, "");

        // Return the sanitized HTML as a String
        sanitized_html.to_string()
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
        let result = HtmlProcessorImpl::sanitize(iframe);

        assert_eq!(expected, result);
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
        let result = HtmlProcessorImpl::sanitize(iframe);

        assert_eq!(expected, result);
    }
}
