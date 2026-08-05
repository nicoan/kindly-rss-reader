// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use crate::repositories::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum PersistedConfigProviderError {
    #[error("a repository error ocurred: {0:?}")]
    Repository(#[from] RepositoryError),
}
