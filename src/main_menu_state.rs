use crate::{command, context, resource, state};
use raylib::prelude::*;

pub struct MainMenuState<F> {
    make_game_play_state: F,
}

impl<F> MainMenuState<F> {
    pub fn new(make_game_play_state: F) -> Self {
        Self {
            make_game_play_state,
        }
    }
}

impl<F> state::State for MainMenuState<F>
where
    F: Fn() -> Box<dyn state::State>,
{
    fn update(&mut self, game_ctx: &context::GameCtx) -> command::StateTransition {
        if game_ctx.rl().is_key_pressed(KeyboardKey::KEY_ENTER) {
            command::StateTransition::PushState((self.make_game_play_state)())
        } else if game_ctx.rl().is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            command::StateTransition::PopState
        } else {
            command::StateTransition::NoTransition
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, _: &resource::Assets) {
        d.draw_text(
            "Press Enter to Start Game...",
            d.get_screen_width() / 2 - 220,
            d.get_screen_height() / 2 - 30,
            30,
            Color::LAWNGREEN,
        );
    }

    fn on_enter(&mut self) {}

    fn on_exit(&mut self) {}
}
