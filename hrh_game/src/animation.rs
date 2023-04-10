use std::time::Duration;

use sprite::Sprite;

pub trait Animation {
    fn advance(&mut self, delta_time: Duration);
    fn current_sprite(&self) -> Sprite;
}
