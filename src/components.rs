use crate::resources::TextureManager;
use raylib::prelude::*;

pub struct Transform {
    position: Vector2,
    velocity: Vector2,
    friction: f32,
}

impl Transform {
    pub fn new(position: Vector2, velocity: Vector2, friction: f32) -> Self {
        Self {
            position,
            velocity,
            friction,
        }
    }

    pub fn position(&self) -> &Vector2 { &self.position }

    pub fn update(&mut self, delta_time: f32) {
        self.velocity *= self.friction;
        self.position += self.velocity * delta_time;
    }
}

pub struct Sprite {
    texture_name: String,
    color: Color,
}

impl Sprite {
    pub fn new(texture_name: String, color: Color) -> Self {
        Self {
            texture_name,
            color,
        }
    }

    pub fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        texture_manager: &TextureManager,
        position: &Vector2,
    ) {
        if let Some(texture) = texture_manager.texture(self.texture_name.as_str()) {
            d.draw_texture_v(texture, position, &self.color);
        }
    }
}

pub struct PlayerController {
    acceleration: f32,
}

impl PlayerController {
    pub fn new(acceleration: f32) -> Self {
        Self { acceleration }
    }

    pub fn update(&mut self, rl: &RaylibHandle, transform: &mut Transform, delta_time: f32) {
        let mut axis_x: f32 = 0.0;
        let mut axis_y: f32 = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_W) {
            axis_y -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            axis_y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            axis_x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            axis_x += 1.0;
        }

        // нормализация для диагоналей (чтобы скорость не была выше по диагонали)
        if axis_x != 0.0 && axis_y != 0.0 {
            let len = (axis_x * axis_x + axis_y * axis_y).sqrt();
            axis_x /= len;
            axis_y /= len;
        }

        transform.velocity.x += axis_x * self.acceleration * delta_time;
        transform.velocity.y += axis_y * self.acceleration * delta_time;
    }
}
