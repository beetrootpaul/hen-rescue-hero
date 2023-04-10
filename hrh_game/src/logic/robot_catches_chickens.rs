use bevy::prelude::*;

use collider::Collider;
use game_objects::chicken::ChickenToken;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;
use position::Position;

type IsChickenAndNotRobot = (With<ChickenToken>, Without<Robot>);

pub struct RobotCachesChickensEcs;

impl RobotCachesChickensEcs {
    pub fn s_perform(
        mut q_robot: Query<(&mut Robot, &mut Collider, &Position, &mut PileOfChickens)>,
        q_chicken: Query<(Entity, &Collider, &Position), IsChickenAndNotRobot>,
        mut commands: Commands,
    ) {
        for (mut robot, mut robot_collider, robot_position, mut pile) in q_robot.iter_mut() {
            if robot.is_overheated() {
                continue;
            }
            for (chicken_entity, chicken_collider, chicken_position) in q_chicken.iter() {
                if Collider::are_colliding(
                    robot_collider.as_ref(),
                    robot_position,
                    chicken_collider,
                    chicken_position,
                ) {
                    commands.entity(chicken_entity).despawn_recursive();
                    pile.increment();
                    robot.update_for_pile(pile.as_ref());
                    robot_collider.rect = robot.collider_rect_for(&pile);
                }
            }
        }
    }
}
