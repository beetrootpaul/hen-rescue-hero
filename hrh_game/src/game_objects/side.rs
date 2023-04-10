use std::ops::Mul;

use bevy::math::uvec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use position::Position;
use sprite::Sprite;

pub struct SideEcs;

impl SideEcs {
    pub fn s_draw(mut draw_queue: ResMut<BrpDrawQueue>, canvas: Canvas) {
        let position = Position(
            uvec2(1, Canvas::GAME_AREA_TILES.y)
                .as_ivec2()
                .mul(Sprite::TILE_ISIZE)
                .as_vec2(),
        );
        draw_queue.enqueue(BrpDrawCommand::Sprite(
            canvas.xy_of_position_within_game_area(position),
            Sprite::Side.into(),
            false,
        ));
    }
}
