use std::time::Duration;

use bevy::prelude::{Res, ResMut, Resource, Timer, TimerMode};
use bevy::time::Time;

pub use crate::input::keyboard_controls::KeyboardControlsEcs;
pub use crate::input::touch_controls::TouchControlsEcs;

mod keyboard_controls;
mod touch_controls;

#[derive(Resource)]
pub struct InputMode {
    block_input_timer: Timer,
}

impl InputMode {
    pub fn block_input_for_a_moment(&mut self) {
        self.block_input_timer.reset()
    }
    pub fn is_input_blocked(&self) -> bool {
        !self.block_input_timer.finished()
    }
    fn update(&mut self, delta_time: Duration) {
        if self.is_input_blocked() {
            self.block_input_timer.tick(delta_time);
        }
    }
}

pub struct InputEcs;

impl InputEcs {
    pub fn r_input_mode() -> InputMode {
        let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        timer.tick(timer.remaining());
        InputMode {
            block_input_timer: timer,
        }
    }
    pub fn s_update(mut input_mode: ResMut<InputMode>, time: Res<Time>) {
        input_mode.update(time.delta());
    }
}
