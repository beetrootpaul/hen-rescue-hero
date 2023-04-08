extern crate bevy;
extern crate brp_game_base;
extern crate rand;

use bevy::prelude::*;

use brp_game_base::{BrpGameBase, BrpGameConfig, BrpGameState, BrpImageAssets, BrpSystemSet};
use canvas::{Canvas, CanvasEcs};
use chicken::ChickenEcs;
use images::Images;
use input::KeyboardControlsEcs;
use pico8_color::Pico8Color;
use rail::RailEcs;
use robot::RobotEcs;

const GAME_TITLE: &str = "Hen Rescue Hero";

#[cfg(not(target_arch = "wasm32"))]
const INITIAL_CANVAS_ZOOM: u32 = 3;

#[cfg(target_arch = "wasm32")]
const HTML_CANVAS_SELECTOR: &str = "#hen_rescue_hero__canvas";

pub struct Game {}

impl Game {
    pub fn create_bevy_app() -> App {
        let mut app = BrpGameBase::new(BrpGameConfig {
            title: GAME_TITLE.to_string(),
            // Same color as the one used for background around HTML canvas in web build
            canvas_margin_color: Pico8Color::DarkBlue.into(),
            landscape_canvas_size: Canvas::CANVAS_SIZE_LANDSCAPE,
            portrait_canvas_size: Canvas::CANVAS_SIZE_PORTRAIT,
            #[cfg(not(target_arch = "wasm32"))]
            initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
            #[cfg(target_arch = "wasm32")]
            html_canvas_selector: HTML_CANVAS_SELECTOR.to_string(),
        })
        .create_bevy_app();

        // RESOURCES
        app.insert_resource(BrpImageAssets::from(Images));
        app.insert_resource(ChickenEcs::r_spawn_timer());

        // STARTUP systems
        app.add_startup_system(RobotEcs::ss_spawn);

        // UPDATE systems
        app.add_systems(
            (
                KeyboardControlsEcs::s_handle_keyboard_input,
                RobotEcs::s_update
                    .after(KeyboardControlsEcs::s_handle_keyboard_input)
                    .run_if(in_state(BrpGameState::InGame)),
                ChickenEcs::s_spawn.run_if(in_state(BrpGameState::InGame)),
                ChickenEcs::s_update.run_if(in_state(BrpGameState::InGame)),
            )
                .in_set(BrpSystemSet::Update),
        );

        // DRAW systems
        app.add_systems(
            (
                CanvasEcs::s_draw_bg.run_if(not(in_state(BrpGameState::Loading))),
                CanvasEcs::s_start_clipping_to_game_area
                    .run_if(not(in_state(BrpGameState::Loading))),
                RailEcs::s_draw.run_if(not(in_state(BrpGameState::Loading))),
                ChickenEcs::s_draw.run_if(not(in_state(BrpGameState::Loading))),
                RobotEcs::s_draw.run_if(not(in_state(BrpGameState::Loading))),
                CanvasEcs::s_end_clipping_to_game_area.run_if(not(in_state(BrpGameState::Loading))),
            )
                .chain()
                .in_set(BrpSystemSet::Draw),
        );

        app
    }
}
