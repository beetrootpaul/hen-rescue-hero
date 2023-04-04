extern crate bevy;

pub struct HrhWindowConfig {
    pub title: String,
    pub logical_width: i32,
    pub logical_height: i32,
}

pub fn new_hrh_base_bevy_app(config: HrhWindowConfig) -> bevy::prelude::App {
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
            title: config.title,
            resolution: bevy::window::WindowResolution::new(
                config.logical_width as f32,
                config.logical_height as f32,
            ),
            ..bevy::utils::default()
        }),
        ..bevy::utils::default()
    });
    app.add_plugin(bevy::a11y::AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());

    app.add_plugin(bevy::input::InputPlugin::default());

    #[cfg(debug_assertions)]
    app.add_system(bevy::window::close_on_esc);

    app
}
