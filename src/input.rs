use raylib::prelude::*;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, serde::Deserialize)]
pub struct ActionConfig {
    pub keys: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct InputConfig {
    pub actions: HashMap<String, ActionConfig>,
}

impl InputConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conf = fs::read_to_string(path)?;
        Ok(toml::from_str(&conf)?)
    }
}

pub struct InputMap {
    config: InputConfig,
}

impl InputMap {
    pub fn new(config: InputConfig) -> Self {
        Self { config }
    }

    pub fn action_pressed(&self, rl: &RaylibHandle, action: &str) -> bool {
        if let Some(action) = self.config.actions.get(action) {
            if let Some(keys) = &action.keys {
                for key in keys {
                    if let Some(key) = parse_key(key) {
                        if rl.is_key_pressed(key) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn action_active(&self, rl: &RaylibHandle, action: &str) -> bool {
        if let Some(action) = self.config.actions.get(action) {
            if let Some(keys) = &action.keys {
                for key in keys {
                    if let Some(key) = parse_key(key) {
                        if rl.is_key_down(key) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

fn parse_key(key: &str) -> Option<KeyboardKey> {
    match key {
        "W" => Some(KeyboardKey::KEY_W),
        "A" => Some(KeyboardKey::KEY_A),
        "S" => Some(KeyboardKey::KEY_S),
        "D" => Some(KeyboardKey::KEY_D),
        "Space" | "Spc" => Some(KeyboardKey::KEY_SPACE),
        "Escape" | "Esc" => Some(KeyboardKey::KEY_ESCAPE),
        "Enter" | "Ent" => Some(KeyboardKey::KEY_ENTER),
        _ => None,
    }
}
