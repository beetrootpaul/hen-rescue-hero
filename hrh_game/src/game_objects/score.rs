use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use pico8_color::Pico8Color;
use sprites::Sprites;

#[derive(Resource)]
pub struct Score {
    rescued_chickens: u32,
}

impl Score {
    pub fn add_to_rescued_chickens(&mut self, amount: u32) {
        self.rescued_chickens += amount;
    }
}

pub struct ScoreEcs;

impl ScoreEcs {
    pub fn r_score() -> Score {
        Score {
            rescued_chickens: 0,
        }
    }

    pub fn s_draw(mut draw_queue: ResMut<BrpDrawQueue>, canvas: Canvas, score: Res<Score>) {
        let nest_xy = canvas.top_bar_rect().left_top + ivec2(12, 13);
        draw_queue.enqueue(BrpDrawCommand::Sprite(
            nest_xy + ivec2(0, -2),
            Sprites::Chicken.into(),
        ));
        draw_queue.enqueue(BrpDrawCommand::Sprite(nest_xy, Sprites::Nest.into()));
        draw_queue.enqueue(BrpDrawCommand::Text(
            nest_xy + ivec2(11, -10),
            format!("{}", score.rescued_chickens),
            Pico8Color::LightPeach.into(),
        ));
    }
}
