use bevy::prelude::*;

use collider::Collider;
use game_objects::chicken::ChickenToken;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::{Robot, RobotSpeed, RobotState, RobotToken};
use position::Position;

type ChickenAndNotRobot = (With<ChickenToken>, Without<RobotToken>);

pub struct RobotCachesChickensEcs;

impl RobotCachesChickensEcs {
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
        q_chicken: Query<(Entity, &Collider, &Position), ChickenAndNotRobot>,
        mut commands: Commands,
    ) {
        for (mut robot_collider, robot_position, mut pile, mut robot_state, mut robot_speed) in
            q_robot.iter_mut()
        {
            for (chicken_entity, chicken_collider, chicken_position) in q_chicken.iter() {
                if Collider::are_colliding(
                    robot_collider.as_ref(),
                    robot_position,
                    chicken_collider,
                    chicken_position,
                ) {
                    commands.entity(chicken_entity).despawn_recursive();

                    pile.increment();

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
