use std::fs;

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &'static str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub show_home_on_startup: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_home_on_startup: true,
        }
    }
}

impl Config {
    pub fn load() -> Config {
        fs::read_to_string(CONFIG_PATH)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(content) = serde_json::to_string(self) {
            fs::write(CONFIG_PATH, content).expect(&format!("saved Config to {}", CONFIG_PATH))
        }
    }
}
