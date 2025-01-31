pub mod config;
pub mod feed;
pub mod not_found;

use axum::response::{Html, IntoResponse};
use reqwest::{header, StatusCode};

pub(crate) struct ApiError {
    original_error: Box<dyn std::error::Error + 'static>,
    message: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{}", &self.original_error);
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

/// This response is used to render a template. It includes the needed headers as well (for example
/// the ones to not save cache)
pub(crate) struct HtmlResponse(String);

impl HtmlResponse {
    pub fn new(rendered_html: String) -> Self {
        Self(rendered_html)
    }
}

impl IntoResponse for HtmlResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::OK,
            [
                (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                (
                    header::CACHE_CONTROL,
                    "no-store, no-cache, must-revalidate, max-age=0",
                ),
                (header::PRAGMA, "no-cache"),
                (header::EXPIRES, "0"),
            ],
            Html(self.0),
        )
            .into_response()
    }
}
