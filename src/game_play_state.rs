use crate::{context, game_object};
use crate::{command, component, config, resource, state};
use raylib::prelude::*;

const PLAYER_TEXTURE_NAME: &str = "player";

pub struct GamePlayState<F> {
    game_objects: Vec<game_object::GameObject>,
    camera2d: Camera2D,
    make_pause_state: F,
}

impl<F> GamePlayState<F> {
    pub fn new(screen_width: f32, screen_height: f32, player_config: config::PlayerConfig, make_pause_state: F) -> Self {
        let player_object = game_object::GameObject::player(
            component::Transform::new(
                Vector2::zero(),
                Vector2::zero(),
                player_config.friction,
            ),
            component::Sprite::new(
                PLAYER_TEXTURE_NAME.to_string(),
                Color::WHITE,
            ),
            component::PlayerController::new(player_config.acceleration),
        );
        let groud = game_object::GameObject::groud();
        let building1 = game_object::GameObject::building(
            Vector2::zero(),
            Vector2::new(100.0, 100.0),
            Color::WHITE,
        );
        let building2 = game_object::GameObject::building(
            Vector2::new(-100.0, -100.0),
            Vector2::new(50.0, 50.0),
            Color::BLANCHEDALMOND,
        );
        let building3 = game_object::GameObject::building(
            Vector2::new(-500.0, -500.0),
            Vector2::new(250.0, 250.0),
            Color::BLANCHEDALMOND,
        );
        let mut camera2d = Camera2D::default();
        camera2d.offset = Vector2::new(screen_width / 2.0, screen_height / 2.0);
        camera2d.zoom = 1.0;
        Self {
            game_objects: vec![groud, building3, building2, building1, player_object],
            camera2d,
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
            if game_object.is_player_controller() {
                if let Some(pos) = game_object.player_position() {
                    self.camera2d.target = pos.clone();
                }
            }
        }
        command::StateTransition::NoTransition
    }

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &resource::Assets) {
        {
            let mut d2d = d.begin_mode2D(self.camera2d);
            for game_object in &self.game_objects {
                game_object.draw(&mut d2d, assets.texture_manager());
            }
        }
        d.draw_text("UI", 50, 50, 20, Color::SILVER)
    }

    fn on_enter(&mut self) {}

    fn on_exit(&mut self) {}
}
