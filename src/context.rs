use raylib::prelude::*;
use crate::input;

pub struct GameCtx<'a> {
    rl: &'a mut RaylibHandle,
    thread: &'a RaylibThread,
    input_map: &'a input::InputMap,
    delta_time: f32,
}

impl<'a> GameCtx<'a> {
    pub fn new(rl: &'a mut RaylibHandle, thread: &'a RaylibThread, input_map: &'a input::InputMap) -> Self {
        Self { rl, thread, input_map, delta_time: 0.0 }
    }

    pub fn rl(&self) -> &RaylibHandle {
        self.rl
    }

    pub fn rl_mut(&mut self) -> &mut RaylibHandle {
        self.rl
    }

    pub fn input_map(&self) -> &input::InputMap {
        self.input_map
    }

    pub fn load_delta_time(&mut self) {
        self.delta_time = self.rl.get_frame_time();
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn begin_drawing(&'_ mut self) -> RaylibDrawHandle<'_>  {
        self.rl.begin_drawing(self.thread)
    }

    pub fn action_pressed(&self, action: &str) -> bool {
        self.input_map.action_pressed(self.rl, action)
    }

    pub fn action_active(&self, action: &str) -> bool {
        self.input_map.action_active(self.rl, action)
    }
}
