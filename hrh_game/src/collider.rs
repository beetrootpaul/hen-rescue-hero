use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue, Rect};
use canvas::Canvas;
use pico8_color::Pico8Color;
use position::Position;

#[derive(Component)]
pub struct Collider {
    pub rect: Rect,
}

#[cfg(debug_assertions)]
#[derive(Resource)]
pub struct CollidersDebugConfig {
    is_debug_draw_enabled: bool,
}

pub struct ColliderEcs;

impl ColliderEcs {
    #[cfg(debug_assertions)]
    pub fn r_debug_config() -> CollidersDebugConfig {
        CollidersDebugConfig {
            is_debug_draw_enabled: false,
        }
    }

    #[cfg(debug_assertions)]
    pub fn s_toggle_debug_draw(
        keyboard_input: Res<Input<KeyCode>>,
        mut config: ResMut<CollidersDebugConfig>,
    ) {
        // c = toggle debug draw of [c]olliders
        if keyboard_input.just_pressed(KeyCode::C) {
            config.is_debug_draw_enabled = !config.is_debug_draw_enabled;
        }
    }

    #[cfg(debug_assertions)]
    pub fn c_is_debug_draw_enabled(config: Res<CollidersDebugConfig>) -> bool {
        config.is_debug_draw_enabled
    }

    #[cfg(debug_assertions)]
    pub fn s_debug_draw_colliders(
        query: Query<(&Collider, &Position)>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (collider, position) in query.iter() {
            draw_queue.enqueue(BrpDrawCommand::Rect(
                collider
                    .rect
                    .move_by(canvas.xy_of_position_within_game_area(position)),
                Pico8Color::Yellow.into(),
            ));
        }
    }
}
