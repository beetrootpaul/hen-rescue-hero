use std::ops::Mul;

use bevy::math::uvec2;
use bevy::prelude::*;

use brp_game_base::{rect, BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use collider::Collider;
use position::Position;
use sprites::Sprites;

#[derive(Bundle)]
struct NestBundle {
    token: NestToken,
    position: Position,
    collider: Collider,
}

#[derive(Component)]
pub struct NestToken;

pub struct NestEcs;

impl NestEcs {
    pub fn ss_spawn(mut commands: Commands) {
        commands.spawn(NestBundle {
            token: NestToken,
            position: Position(
                uvec2(1, Canvas::GAME_AREA_TILES.y - 3)
                    .as_ivec2()
                    .mul(Sprites::TILE_ISIZE)
                    .as_vec2(),
            ),
            collider: Collider {
                rect: rect(10, 12).at(-8, -10),
            },
        });
    }

    pub fn s_draw(
        q: Query<&Position, With<NestToken>>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for position in q.iter() {
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position),
                Sprites::Nest.into(),
            ));
        }
    }
}
