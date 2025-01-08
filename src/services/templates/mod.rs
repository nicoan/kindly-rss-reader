mod template_service_impl;

use axum::async_trait;
use serde::Serialize;
use std::path::Path;

pub use template_service_impl::TemplateServiceImpl;

pub const TEMPLATE_NAME_ARTICLE: &str = "article";
pub const TEMPLATE_NAME_ARTICLE_LIST: &str = "article_list";
pub const TEMPLATE_NAME_COMMON_HEAD: &str = "common_head";
pub const TEMPLATE_NAME_FEED_ADD: &str = "feed_add";
pub const TEMPLATE_NAME_FEED_LIST: &str = "feed_list";
pub const TEMPLATE_NAME_TOOLBAR: &str = "toolbar";
pub const TEMPLATE_PATH_ARTICLE: &str = "./templates/article.html";
pub const TEMPLATE_PATH_ARTICLE_LIST: &str = "./templates/article_list.html";
pub const TEMPLATE_PATH_COMMON_HEAD: &str = "./templates/common_head.html";
pub const TEMPLATE_PATH_FEED_ADD: &str = "./templates/feed_add.html";
pub const TEMPLATE_PATH_FEED_LIST: &str = "./templates/feed_list.html";
pub const TEMPLATE_PATH_TOOLBAR: &str = "./templates/toolbar.html";

#[async_trait]
pub(crate) trait TemplateService<'a>: Sync + Send {
    fn register_template(&mut self, name: &'a str, path: impl AsRef<Path>);

    fn render_template(&self, name: &str, context: impl Serialize) -> String;
}
