use bevy::app::App;
use bevy::prelude::Plugin;

pub fn unused_two() -> i32 {
    234
}

pub struct SomePlugin;

impl Plugin for SomePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ss_lol);
    }
}

fn ss_lol() {
    println!("!STARTUP SYSTEM HERE!")
}
