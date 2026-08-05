// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    message: &'static str,
    error_image_path: &'static str,
}

impl Error {
    pub fn not_found() -> Self {
        Self {
            message: "Page not found.",
            error_image_path: "/static/images/not_found.webp",
        }
    }

    pub fn internal_error() -> Self {
        Self {
            message: "An unexpected error ocurred. Please check the logs.",
            error_image_path: "/static/images/internal_error.webp",
        }
    }

    pub fn bad_request() -> Self {
        Self {
            message: "Bad request. Please check the url.",
            error_image_path: "/static/images/bad_request.webp",
        }
    }
}
