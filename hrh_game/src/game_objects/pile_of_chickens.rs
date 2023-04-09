use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use game_objects::robot::RobotState;
use position::Position;
use sprites::Sprites;

#[derive(Component, Default)]
pub struct PileOfChickens(u32);

impl PileOfChickens {
    pub fn amount(&self) -> u32 {
        self.0
    }
    pub fn increment(&mut self) {
        self.0 += 1;
    }
    pub fn take_all(&mut self) {
        self.0 = 0;
    }
}

pub struct PileOfChickensEcs;

impl PileOfChickensEcs {
    const SEGMENT_OF_8_STACKABLE_Y: i32 = 3 * Sprites::TILE_ISIZE.y;

    pub fn s_draw(
        q_pile: Query<(&PileOfChickens, &Position, Option<&RobotState>)>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (pile, position, maybe_robot_state) in q_pile.iter() {
            let offset = match maybe_robot_state {
                Some(robot_state) => robot_state.body_offset(),
                None => IVec2::ZERO,
            };

            let segments_of_8 = pile.0 / 8;
            let top_sprite = match pile.0 - segments_of_8 * 8 {
                0 => None,
                1 => Some(Sprites::PileOfChicken1.into()),
                2 => Some(Sprites::PileOfChicken2.into()),
                3 => Some(Sprites::PileOfChicken3.into()),
                4 => Some(Sprites::PileOfChicken4.into()),
                5 => Some(Sprites::PileOfChicken5.into()),
                6 => Some(Sprites::PileOfChicken6.into()),
                7 => Some(Sprites::PileOfChicken7.into()),
                // shouldn't happen ;-)
                _ => None,
            };
            if let Some(s) = top_sprite {
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    canvas.xy_of_position_within_game_area(*position)
                        - ivec2(0, Self::SEGMENT_OF_8_STACKABLE_Y * segments_of_8 as i32)
                        + offset,
                    s,
                ));
            }

            for segment in (0..segments_of_8).rev() {
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    canvas.xy_of_position_within_game_area(*position)
                        - ivec2(0, Self::SEGMENT_OF_8_STACKABLE_Y * segment as i32)
                        + offset,
                    Sprites::PileOfChicken8.into(),
                ));
            }
        }
    }
}
