use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::Rect;
use collider::Collider;
use game_objects::chicken::ChickenToken;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::RobotToken;
use position::Position;

type ChickenAndNotRobot = (With<ChickenToken>, Without<RobotToken>);

pub struct ChickensCatchingEcs;

impl ChickensCatchingEcs {
    pub fn s_catch_chickens(
        mut q_robot: Query<(&mut Collider, &Position, &mut PileOfChickens), With<RobotToken>>,
        q_chicken: Query<(Entity, &Collider, &Position), ChickenAndNotRobot>,
        mut commands: Commands,
    ) {
        for (mut robot_collider, robot_position, mut pile) in q_robot.iter_mut() {
            for (chicken_entity, chicken_collider, chicken_position) in q_chicken.iter() {
                if Collider::are_colliding(
                    robot_collider.as_ref(),
                    robot_position,
                    chicken_collider,
                    chicken_position,
                ) {
                    commands.entity(chicken_entity).despawn_recursive();

                    pile.increment();

                    robot_collider.rect = Rect {
                        left_top: ivec2(
                            robot_collider.rect.left_top.x,
                            pile.amount() as i32 * -3 - 16,
                        ),
                        size: uvec2(robot_collider.rect.size.x, pile.amount() * 3 + 7),
                    };
                }
            }
        }
    }
}
