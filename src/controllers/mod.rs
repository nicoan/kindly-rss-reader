pub mod config;
pub mod feed;
pub mod not_found;

use axum::response::{Html, IntoResponse};
use reqwest::{header, StatusCode};

pub(crate) struct ApiError {
    pub original_error: Box<dyn std::error::Error + 'static>,
    pub status_code: StatusCode,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{}", &self.original_error);
        (
            self.status_code,
            [
                (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                (
                    header::CACHE_CONTROL,
                    "no-store, no-cache, must-revalidate, max-age=0",
                ),
                (header::PRAGMA, "no-cache"),
                (header::EXPIRES, "0"),
            ],
        )
            .into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        Self {
            original_error: error.into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// This response is used to render a template. It includes the needed headers as well (for example
/// the ones to not save cache)
pub(crate) struct HtmlResponse {
    body: String,
    status_code: StatusCode,
}

impl HtmlResponse {
    pub fn new(rendered_html: String) -> Self {
        Self {
            body: rendered_html,
            status_code: StatusCode::OK,
        }
    }

    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
}

impl IntoResponse for HtmlResponse {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code,
            [
                (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                (
                    header::CACHE_CONTROL,
                    "no-store, no-cache, must-revalidate, max-age=0",
                ),
                (header::PRAGMA, "no-cache"),
                (header::EXPIRES, "0"),
            ],
            Html(self.body),
        )
            .into_response()
    }
}
