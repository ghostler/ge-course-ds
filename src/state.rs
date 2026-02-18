use crate::command;
use crate::resource;
use raylib::prelude::*;

pub trait State {
    fn update(&mut self, rl: &RaylibHandle, delta_time: f32) -> command::StateTransition;

    fn draw(&self, d: &mut RaylibDrawHandle, assets: &resource::Assets);

    fn draw_next(&self) -> bool {
        false
    }

    fn on_enter(&mut self);

    fn on_exit(&mut self);
}

pub struct StateStack {
    stack: Vec<Box<dyn State>>,
}

impl StateStack {
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            stack: vec![initial_state],
        }
    }

    pub fn has_state(&self) -> bool {
        !self.stack.is_empty()
    }

    fn push(&mut self, mut state: Box<dyn State>) {
        if let Some(current) = self.stack.last_mut() {
            current.on_exit();
        }
        state.on_enter();
        self.stack.push(state);
    }

    fn pop(&mut self) {
        if let Some(mut current) = self.stack.pop() {
            current.on_exit();
        }
        if let Some(current) = self.stack.last_mut() {
            current.on_enter();
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        if let Some(current) = self.stack.last_mut() {
            match current.update(rl, delta_time) {
                command::StateTransition::NoTransition => {}
                command::StateTransition::PushState(new_state) => {
                    self.push(new_state);
                }
                command::StateTransition::SwitchState(new_state) => {
                    current.on_exit();
                    *current = new_state;
                    current.on_enter();
                }
                command::StateTransition::PopState => {
                    self.pop();
                }
                command::StateTransition::Quit => {
                    self.stack.drain(0..).for_each(|mut s| s.on_exit());
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &resource::Assets) {
        if let Some(current) = self.stack.last() {
            if current.draw_next()
                && self.stack.len() > 1
                && let Some(next) = self.stack.get(self.stack.len() - 2)
            {
                next.draw(d, assets);
            }
            current.draw(d, assets);
        }
    }
}
