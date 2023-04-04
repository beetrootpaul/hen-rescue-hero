extern crate bevy;
extern crate hrh_base;

use std::ops::Mul;

use bevy::math::{ivec2, uvec2, IVec2, UVec2};

use hrh_base::{new_hrh_base_bevy_app, HrhBaseConfig};

const TILE_SIZE: UVec2 = uvec2(16, 16);
const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(20, 12);
const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(12, 18);
const INITIAL_CANVAS_ZOOM: u32 = 3;

pub fn new_hrh_game() -> bevy::prelude::App {
    new_hrh_base_bevy_app(HrhBaseConfig {
        title: "Hen Rescue Hero".to_string(),
        canvas_size: CANVAS_TILES_LANDSCAPE * TILE_SIZE,
        initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
    })
}
