use bevy::prelude::*;

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

    pub fn s_draw(score: Res<Score>) {
        // TODO: DRAW SCORE IN THE TOP BAR
        println!("score: {}", score.rescued_chickens);
    }
}
