use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use game_objects::robot::Robot;
use position::Position;
use sprite::Sprite;

#[derive(Default, Component)]
pub struct PileOfChickens(u32);

impl PileOfChickens {
    pub fn amount(&self) -> u32 {
        self.0
    }
    pub fn increment(&mut self) {
        self.0 += 1;
    }
    pub fn take_all(&mut self) -> u32 {
        let prev_amount = self.0;
        self.0 = 0;
        prev_amount
    }
}

pub struct PileOfChickensEcs;

impl PileOfChickensEcs {
    const SEGMENT_OF_8_STACKABLE_Y: i32 = 3 * Sprite::TILE_ISIZE.y;

    pub fn s_draw(
        q: Query<(&Robot, &PileOfChickens, &Position)>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (robot, pile, position) in q.iter() {
            let mut offset = robot.position_body_offset();
            let flip = robot.is_flipped();
            if flip {
                offset += ivec2(-2, 0);
            }

            let segments_of_8 = pile.0 / 8;
            let top_sprite = match pile.0 - segments_of_8 * 8 {
                0 => None,
                1 => Some(Sprite::PileOfChicken1.into()),
                2 => Some(Sprite::PileOfChicken2.into()),
                3 => Some(Sprite::PileOfChicken3.into()),
                4 => Some(Sprite::PileOfChicken4.into()),
                5 => Some(Sprite::PileOfChicken5.into()),
                6 => Some(Sprite::PileOfChicken6.into()),
                7 => Some(Sprite::PileOfChicken7.into()),
                // shouldn't happen ;-)
                _ => None,
            };
            if let Some(s) = top_sprite {
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    canvas.xy_of_position_within_game_area(*position)
                        - ivec2(0, Self::SEGMENT_OF_8_STACKABLE_Y * segments_of_8 as i32)
                        + offset,
                    s,
                    flip,
                ));
            }

            for segment in (0..segments_of_8).rev() {
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    canvas.xy_of_position_within_game_area(*position)
                        - ivec2(0, Self::SEGMENT_OF_8_STACKABLE_Y * segment as i32)
                        + offset,
                    Sprite::PileOfChicken8.into(),
                    flip,
                ));
            }
        }
    }
}
