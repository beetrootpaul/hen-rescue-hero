use bevy::prelude::*;

use brp_game_base::Rect;
#[cfg(debug_assertions)]
use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
#[cfg(debug_assertions)]
use canvas::Canvas;
#[cfg(debug_assertions)]
use pico8_color::Pico8Color;
use position::Position;

#[derive(Component)]
pub struct Collider {
    pub rect: Rect,
}

impl Collider {
    pub fn are_colliding(
        collider_1: &Collider,
        position_1: &Position,
        collider_2: &Collider,
        position_2: &Position,
    ) -> bool {
        let rect_1 = collider_1.rect.move_by(position_1.0.as_ivec2());
        let rect_2 = collider_2.rect.move_by(position_2.0.as_ivec2());
        let common_rect = rect_1.intersection_with(rect_2);
        common_rect.width() > 0 && common_rect.height() > 0
    }
}

#[cfg(debug_assertions)]
#[derive(Resource)]
pub struct CollidersDebugConfig {
    pub is_debug_draw_enabled: bool,
}

#[cfg(debug_assertions)]
pub struct ColliderEcs;

#[cfg(debug_assertions)]
impl ColliderEcs {
    #[cfg(debug_assertions)]
    pub fn r_debug_config() -> CollidersDebugConfig {
        CollidersDebugConfig {
            is_debug_draw_enabled: false,
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
