use std::time::Duration;

use bevy::prelude::*;
use collider::Collider;

use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::{Robot, RobotDirection, RobotSpeed, RobotState};

#[derive(Component)]
pub struct OverheatingTimer(pub Timer);

pub struct OverheatingEcs;

impl OverheatingEcs {
    pub fn s_start_timer(mut q: Query<(&RobotState, &mut OverheatingTimer), Changed<RobotState>>) {
        for (robot_state, mut timer) in q.iter_mut() {
            if *robot_state == RobotState::Overheated {
                println!("==== RESET");
                timer.0.reset();
            }
        }
    }

    pub fn s_advance_timer(
        mut q: Query<(
            &mut RobotState,
            &mut OverheatingTimer,
            &mut PileOfChickens,
            &mut RobotSpeed,
            &mut Collider,
            &RobotDirection,
        )>,
        time: Res<Time>,
    ) {
        for (
            mut robot_state,
            mut timer,
            mut pile,
            mut robot_speed,
            mut robot_collider,
            robot_direction,
        ) in q.iter_mut()
        {
            if *robot_state == RobotState::Overheated && timer.0.tick(time.delta()).just_finished()
            {
                println!("!! finished");
                pile.take_all();
                *robot_state = RobotState::for_pile(pile.as_ref());
                *robot_speed = RobotSpeed::for_state(robot_state.as_ref());
                robot_collider.rect =
                    Robot::collider_rect_for(pile.as_ref(), robot_state.as_ref(), robot_direction);
            }
        }
    }
}
