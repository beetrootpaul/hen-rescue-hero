use std::ops::Mul;

use bevy::math::uvec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::{Canvas, GAME_AREA_TILES};
use position::Position;
use sprites::{Sprites, TILE_SIZE};

pub struct RailSystems;

impl RailSystems {
    pub fn draw(mut draw_queue: ResMut<BrpDrawQueue>, canvas: Canvas) {
        for tile_x in 0..GAME_AREA_TILES.x {
            let position = Position(
                uvec2(tile_x, GAME_AREA_TILES.y - 2)
                    .as_ivec2()
                    .mul(TILE_SIZE.as_ivec2())
                    .as_vec2(),
            );
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(&position),
                Sprites::Chain.into(),
            ));
        }
    }
}
