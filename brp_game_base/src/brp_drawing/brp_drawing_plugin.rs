use bevy::math::{ivec2, uvec2, DVec2};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowBackendScaleFactorChanged, WindowResized};

use brp_drawing::brp_canvas_variant::{BrpCanvasVariant, BrpCurrentCanvasVariant};
use brp_drawing::brp_draw_queue::BrpDrawQueue;
use brp_game_config::BrpGameConfig;
use brp_input::{BrpInputConfig, BrpInputPlugin};
use BrpColor;

#[derive(Resource)]
pub struct BrpDrawingInfo {
    real_position_inside_window: IVec2,
    scale_logical_to_real: i32,
    viewport_scale_factor: f64,
}

impl BrpDrawingInfo {
    pub fn real_viewport_xy_to_canvas_xy(&self, viewport_xy: IVec2) -> IVec2 {
        let scaled_viewport_xy: DVec2 = viewport_xy.as_dvec2() * self.viewport_scale_factor;
        let real_canvas_xy: DVec2 =
            scaled_viewport_xy - self.real_position_inside_window.as_dvec2();
        let logical_canvas_xy: DVec2 = real_canvas_xy / (self.scale_logical_to_real as f64);
        logical_canvas_xy.as_ivec2()
    }
}

pub struct BrpDrawingPlugin {
    pub canvas_margin_color: BrpColor,
    pub square_canvas_size: UVec2,
    pub landscape_canvas_size: UVec2,
    pub portrait_canvas_size: UVec2,
}

impl BrpDrawingPlugin {
    fn s_update_drawing_info(
        primary_window_query: Query<Entity, With<PrimaryWindow>>,
        winit_windows: NonSend<bevy::winit::WinitWindows>,
        #[allow(clippy::type_complexity)] query: Query<
            (&bevy_pixels::PixelsWrapper, &bevy_pixels::PixelsOptions),
            (With<Window>, Changed<bevy_pixels::PixelsOptions>),
        >,
        mut commands: Commands,
    ) {
        let primary_window = primary_window_query
            .get_single()
            .expect("should query single primary window");
        if let Ok((wrapper, options)) = query.get(primary_window) {
            warn!("INIT DRAWING PROPERLY 2");
            if let Some(winit_window) = winit_windows.get_window(primary_window) {
                warn!("INIT DRAWING PROPERLY 3");

                let (real_x, real_y, real_w, _real_h) =
                    wrapper.pixels.context().scaling_renderer.clip_rect();
                commands.insert_resource(BrpDrawingInfo {
                    scale_logical_to_real: real_w as i32 / options.width as i32,
                    real_position_inside_window: uvec2(real_x, real_y).as_ivec2(),
                    viewport_scale_factor: winit_window.scale_factor(),
                });
            }
        }
    }

    fn s_update_pixels_on_window_scale_factor_changed(
        mut window_backend_scale_factor_changed_events: EventReader<
            WindowBackendScaleFactorChanged,
        >,
        mut query: Query<
            (&mut bevy_pixels::PixelsWrapper, &bevy_pixels::PixelsOptions),
            With<Window>,
        >,
        winit_windows: NonSend<bevy::winit::WinitWindows>,
        mut commands: Commands,
    ) {
        for event in window_backend_scale_factor_changed_events.iter() {
            if let Ok((mut wrapper, options)) = query.get_mut(event.window) {
                if let Some(resized_winit_window) = winit_windows.get_window(event.window) {
                    Self::resize_pixels_surface_to_window(
                        &mut wrapper,
                        resized_winit_window.inner_size().width,
                        resized_winit_window.inner_size().height,
                    );

                    let (real_x, real_y, real_w, _real_h) =
                        wrapper.pixels.context().scaling_renderer.clip_rect();
                    commands.insert_resource(BrpDrawingInfo {
                        scale_logical_to_real: real_w as i32 / options.width as i32,
                        real_position_inside_window: uvec2(real_x, real_y).as_ivec2(),
                        viewport_scale_factor: resized_winit_window.scale_factor(),
                    });
                }
            }
        }
    }

