use bevy::prelude::*;
use bevy::window::{WindowBackendScaleFactorChanged, WindowResized};

use drawing::queue::BrpDrawQueue;
use game_config::BrpGameConfig;

pub struct BrpDrawingPlugin {
    pub landscape_canvas_size: UVec2,
    pub portrait_canvas_size: UVec2,
}

impl BrpDrawingPlugin {
    // TODO: test if it works at all (e.g. by changed DPI in web build)
    fn sys_update_pixels_on_window_scale_factor_changed(
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut query: Query<&mut bevy_pixels::PixelsWrapper, With<Window>>,
        winit_windows: NonSend<bevy::winit::WinitWindows>,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            if let Ok(mut wrapper) = query.get_mut(event.window) {
                if let Some(resized_winit_window) = winit_windows.get_window(event.window) {
                    Self::resize_pixels_surface_to_window(
                        &mut wrapper,
                        resized_winit_window.inner_size().width,
                        resized_winit_window.inner_size().height,
                    );
                }
            }
        }
    }

    fn sys_update_pixels_on_window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        mut query: Query<
            (
                &mut bevy_pixels::PixelsWrapper,
                &mut bevy_pixels::PixelsOptions,
            ),
            With<Window>,
        >,
        winit_windows: NonSend<bevy::winit::WinitWindows>,
        game_config: Res<BrpGameConfig>,
    ) {
        if let Some(event) = window_resized_events.iter().last() {
            if let Ok((mut wrapper, mut options)) = query.get_mut(event.window) {
                if let Some(resized_winit_window) = winit_windows.get_window(event.window) {
                    let window_w = resized_winit_window.inner_size().width;
                    let window_h = resized_winit_window.inner_size().height;

                    let new_canvas_size = if window_w > window_h {
                        game_config.landscape_canvas_size
                    } else {
                        game_config.portrait_canvas_size
                    };

                    // mutate options only when really needed, in order to not trigger `Changed<bevy_pixels::PixelsOptions>` too often
                    if options.width != new_canvas_size.x || options.height != new_canvas_size.y {
                        options.width = new_canvas_size.x;
                        options.height = new_canvas_size.y;
                    }

                    Self::resize_pixels_surface_to_window(&mut wrapper, window_w, window_h);
                }
            }
        }
    }

    fn resize_pixels_surface_to_window(
        wrapper: &mut bevy_pixels::PixelsWrapper,
        window_w: u32,
        window_h: u32,
    ) {
        let _ = wrapper.pixels.resize_surface(window_w, window_h);
    }

    fn sys_resize_pixels_buffer_if_needed(
        mut query: Query<
            (&mut bevy_pixels::PixelsWrapper, &bevy_pixels::PixelsOptions),
            Changed<bevy_pixels::PixelsOptions>,
        >,
    ) {
        for (mut pixels_wrapper, pixels_options) in &mut query {
            pixels_wrapper
                .pixels
                .resize_buffer(pixels_options.width, pixels_options.height)
                .expect("should resize pixels buffer");
        }
    }
}

impl Plugin for BrpDrawingPlugin {
    fn build(&self, app: &mut App) {
        // https://crates.io/crates/bevy_pixels
        app.add_plugin(bevy_pixels::PixelsPlugin {
            primary_window: Some(bevy_pixels::PixelsOptions {
                width: self.landscape_canvas_size.x,
                height: self.landscape_canvas_size.y,
                // has to set both values below to false, because we do custom resizing on our own
                auto_resize_surface: false,
                auto_resize_buffer: false,
                // value below is not used, because of auto_resize_buffer set to false above
                scale_factor: 1.0,
            }),
        });
        app.add_systems(
            (
                Self::sys_update_pixels_on_window_scale_factor_changed,
                Self::sys_update_pixels_on_window_resize,
                Self::sys_resize_pixels_buffer_if_needed
                    .after(Self::sys_update_pixels_on_window_resize),
            )
                .in_base_set(CoreSet::PreUpdate),
        );

        app.insert_resource(BrpDrawQueue::default());
        app.add_system(
            BrpDrawQueue::sys_draw_queued_commands
                .in_base_set(CoreSet::PostUpdate)
                .after(bevy_pixels::PixelsSet::Draw),
        );
    }
}
