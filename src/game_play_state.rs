use crate::{context, game_object};
use crate::{command, component, config, resource, state};
use raylib::prelude::*;

const PLAYER_TEXTURE_NAME: &str = "player";

pub struct GamePlayState<F> {
    game_objects: Vec<game_object::GameObject>,
    make_pause_state: F,
}

impl<F> GamePlayState<F> {
    pub fn new(player_config: config::PlayerConfig, make_pause_state: F) -> Self {
        let player_object = game_object::GameObject::new(
            Some(component::Transform::new(
                Vector2::zero(),
                Vector2::zero(),
                player_config.friction,
            )),
            Some(component::Sprite::new(
                PLAYER_TEXTURE_NAME.to_string(),
                Color::WHITE,
            )),
            Some(component::PlayerController::new(player_config.acceleration)),
        );
        Self {
            game_objects: vec![player_object],
            make_pause_state,
        }
    }
}

impl<F> state::State for GamePlayState<F>
where
    F: Fn() -> Box<dyn state::State>,
{
    fn update(&mut self, game_ctx: &context::GameCtx) -> command::StateTransition {
        if game_ctx.action_pressed("pause") {
            return command::StateTransition::PushState((self.make_pause_state)());
        }
        for game_object in &mut self.game_objects {
            game_object.update(game_ctx);
        }
        command::StateTransition::NoTransition
    }

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &resource::Assets) {
        for game_object in &self.game_objects {
            game_object.draw(d, assets.texture_manager());
        }
    }

    fn on_enter(&mut self) {}

    fn on_exit(&mut self) {}
}
