use std::time::Duration;

use bevy::prelude::{Component, TimerMode};
use bevy::time::Timer;

use animation::Animation;
use sprite::Sprite;

#[derive(Component)]
pub struct MenuArrowButton {
    pub is_right: bool,
    timer: Timer,
}

impl MenuArrowButton {
    pub fn new(is_right: bool) -> Self {
        let mut timer = Timer::from_seconds(2.0, TimerMode::Repeating);
        if is_right {
            timer.tick(timer.duration() / 2);
        }
        Self { is_right, timer }
    }
}

impl Animation for MenuArrowButton {
    fn advance(&mut self, delta_time: Duration) {
        self.timer.tick(delta_time);
    }

    fn current_sprite(&self) -> Sprite {
        match self.timer.percent() {
            p if p < 0.75 => Sprite::LeftArrow,
            _ => Sprite::LeftArrowPressed,
        }
    }
}
