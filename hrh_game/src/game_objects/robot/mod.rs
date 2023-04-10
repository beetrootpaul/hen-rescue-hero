use std::time::Duration;

use bevy::math::{ivec2, uvec2, vec2};
use bevy::prelude::*;

use brp_game_base::Rect;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::robot_direction::RobotDirection;
pub use game_objects::robot::robot_ecs::RobotEcs;
use game_objects::robot::robot_speed::RobotSpeed;
use game_objects::robot::robot_state::RobotState;
use sprite::Sprite;

mod robot_direction;
mod robot_ecs;
mod robot_speed;
mod robot_state;

#[derive(Component)]
pub struct Robot {
    direction: RobotDirection,
    speed: RobotSpeed,
    state: RobotState,
    overheating_timer: Timer,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            direction: RobotDirection::LeftStaying,
            speed: RobotSpeed(0.0),
            state: RobotState::Good,
            overheating_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }

    //

    pub fn is_flipped(&self) -> bool {
        self.direction.is_right()
    }

    pub fn is_overheated(&self) -> bool {
        self.state == RobotState::Overheated
    }

    pub fn position_diff_after(&self, duration: Duration) -> Vec2 {
        let diff = self.speed.0 * duration.as_secs_f32();
        match self.direction {
            RobotDirection::LeftMoving => vec2(-diff, 0.0),
            RobotDirection::RightMoving => vec2(diff, 0.0),
            _ => Vec2::ZERO,
        }
    }

    pub fn position_body_offset(&self) -> IVec2 {
        self.state.body_offset()
    }

    pub fn face_sprite(&self) -> Sprite {
        match self.state {
            RobotState::Good => Sprite::RobotFace1,
            RobotState::Tired => Sprite::RobotFace2,
            RobotState::VeryTired => Sprite::RobotFace3,
            RobotState::Overheated => Sprite::RobotFace5,
        }
    }

    pub fn collider_rect_for(&self, pile_of_chickens: &PileOfChickens) -> Rect {
        let chicken_amount = pile_of_chickens.amount();
        let top = match chicken_amount {
            0 => -13,
            _ => chicken_amount as i32 * -3 - 16,
        };
        let height = match chicken_amount {
            0 => 4,
            _ => chicken_amount * 3 + 7,
        };
        let collider_rect = Rect {
            left_top: ivec2(-8, top) + self.position_body_offset(),
            size: uvec2(17, height),
        };
        if self.direction.is_right() {
            collider_rect.move_by(ivec2(-1, 0))
        } else {
            collider_rect
        }
    }

    //

    pub fn update_for_pile(&mut self, pile: &PileOfChickens) {
        self.state = RobotState::for_pile(pile);
        self.speed = RobotSpeed::for_state(&self.state);
    }

    pub fn update_direction(&mut self, is_left_requested: bool, is_right_requested: bool) {
        self.direction = match (self.state, is_left_requested, is_right_requested) {
            (RobotState::Overheated, _, _) => self.direction.to_staying(),
            (_, true, false) => RobotDirection::LeftMoving,
            (_, false, true) => RobotDirection::RightMoving,
            (_, _, _) => self.direction.to_staying(),
        };
    }

    pub fn update_overheating(&mut self, duration: Duration) {
        if self.overheating_timer.finished() {
            self.overheating_timer.reset();
        }
        self.overheating_timer.tick(duration);
    }

    pub fn just_finished_overheating(&mut self) -> bool {
        self.overheating_timer.just_finished()
    }
}
