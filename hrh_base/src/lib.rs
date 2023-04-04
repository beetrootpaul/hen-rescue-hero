extern crate bevy;
extern crate bevy_pixels;

use bevy::math::UVec2;
use bevy::prelude::IntoSystemConfig;

pub struct HrhBaseConfig {
    pub title: String,
    pub canvas_size: UVec2,
    pub initial_canvas_zoom: u32,
}

pub fn new_hrh_base_bevy_app(config: HrhBaseConfig) -> bevy::prelude::App {
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
                (config.canvas_size.x * config.initial_canvas_zoom) as f32,
                (config.canvas_size.y * config.initial_canvas_zoom) as f32,
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

    // https://crates.io/crates/bevy_pixels
    app.add_plugin(bevy_pixels::PixelsPlugin {
        primary_window: Some(bevy_pixels::PixelsOptions {
            width: config.canvas_size.x,
            height: config.canvas_size.y,
            auto_resize_surface: false,
            auto_resize_buffer: false,
            // value below is not used, because of auto_resize_buffer set to false above
            scale_factor: 1.0,
        }),
    });
    app.add_system(draw.in_set(bevy_pixels::PixelsSet::Draw));

    app
}

fn draw(mut wrapper_query: bevy::prelude::Query<&mut bevy_pixels::PixelsWrapper>) {
    for mut wrapper in &mut wrapper_query {
        let frame = wrapper.pixels.frame_mut();
        let f_len = frame.len();
        frame[0..(f_len / 2)]
            .copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff, 0x23, 0xe2, 0x78, 0xff].repeat(f_len / 16));
        frame[(f_len / 2)..f_len]
            .copy_from_slice(&[0xf5, 0xb2, 0x12, 0xff, 0xd4, 0xe2, 0x33, 0xff].repeat(f_len / 16));
    }
}
