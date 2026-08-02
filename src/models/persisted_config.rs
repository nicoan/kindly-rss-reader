// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PersistedConfig {
    pub dark_theme: bool,
    pub zoom: f64,
}

impl Default for PersistedConfig {
    fn default() -> Self {
        Self {
            dark_theme: false,
            zoom: 1.0,
        }
    }
}
