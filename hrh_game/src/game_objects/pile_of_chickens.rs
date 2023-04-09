use bevy::math::{ivec2, vec2};
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use position::Position;
use sprites::Sprites;

#[derive(Bundle)]
pub struct PileOfChickensBundle {
    token: PileOfChickensToken,
    amount: AmountOfChickens,
    position: Position,
}

impl PileOfChickensBundle {
    pub fn new(relative_xy: Vec2) -> Self {
        Self {
            token: PileOfChickensToken,
            amount: AmountOfChickens(0),
            position: Position(relative_xy),
        }
    }
}

#[derive(Component)]
pub struct PileOfChickensToken;

#[derive(Component)]
pub struct AmountOfChickens(u32);

impl AmountOfChickens {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

pub struct PileOfChickensEcs;

impl PileOfChickensEcs {
    const SEGMENT_OF_8_STACKABLE_Y: i32 = 3 * Sprites::TILE_ISIZE.y;

    pub fn s_draw(
        q_pile: Query<(Option<&Parent>, &AmountOfChickens, &Position), With<PileOfChickensToken>>,
        q_position: Query<(&Position)>,
        commands: Commands,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (maybe_parent, amount, relative_position) in q_pile.iter() {
            let position: Position = match maybe_parent {
                None => *relative_position,
                Some(parent) => {
                    if let Ok(parent_position) = q_position.get(parent.get()) {
                        Position(parent_position.0 + relative_position.0)
                    } else {
                        *relative_position
                    }
                },
            };

            let segments_of_8 = amount.0 / 8;
            let top_sprite = match amount.0 - segments_of_8 * 8 {
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
                    canvas.xy_of_position_within_game_area(position)
                        - ivec2(0, Self::SEGMENT_OF_8_STACKABLE_Y * segments_of_8 as i32),
                    s,
                ));
            }
            for segment in (0..segments_of_8).rev() {
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    canvas.xy_of_position_within_game_area(position)
                        - ivec2(0, Self::SEGMENT_OF_8_STACKABLE_Y * segment as i32),
                    Sprites::PileOfChicken8.into(),
                ));
            }
        }
    }
}
