mod add_new_feed;
mod add_new_feed_form;
mod get_article;
mod get_article_list;
mod get_feed_list;

pub use add_new_feed::add_new_feed;
pub use add_new_feed_form::add_new_feed_form;
use axum::response::IntoResponse;
pub use get_article::get_article;
pub use get_article_list::get_article_list;
pub use get_feed_list::get_feed_list;

pub(crate) struct ApiError {
    original_error: Box<dyn std::error::Error + 'static>,
    message: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", &self.original_error);
        axum::response::Html(format!("<h1> {} </h1>", self.message)).into_response()
    }
}

impl<E: std::error::Error + 'static> From<E> for ApiError {
    fn from(value: E) -> Self {
        Self {
            original_error: Box::new(value),
            message: "An unexpected error ocurred. Please check the logs.",
        }
    }
}
