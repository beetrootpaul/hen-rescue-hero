use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use pico8_color::Pico8Color;
use sprite::Sprite;

#[derive(Resource)]
pub struct Score {
    rescued_chickens: u32,
}

impl Score {
    pub fn rescued_chickens(&self) -> u32 {
        self.rescued_chickens
    }
    pub fn add_to_rescued_chickens(&mut self, amount: u32) {
        self.rescued_chickens += amount;
    }
    pub fn reset(&mut self) {
        self.rescued_chickens = 0;
    }
}

pub struct ScoreEcs;

impl ScoreEcs {
    pub fn r_score() -> Score {
        Score {
            rescued_chickens: 0,
        }
    }

    pub fn s_reset(mut score: ResMut<Score>) {
        score.reset();
    }

    pub fn s_draw(mut draw_queue: ResMut<BrpDrawQueue>, canvas: Canvas, score: Res<Score>) {
        let nest_xy = canvas.top_bar_rect().left_top + ivec2(12, 13);
        draw_queue.enqueue(BrpDrawCommand::Sprite(
            nest_xy + ivec2(0, -2),
            Sprite::Chicken.into(),
            false,
        ));
        draw_queue.enqueue(BrpDrawCommand::Sprite(nest_xy, Sprite::Nest.into(), false));
        draw_queue.enqueue(BrpDrawCommand::Text(
            nest_xy + ivec2(11, -10),
            format!("{}", score.rescued_chickens()),
            Pico8Color::LightPeach.into(),
        ));
    }
}