    fn s_update_pixels_on_window_resize(
        mut window_resized_events: EventReader<WindowResized>,
        mut query: Query<
            (
                &mut bevy_pixels::PixelsWrapper,
                &mut bevy_pixels::PixelsOptions,
            ),
            With<Window>,
        >,
        winit_windows: NonSend<bevy::winit::WinitWindows>,
        input_config: Res<BrpInputConfig>,
        mut current_canvas_variant: ResMut<BrpCurrentCanvasVariant>,
        game_config: Res<BrpGameConfig>,
        mut commands: Commands,
    ) {
        if let Some(event) = window_resized_events.iter().last() {
            if let Ok((mut wrapper, mut options)) = query.get_mut(event.window) {
                if let Some(resized_winit_window) = winit_windows.get_window(event.window) {
                    let window_w = resized_winit_window.inner_size().width;
                    let window_h = resized_winit_window.inner_size().height;

                    Self::resize_pixels_surface_to_window(&mut wrapper, window_w, window_h);

                    let canvas_variant =
                        match (input_config.is_touch_available, window_w > window_h) {
                            (false, _) => BrpCanvasVariant::NoTouchControls,
                            (true, true) => BrpCanvasVariant::TouchControlsLandscape,
                            (true, false) => BrpCanvasVariant::TouchControlsPortrait,
                        };

                    let new_canvas_size = match canvas_variant {
                        BrpCanvasVariant::NoTouchControls => game_config.square_canvas_size,
                        BrpCanvasVariant::TouchControlsLandscape => {
                            game_config.landscape_canvas_size
                        },
                        BrpCanvasVariant::TouchControlsPortrait => game_config.portrait_canvas_size,
                    };

                    // mutate things only when really needed, in order to not trigger systems based on `Changed<_>` queries w/o need
                    if canvas_variant != current_canvas_variant.0 {
                        options.width = new_canvas_size.x;
                        options.height = new_canvas_size.y;
                        current_canvas_variant.0 = canvas_variant;
                    }

                    let (real_x, real_y, real_w, _real_h) =
                        wrapper.pixels.context().scaling_renderer.clip_rect();
                    commands.insert_resource(BrpDrawingInfo {
                        scale_logical_to_real: real_w as i32 / new_canvas_size.x as i32,
                        real_position_inside_window: uvec2(real_x, real_y).as_ivec2(),
                        viewport_scale_factor: resized_winit_window.scale_factor(),
                    });
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

    fn s_resize_pixels_buffer_if_needed(
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
                width: self.square_canvas_size.x,
                height: self.square_canvas_size.y,
                // has to set both values below to false, because we do custom resizing on our own
                auto_resize_surface: false,
                auto_resize_buffer: false,
                // value below is not used, because of auto_resize_buffer set to false above
                scale_factor: 1.0,
            }),
        });

        app.insert_resource(BrpDrawingInfo {
            real_position_inside_window: IVec2::ZERO,
            scale_logical_to_real: 1,
            viewport_scale_factor: 1.0,
        });

        let canvas_margin_color: BrpColor = self.canvas_margin_color;
        app.add_system(
            move |mut query: Query<
                &mut bevy_pixels::PixelsWrapper,
                Added<bevy_pixels::PixelsOptions>,
            >| {
                for mut pixels_wrapper in query.iter_mut() {
                    let wgpu_color =
                        bevy_pixels::pixels::wgpu::Color::from(Color::from(canvas_margin_color));
                    pixels_wrapper.pixels.clear_color(wgpu_color);
                }
            },
        );

        app.init_resource::<BrpCurrentCanvasVariant>();

        app.add_systems(
            (
                Self::s_update_pixels_on_window_scale_factor_changed,
                Self::s_update_pixels_on_window_resize,
                Self::s_resize_pixels_buffer_if_needed
                    .after(Self::s_update_pixels_on_window_resize),
                Self::s_update_drawing_info
                    .after(Self::s_resize_pixels_buffer_if_needed)
                    .after(Self::s_update_pixels_on_window_resize),
            )
                .in_base_set(CoreSet::PreUpdate),
        );

        app.insert_resource(BrpDrawQueue::default());
        app.add_system(
            BrpDrawQueue::s_draw_queued_commands
                .in_base_set(CoreSet::PostUpdate)
                .after(bevy_pixels::PixelsSet::Draw),
        );
    }
}
