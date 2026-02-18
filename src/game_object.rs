use crate::{component, context, resource};
use raylib::prelude::*;

pub struct GameObject {
    transform: Option<component::Transform>,
    sprite: Option<component::Sprite>,
    player_controller: Option<component::PlayerController>,
    position: Option<Vector2>,
    size: Option<Vector2>,
    color: Option<Color>,
}

impl GameObject {
    pub fn player(
        transform: component::Transform,
        sprite: component::Sprite,
        player_controller: component::PlayerController,
    ) -> Self {
        Self {
            transform: Some(transform),
            sprite: Some(sprite),
            player_controller: Some(player_controller),
            position: None,
            size: None,
            color: None,
        }
    }

    pub fn groud() -> Self {
        Self {
            transform: None,
            sprite: None,
            player_controller: None,
            position: Some(Vector2::new(-100000.0, -100000.0)),
            size: Some(Vector2::new(200000.0, 200000.0)),
            color: Some(Color::DARKBROWN),
        }
    }

    pub fn building(position: Vector2, size: Vector2, color: Color) -> Self {
        Self {
            transform: None,
            sprite: None,
            player_controller: None,
            position: Some(position),
            size: Some(size),
            color: Some(color),
        }
    }

    pub fn update(&mut self, game_ctx: &context::GameCtx) {
        if let Some(ref mut player_controller) = self.player_controller {
            if let Some(ref mut transform) = self.transform {
                player_controller.update(game_ctx, transform);
            }
        }
        if let Some(ref mut transform) = self.transform {
            transform.update(game_ctx.delta_time());
        }
    }

    pub fn is_player_controller(&self) -> bool {
        self.player_controller.is_some()
    }

    pub fn player_position(&self) -> Option<&Vector2> {
        self.transform.as_ref().and_then(|t| { Some(t.position()) })
    }


    pub fn draw(
        &self,
        d: &mut RaylibDrawHandle,
        texture_manager: &resource::TextureManager,
    ) {
        if let Some(ref transform) = self.transform {
            if let Some(ref sprite) = self.sprite {
                sprite.draw(d, texture_manager, transform.position());
            }
        }
        if let Some(ref pos) = self.position {
            if let Some(ref sprite) = self.sprite {
                sprite.draw(d, texture_manager, pos);
            }
            else if let Some (ref size) = self.size {
                let color = self.color.unwrap_or(Color::WHITE);
                d.draw_rectangle_v(pos, size, color);
            }
        }
    }
}
