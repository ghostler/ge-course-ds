use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use crate::{component, context};
use crate::resource::TextureManager;

pub struct GameObject {
    transform: Option<component::Transform>,
    sprite: Option<component::Sprite>,
    player_controller: Option<component::PlayerController>,
}

impl GameObject {
    pub fn new(
        transform: Option<component::Transform>,
        sprite: Option<component::Sprite>,
        player_controller: Option<component::PlayerController>,
    ) -> Self {
        Self {
            transform,
            sprite,
            player_controller,
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

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture_manager: &TextureManager) {
        if let Some(ref transform) = self.transform {
            if let Some(ref sprite) = self.sprite {
                sprite.draw(d, texture_manager, transform.position());
            }
        }
    }
}
