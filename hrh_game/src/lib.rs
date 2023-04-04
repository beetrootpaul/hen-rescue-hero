extern crate bevy;
extern crate brp_game_base;

use bevy::math::{uvec2, UVec2};
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue, BrpGameBase, BrpGameConfig, BrpSystemSet};
use pico8_color::Pico8Color;

mod pico8_color;

const GAME_TITLE: &str = "Hen Rescue Hero";
const TILE_SIZE: UVec2 = uvec2(16, 16);
const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(20, 12);
const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(12, 18);
const INITIAL_CANVAS_ZOOM: u32 = 3;

pub struct HrhGame {}

impl HrhGame {
    pub fn create_bevy_app() -> App {
        let mut app = BrpGameBase::new(BrpGameConfig {
            title: GAME_TITLE.to_string(),
            landscape_canvas_size: CANVAS_TILES_LANDSCAPE * TILE_SIZE,
            portrait_canvas_size: CANVAS_TILES_PORTRAIT * TILE_SIZE,
            initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
        })
        .create_bevy_app();

        app.add_system(Self::draw_bg.in_set(BrpSystemSet::Draw));

        app
    }

    fn draw_bg(mut draw_queue: ResMut<BrpDrawQueue>) {
        draw_queue.enqueue(BrpDrawCommand::Clear(Pico8Color::Blue.into()));
    }
}
