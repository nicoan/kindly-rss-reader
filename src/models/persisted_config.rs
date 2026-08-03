use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PersistedConfig {
    #[serde(default)]
    pub dark_theme: bool,

    #[serde(default = "default_zoom")]
    pub zoom: f64,

    #[serde(default)]
    pub toolbar_position_left: bool,
}

impl Default for PersistedConfig {
    fn default() -> Self {
        Self {
            dark_theme: false,
            zoom: 1.0,
            toolbar_position_left: false,
        }
    }
}

fn default_zoom() -> f64 {
    1.0_f64
}
