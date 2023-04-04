use bevy::prelude::*;

use drawing::BrpDrawingPlugin;
use game_config::BrpGameConfig;

pub struct BrpGameBase {
    config: BrpGameConfig,
}

impl BrpGameBase {
    pub fn new(config: BrpGameConfig) -> Self {
        Self { config }
    }

    pub fn create_bevy_app(&self) -> App {
        let mut app = App::new();

        app.insert_resource(self.config.clone());

        app.add_plugins(MinimalPlugins);

        app.add_plugin(bevy::log::LogPlugin::default());

        #[cfg(debug_assertions)]
        app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
            // https://bevy-cheatbook.github.io/cookbook/print-framerate.html
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

        app.add_plugin(WindowPlugin {
            primary_window: Some(Window {
                title: self.config.title.clone(),
                resolution: bevy::window::WindowResolution::new(
                    (self.config.landscape_canvas_size.x * self.config.initial_canvas_zoom) as f32,
                    (self.config.landscape_canvas_size.y * self.config.initial_canvas_zoom) as f32,
                ),
                ..default()
            }),
            ..default()
        });
        app.add_plugin(bevy::a11y::AccessibilityPlugin);
        app.add_plugin(bevy::winit::WinitPlugin::default());

        app.add_plugin(bevy::input::InputPlugin::default());

        #[cfg(debug_assertions)]
        app.add_system(bevy::window::close_on_esc);

        app.add_plugin(BrpDrawingPlugin {
            landscape_canvas_size: self.config.landscape_canvas_size,
            portrait_canvas_size: self.config.portrait_canvas_size,
        });

        app
    }
}
