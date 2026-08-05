// SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
// SPDX-License-Identifier: AGPL-3.0-only
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PersistedConfig {
    #[serde(default)]
    pub dark_theme: bool,

    #[serde(default = "default_zoom")]
    pub zoom: f64,

    #[serde(default)]
    pub toolbar_position_left: bool,

    #[serde(default)]
    pub hide_article_header: bool,
}

impl Default for PersistedConfig {
    fn default() -> Self {
        Self {
            dark_theme: false,
            zoom: 1.0,
            toolbar_position_left: false,
            hide_article_header: false,
        }
    }
}

fn default_zoom() -> f64 {
    1.0_f64
}
