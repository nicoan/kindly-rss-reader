use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    message: &'static str,
    error_image_path_dark: &'static str,
    error_image_path: &'static str,
}

impl Error {
    pub fn not_found() -> Self {
        Self {
            message: "Page not found.",
            error_image_path_dark: "/static/images/not_found_dark.webp",
            error_image_path: "/static/images/not_found.webp",
        }
    }

    pub fn internal_error() -> Self {
        Self {
            message: "An unexpected error ocurred. Please check the logs.",
            error_image_path_dark: "/static/images/internal_error_dark.webp",
            error_image_path: "/static/images/internal_error.webp",
        }
    }

    pub fn bad_request() -> Self {
        Self {
            message: "Bad request. Please check the url.",
            error_image_path_dark: "/static/images/bad_request_dark.webp",
            error_image_path: "/static/images/bad_request.webp",
        }
    }
}
