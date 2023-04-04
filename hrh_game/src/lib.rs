extern crate bevy;
extern crate hrh_base;

use bevy::math::{uvec2, UVec2};

use hrh_base::{HrhBase, HrhBaseConfig};

const TILE_SIZE: UVec2 = uvec2(16, 16);
const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(20, 12);
const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(12, 18);
const INITIAL_CANVAS_ZOOM: u32 = 3;

pub fn new_hrh_game() -> bevy::prelude::App {
    HrhBase::new(HrhBaseConfig {
        title: "Hen Rescue Hero".to_string(),
        landscape_canvas_size: CANVAS_TILES_LANDSCAPE * TILE_SIZE,
        portrait_canvas_size: CANVAS_TILES_PORTRAIT * TILE_SIZE,
        initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
    })
    .create_bevy_app()
}
