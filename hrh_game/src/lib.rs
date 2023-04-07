extern crate bevy;
extern crate brp_game_base;

use bevy::math::{uvec2, UVec2};
use bevy::prelude::*;

use brp_game_base::{
    BrpDrawCommand, BrpDrawQueue, BrpGameBase, BrpGameConfig, BrpGameState, BrpImageAssets,
    BrpSystemSet,
};
use images::Images;
use pico8_color::Pico8Color;
use robot::RobotSystems;

mod images;
mod pico8_color;
mod robot;
mod sprites;

const GAME_TITLE: &str = "Hen Rescue Hero";
const TILE_SIZE: UVec2 = uvec2(16, 16);
const CANVAS_TILES_LANDSCAPE: UVec2 = uvec2(20, 12);
const CANVAS_TILES_PORTRAIT: UVec2 = uvec2(12, 18);
#[cfg(not(target_arch = "wasm32"))]
const INITIAL_CANVAS_ZOOM: u32 = 3;
#[cfg(target_arch = "wasm32")]
const HTML_CANVAS_SELECTOR: &str = "#hen_rescue_hero_canvas";

pub struct HrhGame {}

impl HrhGame {
    pub fn create_bevy_app() -> App {
        let mut app = BrpGameBase::new(BrpGameConfig {
            title: GAME_TITLE.to_string(),
            canvas_margin_color: Pico8Color::DarkBlue.into(),
            landscape_canvas_size: CANVAS_TILES_LANDSCAPE * TILE_SIZE,
            portrait_canvas_size: CANVAS_TILES_PORTRAIT * TILE_SIZE,
            #[cfg(not(target_arch = "wasm32"))]
            initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
            #[cfg(target_arch = "wasm32")]
            html_canvas_selector: HTML_CANVAS_SELECTOR.to_string(),
        })
        .create_bevy_app();

        app.insert_resource(BrpImageAssets::from(Images));

        app.add_systems(
            (
                Self::draw_bg,
                RobotSystems::draw.run_if(not(in_state(BrpGameState::Loading))),
            )
                .chain()
                .in_set(BrpSystemSet::Draw),
        );

        app
    }

    fn draw_bg(current_state: Res<State<BrpGameState>>, mut draw_queue: ResMut<BrpDrawQueue>) {
        let color = match *current_state {
            // Same color as the one used for background around HTML canvas in web build
            State(BrpGameState::Loading) => Pico8Color::DarkBlue,
            State(_) => Pico8Color::Blue,
        };
        draw_queue.enqueue(BrpDrawCommand::Clear(color.into()));
    }
}
