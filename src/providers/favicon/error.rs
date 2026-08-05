// SPDX-FileCopyrightText: 2025-2026 Keheliya Gallaba
// SPDX-License-Identifier: AGPL-3.0-only
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FaviconProviderError {
    #[error("Request error: {0}")]
    RequestError(String),

    #[error("IO error: {0}")]
    IoError(String),
}
