#[derive(Debug, thiserror::Error)]
pub enum HtmlProcessorError {
    #[error("unable to parse html, this usually means the article have no <main> and/or <article> tag(s)")]
    UnableToParse,

    #[error("unexpected error ocurred: {0:?}")]
    Unexpected(#[source] anyhow::Error),
}
