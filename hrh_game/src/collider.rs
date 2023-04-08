use bevy::prelude::{Component, Query, ResMut};
use brp_game_base::{BrpDrawCommand, BrpDrawQueue, Rect};
use canvas::Canvas;
use pico8_color::Pico8Color;
use position::Position;

#[derive(Component)]
pub struct Collider {
    pub rect: Rect,
}

pub struct ColliderEcs;

impl ColliderEcs {
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
