use bevy::prelude::*;

use brp_assets::BrpAssetEcs;
#[cfg(debug_assertions)]
use brp_debug::BrpDebugPausePlugin;
use brp_drawing::BrpDrawingPlugin;
use brp_font::BrpFontPlugin;
use brp_game_config::BrpGameConfig;
use brp_game_state::BrpGameState;
use brp_input::BrpInputPlugin;
use {BrpImageAssets, BrpSystemSet};

pub struct BrpGameBase {
    config: BrpGameConfig,
}

impl BrpGameBase {
    pub fn new(config: BrpGameConfig) -> Self {
        Self { config }
    }

    pub fn create_bevy_app(&self) -> App {
        let mut app = App::new();
        self.configure_bevy_for(&mut app);
        self.configure_own_for(&mut app);
        app
    }

    fn configure_bevy_for(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);

        app.add_plugin(bevy::log::LogPlugin::default());

        app.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default());
        #[cfg(debug_assertions)]
        app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
        #[cfg(debug_assertions)]
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

        app.add_plugin(WindowPlugin {
            primary_window: Some(Window {
                title: self.config.title.clone(),
                #[cfg(not(target_arch = "wasm32"))]
                resolution: bevy::window::WindowResolution::new(
                    (self.config.square_canvas_size.x * self.config.initial_canvas_zoom) as f32,
                    (self.config.square_canvas_size.y * self.config.initial_canvas_zoom) as f32,
                ),
                #[cfg(target_arch = "wasm32")]
                canvas: Some(self.config.html_canvas_selector.clone()),
                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                #[cfg(target_arch = "wasm32")]
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        });
        app.add_plugin(bevy::a11y::AccessibilityPlugin);
        app.add_plugin(bevy::winit::WinitPlugin::default());

        app.add_plugin(AssetPlugin {
            #[cfg(not(target_arch = "wasm32"))]
            asset_folder: "assets".to_string(),
            #[cfg(target_arch = "wasm32")]
            asset_folder: "".to_string(),
            ..AssetPlugin::default()
        });
        app.add_plugin(ImagePlugin::default());

        app.add_plugin(bevy::input::InputPlugin::default());
        app.add_plugin(bevy::gilrs::GilrsPlugin::default());

        #[cfg(debug_assertions)]
        app.add_system(bevy::window::close_on_esc);
    }

    fn configure_own_for(&self, app: &mut App) {
        app.insert_resource(self.config.clone());

        #[cfg(debug_assertions)]
        app.add_plugin(BrpDebugPausePlugin);

        app.add_plugin(BrpInputPlugin);

        app.add_plugin(BrpDrawingPlugin {
            canvas_margin_color: self.config.canvas_margin_color,
            square_canvas_size: self.config.square_canvas_size,
            landscape_canvas_size: self.config.landscape_canvas_size,
            portrait_canvas_size: self.config.portrait_canvas_size,
        });

        app.add_plugin(BrpFontPlugin);

        app.add_state::<BrpGameState>();

        app.init_resource::<BrpImageAssets>();
        app.add_startup_system(BrpAssetEcs::s_start_loading);
        app.add_system(
            BrpAssetEcs::s_wait_for_loading_to_complete
                .run_if(in_state(BrpGameState::Loading))
                .in_set(BrpSystemSet::Update),
        );
    }
}
