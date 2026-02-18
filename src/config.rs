use std::fs;

#[derive(Debug, serde::Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GameConfig {
    pub target_fps: u32,
    pub show_fps: bool,
}

#[derive(Debug, Copy, Clone, serde::Deserialize)]
pub struct PlayerConfig {
    pub speed: f32,
    pub acceleration: f32,
    pub friction: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub game: GameConfig,
    pub player: PlayerConfig,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conf = fs::read_to_string(path)?;
        Ok(toml::from_str(&conf)?)
    }
}
