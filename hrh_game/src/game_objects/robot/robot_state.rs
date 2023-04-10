use bevy::math::ivec2;
use bevy::prelude::*;

use game_objects::pile_of_chickens::PileOfChickens;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum RobotState {
    Good,
    Tired,
    VeryTired,
    Overheated,
}

impl RobotState {
    pub fn for_pile(pile: &PileOfChickens) -> Self {
        match pile.amount() {
            // TODO: uncomment
            // 0..=5 => RobotState::Good,
            // 6..=10 => RobotState::Tired,
            // 11..=15 => RobotState::VeryTired,
            // _ => RobotState::Overheated,
            0..=1 => RobotState::Good,
            2..=2 => RobotState::Tired,
            3..=3 => RobotState::VeryTired,
            _ => RobotState::Overheated,
        }
    }

    pub fn body_offset(&self) -> IVec2 {
        match *self {
            RobotState::Good => ivec2(0, 0),
            RobotState::Tired => ivec2(0, 1),
            RobotState::VeryTired => ivec2(0, 2),
            RobotState::Overheated => ivec2(0, 2),
        }
    }
}
