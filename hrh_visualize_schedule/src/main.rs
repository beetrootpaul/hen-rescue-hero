extern crate hrh_game;

use hrh_game::HrhGame;

fn main() {
    let mut app = HrhGame::create_bevy_app();
    bevy_mod_debugdump::print_main_schedule(&mut app);
}
