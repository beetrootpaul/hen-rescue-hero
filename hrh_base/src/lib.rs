extern crate bevy;
extern crate bevy_pixels;

use bevy::math::UVec2;
use bevy::prelude::*;
use bevy::window::{WindowBackendScaleFactorChanged, WindowResized};

#[derive(Clone)]
pub struct HrhBaseConfig {
    pub title: String,
    pub landscape_canvas_size: UVec2,
    pub portrait_canvas_size: UVec2,
    pub initial_canvas_zoom: u32,
}

pub struct HrhBase {
    config: HrhBaseConfig,
}

// TODO: rename, cleanup, move to a separate module maybe
#[derive(Resource)]
struct HrhBaseConfigRes(HrhBaseConfig);

impl HrhBase {
    pub fn new(config: HrhBaseConfig) -> Self {
        Self { config }
    }

    pub fn create_bevy_app(&self) -> App {
        let mut app = App::new();

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

        // https://crates.io/crates/bevy_pixels
        app.add_plugin(bevy_pixels::PixelsPlugin {
            primary_window: Some(bevy_pixels::PixelsOptions {
                width: self.config.landscape_canvas_size.x,
                height: self.config.landscape_canvas_size.y,
                // has to set both values below to false, because we do custom resizing on our own
                auto_resize_surface: false,
                auto_resize_buffer: false,
                // value below is not used, because of auto_resize_buffer set to false above
                scale_factor: 1.0,
            }),
        });
        app.insert_resource(HrhBaseConfigRes(self.config.clone()));
        app.add_systems(
            (
                Self::window_change,
                Self::window_resize,
                Self::resize_buffer.after(Self::window_resize),
            )
                .in_base_set(CoreSet::PreUpdate),
        );
        app.add_system(Self::draw.in_set(bevy_pixels::PixelsSet::Draw));

        app
    }

    // TODO: test if it works at all (e.g. by changed DPI in web build)
    // TODO: rename, cleanup, move to a separate module maybe
    fn window_change(
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut query: Query<(
            &mut bevy_pixels::PixelsWrapper,
            &bevy_pixels::PixelsOptions,
            &Window,
        )>,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            println!("WINDOW_CHANGE");
            if let Ok((mut wrapper, options, window)) = query.get_mut(event.window) {
                Self::resize_surface_to_window(&mut wrapper, window);
            }
        }
    }

    // TODO: rename, cleanup, move to a separate module maybe
    fn window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        mut query: Query<(
            &mut bevy_pixels::PixelsWrapper,
            &mut bevy_pixels::PixelsOptions,
            &Window,
        )>,
        hrh_base_config: Res<HrhBaseConfigRes>,
    ) {
        for event in window_resized_events.iter() {
            if let Ok((mut wrapper, mut options, window)) = query.get_mut(event.window) {
                let win_w = window.width();
                let win_h = window.height();

                let ratio = win_w / win_h;
                let new_size = if ratio > 1.0 {
                    hrh_base_config.0.landscape_canvas_size
                } else {
                    hrh_base_config.0.portrait_canvas_size
                };

                // mutate options only when really needed, in order to not trigger `Changed<bevy_pixels::PixelsOptions>` too often
                if options.width != new_size.x || options.height != new_size.y {
                    options.width = new_size.x;
                    options.height = new_size.y;
                }

                Self::resize_surface_to_window(&mut wrapper, window);
            }
        }
    }

    // TODO: rename, cleanup, move to a separate module maybe
    fn resize_buffer(
        mut query: Query<
            (&mut bevy_pixels::PixelsWrapper, &bevy_pixels::PixelsOptions),
            Changed<bevy_pixels::PixelsOptions>,
        >,
    ) {
        for (mut wrapper, options) in &mut query {
            println!("RESIZE_BUFFER");
            let _ = wrapper.pixels.resize_buffer(options.width, options.height);
        }
    }

    // TODO: rename, cleanup, move to a separate module maybe
    fn resize_surface_to_window(wrapper: &mut bevy_pixels::PixelsWrapper, window: &Window) {
        let _ = wrapper
            .pixels
            .resize_surface(window.physical_width(), window.physical_height());
    }

    // TODO: rename, cleanup, move to a separate module maybe
    fn draw(mut wrapper_query: bevy::prelude::Query<&mut bevy_pixels::PixelsWrapper>) {
        for mut wrapper in &mut wrapper_query {
            let frame = wrapper.pixels.frame_mut();
            let f_len = frame.len();
            frame[0..(f_len / 2)].copy_from_slice(
                &[0x48, 0xb2, 0xe8, 0xff, 0x23, 0xe2, 0x78, 0xff].repeat(f_len / 16),
            );
            frame[(f_len / 2)..f_len].copy_from_slice(
                &[0xf5, 0xb2, 0x12, 0xff, 0xd4, 0xe2, 0x33, 0xff].repeat(f_len / 16),
            );
        }
    }
}
