extern crate bevy;
extern crate bevy_mod_debugdump;
extern crate hrh_game;

use bevy::prelude::CoreSchedule;

use hrh_game::HrhGame;

fn main() {
    let mut app = HrhGame::create_bevy_app();
    let dot = bevy_mod_debugdump::schedule_graph_dot(
        &mut app,
        CoreSchedule::Main,
        &bevy_mod_debugdump::schedule_graph::Settings {
            prettify_system_names: false,
            ..bevy_mod_debugdump::schedule_graph::Settings::default()
        },
    );
    println!("{dot}");
}
