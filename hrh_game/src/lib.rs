extern crate bevy;
extern crate brp_game_base;

use bevy::math::{uvec2, UVec2};
use bevy::prelude::*;

use brp_game_base::{BrpGameBase, BrpGameConfig, BrpPixelsSet, BrpPixelsWrapper};

const TILE_SIZE: UVec2 = uvec2(16, 16);
const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(20, 12);
const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(12, 18);
const INITIAL_CANVAS_ZOOM: u32 = 3;

pub struct HrhGame {}

impl HrhGame {
    pub fn create_bevy_app() -> App {
        let mut app = BrpGameBase::new(BrpGameConfig {
            title: "Hen Rescue Hero".to_string(),
            landscape_canvas_size: CANVAS_TILES_LANDSCAPE * TILE_SIZE,
            portrait_canvas_size: CANVAS_TILES_PORTRAIT * TILE_SIZE,
            initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
        })
        .create_bevy_app();

        app.add_system(draw_something.in_set(BrpPixelsSet::Draw));

        app
    }
}

// TODO: rename, cleanup, move to a separate module maybe
fn draw_something(
    // mut pixel_canvas: ResMut<PixelCanvas>,
    // game_area: Res<GameArea>,
    mut wrapper_query: Query<&mut BrpPixelsWrapper>,
) {
    // pixel_canvas.draw_ellipse_filled(
    //     game_area.game_area_rect_from(particle.bounding_rect()),
    //     Pico8Color::DarkGreen.into(),
    // );
    if let Ok(mut wrapper) = wrapper_query.get_single_mut() {
        let frame = wrapper.pixels.frame_mut();
        let f_len = frame.len();
        frame[0..(f_len / 2)]
            .copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff, 0x23, 0xe2, 0x78, 0xff].repeat(f_len / 16));
        frame[(f_len / 2)..f_len]
            .copy_from_slice(&[0xf5, 0xb2, 0x12, 0xff, 0xd4, 0xe2, 0x33, 0xff].repeat(f_len / 16));
    }
}
