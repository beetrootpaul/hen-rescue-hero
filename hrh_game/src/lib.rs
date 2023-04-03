extern crate hrh_base;
extern crate bevy;

use hrh_base::engine_world;

pub use abc::unused_one;

mod abc;

pub fn run_this_lib() {
    println!("Hello, {}!", engine_world());

    let mut app = bevy::app::App::new();

    app.add_plugins(bevy::MinimalPlugins);

    app.add_plugin(bevy::log::LogPlugin::default());

    #[cfg(debug_assertions)]
    app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

    app.add_plugin(bevy::window::WindowPlugin {
        primary_window: Some(bevy::window::Window {
            title: "Hen Rescue Hero".to_string(), // TODO: GAME_TITLE.to_string()
            #[cfg(not(target_arch = "wasm32"))]
            resolution: bevy::window::WindowResolution::new(1120.0, 864.0), // TODO: 560 432 x 2 from a package
            ..bevy::utils::default()
        }),
        ..bevy::utils::default()
    });
    app.add_plugin(bevy::a11y::AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());

    app.add_plugin(bevy::input::InputPlugin::default());

    #[cfg(debug_assertions)]
    app.add_system(bevy::window::close_on_esc);

    app.add_plugin(hrh_base::SomePlugin);

    app.run();
}
