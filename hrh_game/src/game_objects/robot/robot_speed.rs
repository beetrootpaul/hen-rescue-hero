use bevy::prelude::*;
use game_objects::robot::robot_state::RobotState;

#[derive(Component)]
pub struct RobotSpeed(pub f32);

impl RobotSpeed {
    pub fn for_state(state: &RobotState) -> Self {
        match *state {
            RobotState::Good => RobotSpeed(100.0),
            RobotState::Tired => RobotSpeed(80.0),
            RobotState::VeryTired => RobotSpeed(60.0),
            RobotState::Overheated => RobotSpeed(0.0),
        }
    }
}
