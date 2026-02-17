use crate::{components, config};
use crate::resources;
use raylib::prelude::*;
use crate::game_object;

const PLAYER_TEXTURE_NAME: &str = "player";

pub struct GameState {
    game_objects: Vec<game_object::GameObject>,
    texture_manager: resources::TextureManager,
}

impl GameState {
    pub fn new(
        box_pos: Vector2,
        player_config: config::PlayerConfig,
        texture_manager: resources::TextureManager,
    ) -> Self {
        let player_object = game_object::GameObject::new(
            Some(components::Transform::new(box_pos, Vector2::zero(), player_config.friction)),
            Some(components::Sprite::new(PLAYER_TEXTURE_NAME.to_string(), Color::WHITE)),
            Some(components::PlayerController::new(player_config.acceleration))
        );
        Self {
            game_objects: vec![player_object],
            texture_manager,
        }
    }
    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        for game_object in &mut self.game_objects {
            game_object.update(rl, delta_time);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for game_object in &self.game_objects {
            game_object.draw(d, &self.texture_manager);
        }
    }
}
