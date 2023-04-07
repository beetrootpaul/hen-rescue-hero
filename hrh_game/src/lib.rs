extern crate bevy;
extern crate brp_game_base;

use bevy::prelude::*;

use brp_game_base::{BrpGameBase, BrpGameConfig, BrpGameState, BrpImageAssets, BrpSystemSet};
use canvas::{Canvas, CanvasSystems};
use chicken::{ChickenSpawnTimer, ChickenSystems};
use images::Images;
use input::KeyboardControlsSystems;
use pico8_color::Pico8Color;
use robot::RobotSystems;

mod canvas;
mod chicken;
mod images;
mod input;
mod pico8_color;
mod position;
mod robot;
mod sprites;

const GAME_TITLE: &str = "Hen Rescue Hero";

#[cfg(not(target_arch = "wasm32"))]
const INITIAL_CANVAS_ZOOM: u32 = 3;

#[cfg(target_arch = "wasm32")]
const HTML_CANVAS_SELECTOR: &str = "#hen_rescue_hero__canvas";

pub struct HrhGame {}

impl HrhGame {
    pub fn create_bevy_app() -> App {
        let mut app = BrpGameBase::new(BrpGameConfig {
            title: GAME_TITLE.to_string(),
            // Same color as the one used for background around HTML canvas in web build
            canvas_margin_color: Pico8Color::DarkBlue.into(),
            landscape_canvas_size: Canvas::canvas_size_landscape(),
            portrait_canvas_size: Canvas::canvas_size_portrait(),
            #[cfg(not(target_arch = "wasm32"))]
            initial_canvas_zoom: INITIAL_CANVAS_ZOOM,
            #[cfg(target_arch = "wasm32")]
            html_canvas_selector: HTML_CANVAS_SELECTOR.to_string(),
        })
        .create_bevy_app();

        // RESOURCES
        app.insert_resource(BrpImageAssets::from(Images));
        app.insert_resource(ChickenSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));

        // STARTUP systems
        app.add_startup_system(RobotSystems::spawn);

        // UPDATE systems
        app.add_systems(
            (
                KeyboardControlsSystems::handle_keyboard_input,
                RobotSystems::update
                    .after(KeyboardControlsSystems::handle_keyboard_input)
                    .run_if(in_state(BrpGameState::InGame)),
                ChickenSystems::spawn.run_if(in_state(BrpGameState::InGame)),
                ChickenSystems::update.run_if(in_state(BrpGameState::InGame)),
            )
                .in_set(BrpSystemSet::Update),
        );

        // DRAW systems
        app.add_systems(
            (
                CanvasSystems::draw_bg.run_if(not(in_state(BrpGameState::Loading))),
                ChickenSystems::draw.run_if(not(in_state(BrpGameState::Loading))),
                RobotSystems::draw.run_if(not(in_state(BrpGameState::Loading))),
            )
                .chain()
                .in_set(BrpSystemSet::Draw),
        );

        app
    }
}
