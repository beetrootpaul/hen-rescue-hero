extern crate hrh_base;

use hrh_base::{new_hrh_base_bevy_app, HrhWindowConfig};

const GAME_W: i32 = 560;
const GAME_H: i32 = 432;
const WINDOW_SCALE: i32 = 2;

pub fn new_hrh_game() -> bevy::prelude::App {
    new_hrh_base_bevy_app(HrhWindowConfig {
        title: "Hen Rescue Hero".to_string(),
        logical_width: GAME_W * WINDOW_SCALE,
        logical_height: GAME_H * WINDOW_SCALE,
    })
}
