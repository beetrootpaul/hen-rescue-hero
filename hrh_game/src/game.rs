extern crate bevy;
extern crate brp_game_base;
extern crate rand;

use bevy::prelude::*;

use brp_game_base::{
    BrpGameBase, BrpGameConfig, BrpGameState, BrpGameStateEcs, BrpImageAssets, BrpSystemSet,
};
use canvas::{Canvas, CanvasEcs};
#[cfg(debug_assertions)]
use collider::ColliderEcs;
use font::FontEcs;
use game_objects::chicken::ChickenEcs;
use game_objects::countdown::CountdownEcs;
use game_objects::nest::NestEcs;
use game_objects::pile_of_chickens::PileOfChickensEcs;
use game_objects::rail::RailEcs;
use game_objects::robot::RobotEcs;
use game_objects::score::ScoreEcs;
use game_objects::side::SideEcs;
use images::Images;
use input::{ControlsEcs, InputEcs};
use logic::chickens_go_to_nest::ChickensGoToNestEcs;
use logic::overheating::OverheatingEcs;
use logic::robot_catches_chickens::RobotCachesChickensEcs;
use menu::MenuEcs;
use pico8_color::Pico8Color;

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
            square_canvas_size: Canvas::CANVAS_SIZE_SQUARE,
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
        app.insert_resource(FontEcs::r_font_config());
        app.insert_resource(InputEcs::r_input_mode());
        app.insert_resource(MenuEcs::r_menu_mode());
        app.insert_resource(ChickenEcs::r_spawn_timer());
        #[cfg(debug_assertions)]
        app.insert_resource(ColliderEcs::r_debug_config());
        app.insert_resource(ScoreEcs::r_score());
        app.insert_resource(CountdownEcs::r_countdown());

        // STARTUP systems
        app.add_startup_system(RobotEcs::s_spawn);
        app.add_startup_system(NestEcs::s_spawn);

        // ENTER systems
        app.add_system(MenuEcs::s_enter_menu.in_schedule(OnEnter(BrpGameState::Menu)));
        app.add_systems(
            (
                CountdownEcs::s_reset,
                ScoreEcs::s_reset,
                RobotEcs::s_reset,
                ChickenEcs::s_reset,
            )
                .in_schedule(OnExit(BrpGameState::Menu)),
        );

        // EXIT systems
        app.add_system(MenuEcs::s_exit_menu.in_schedule(OnExit(BrpGameState::Menu)));

        // UPDATE systems
        app.add_systems(
            (
                InputEcs::s_update,
                ControlsEcs::s_handle_input.after(InputEcs::s_update),
            )
                .in_set(BrpSystemSet::Update),
        );
        app.add_system(
            MenuEcs::s_update
                .in_set(BrpSystemSet::Update)
                .run_if(BrpGameStateEcs::c_is_in_menu),
        );
        app.add_systems(
            (
                CountdownEcs::s_update,
                RobotEcs::s_update.after(ControlsEcs::s_handle_input),
                ChickenEcs::s_spawn,
                ChickenEcs::s_update,
                RobotCachesChickensEcs::s_perform.after(ChickenEcs::s_update),
                ChickensGoToNestEcs::s_perform.after(RobotCachesChickensEcs::s_perform),
                OverheatingEcs::s_advance_timer.after(ChickensGoToNestEcs::s_perform),
            )
                .in_set(BrpSystemSet::Update)
                .distributive_run_if(in_state(BrpGameState::InGame)),
        );

        // DRAW systems
        app.add_systems(
            (
                CanvasEcs::s_draw_bg,
                //
                CanvasEcs::s_start_clipping_to_game_area,
                OverheatingEcs::s_draw,
                RailEcs::s_draw,
                SideEcs::s_draw,
                NestEcs::s_draw,
                RobotEcs::s_draw,
                PileOfChickensEcs::s_draw,
                ChickenEcs::s_draw,
                CanvasEcs::s_end_clipping_to_game_area,
                //
                ScoreEcs::s_draw,
                CountdownEcs::s_draw,
                //
                #[cfg(debug_assertions)]
                ColliderEcs::s_debug_draw_colliders.run_if(ColliderEcs::c_is_debug_draw_enabled),
                //
                MenuEcs::s_draw.run_if(BrpGameStateEcs::c_is_in_menu),
            )
                .chain()
                .in_set(BrpSystemSet::Draw)
                .distributive_run_if(BrpGameStateEcs::c_is_game_loaded),
        );

        app
    }
}
