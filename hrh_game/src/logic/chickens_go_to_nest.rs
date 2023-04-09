use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::Rect;
use collider::Collider;
use game_objects::chicken::ChickenToken;
use game_objects::nest::NestToken;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::{Robot, RobotState, RobotToken};
use position::Position;

type NestAndNotRobot = (With<NestToken>, Without<RobotToken>);

pub struct ChickensGoToNestEcs;

impl ChickensGoToNestEcs {
    pub fn s_perform(
        mut q_robot: Query<
            (
                &mut Collider,
                &Position,
                &mut PileOfChickens,
                &mut RobotState,
            ),
            With<RobotToken>,
        >,
        q_nest: Query<(&Collider, &Position), NestAndNotRobot>,
    ) {
        for (mut robot_collider, robot_position, mut pile, mut robot_state) in q_robot.iter_mut() {
            for (nest_collider, nest_position) in q_nest.iter() {
                if Collider::are_colliding(
                    robot_collider.as_ref(),
                    robot_position,
                    nest_collider,
                    nest_position,
                ) {
                    pile.take_all();

                    *robot_state = RobotState::for_pile(pile.as_ref());

                    robot_collider.rect = Robot::collider_rect_for(
                        robot_collider.rect.left_top.x,
                        robot_collider.rect.size.x,
                        pile.as_ref(),
                        robot_state.as_ref(),
                    );
                }
            }
        }
    }
}
