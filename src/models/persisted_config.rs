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
