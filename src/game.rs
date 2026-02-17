use crate::config;
use crate::resources;
use raylib::prelude::*;

const PLAYER_TEXTURE_NAME: &str = "player";

pub struct GameState {
    box_size: Vector2,
    box_pos: Vector2,
    velocity: Vector2,
    acceleration: f32,
    friction: f32,
    texture_manager: resources::TextureManager,
}

impl GameState {
    pub fn new(
        box_pos: Vector2,
        player_config: config::PlayerConfig,
        texture_manager: resources::TextureManager,
    ) -> Self {
        Self {
            box_size: Vector2 { x: 50.0, y: 50.0 },
            box_pos,
            velocity: Vector2::zero(),
            acceleration: player_config.acceleration,
            friction: player_config.friction,
            texture_manager,
        }
    }
    pub fn update(&mut self, delta_time: f32, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_W)
            || rl.is_key_down(KeyboardKey::KEY_S)
            || rl.is_key_down(KeyboardKey::KEY_A)
            || rl.is_key_down(KeyboardKey::KEY_D)
        {
            let mut axis_x = 0.0;
            if rl.is_key_down(KeyboardKey::KEY_A) {
                axis_x -= 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_D) {
                axis_x += 1.0;
            }
            self.velocity.x += axis_x * self.acceleration * delta_time;

            let mut axis_y = 0.0;
            if rl.is_key_down(KeyboardKey::KEY_W) {
                axis_y -= 1.0;
            }
            if rl.is_key_down(KeyboardKey::KEY_S) {
                axis_y += 1.0;
            }
            self.velocity.y += axis_y * self.acceleration * delta_time;
        } else {
            self.velocity *= self.friction;
        }
        self.box_pos.x += self.velocity.x * delta_time;
        self.box_pos.y += self.velocity.y * delta_time;
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        if let Some(player_texture) = self.texture_manager.texture(PLAYER_TEXTURE_NAME) {
            d.draw_texture_v(
                player_texture,
                self.box_pos,
                Color::WHITE,
            );
        } else {
            d.draw_rectangle_v(self.box_pos, self.box_size, Color::RED);
        }
    }
}
