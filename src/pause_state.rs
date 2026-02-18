use crate::{command, resource, state};
use raylib::prelude::*;

pub struct PauseState;

impl PauseState {
    pub fn new() -> Self {
        Self {}
    }
}

impl state::State for PauseState {
    fn update(&mut self, rl: &RaylibHandle, _: f32) -> command::StateTransition {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            command::StateTransition::PopState
        } else {
            command::StateTransition::NoTransition
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle, _: &resource::Assets) {
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            d.get_screen_height(),
            Color::new(255, 255, 255, 15),
        );
        d.draw_text(
            "PAUSE",
            d.get_screen_width() / 2 - 100,
            d.get_screen_height() / 2 - 15,
            30,
            Color::LIGHTYELLOW,
        );
    }

    fn draw_next(&self) -> bool {
        true
    }

    fn on_enter(&mut self) {}

    fn on_exit(&mut self) {}
}
