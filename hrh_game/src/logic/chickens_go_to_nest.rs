use bevy::prelude::*;

use collider::Collider;
use game_objects::nest::NestToken;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::{Robot, RobotSpeed, RobotState, RobotToken};
use game_objects::score::Score;
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
                &mut RobotSpeed,
            ),
            With<RobotToken>,
        >,
        q_nest: Query<(&Collider, &Position), NestAndNotRobot>,
        mut score: ResMut<Score>,
    ) {
        for (mut robot_collider, robot_position, mut pile, mut robot_state, mut robot_speed) in
            q_robot.iter_mut()
        {
            for (nest_collider, nest_position) in q_nest.iter() {
                if Collider::are_colliding(
                    robot_collider.as_ref(),
                    robot_position,
                    nest_collider,
                    nest_position,
                ) {
                    let rescued_amount = pile.take_all();
                    score.add_to_rescued_chickens(rescued_amount);

                    *robot_state = RobotState::for_pile(pile.as_ref());

                    *robot_speed = RobotSpeed::for_state(robot_state.as_ref());

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
