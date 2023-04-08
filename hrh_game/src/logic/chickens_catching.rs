use bevy::prelude::*;
use collider::Collider;
use game_objects::chicken::ChickenToken;
use game_objects::robot::RobotToken;
use position::Position;

pub struct ChickensCatchingEcs;

impl ChickensCatchingEcs {
    pub fn s_catch_chickens(
        robot_query: Query<(&Collider, &Position), With<RobotToken>>,
        chicken_query: Query<(Entity, &Collider, &Position), With<ChickenToken>>,
        mut commands: Commands,
    ) {
        for (robot_collider, robot_position) in robot_query.iter() {
            for (chicken_entity, chicken_collider, chicken_position) in chicken_query.iter() {
                if Collider::are_colliding(
                    robot_collider,
                    robot_position,
                    chicken_collider,
                    chicken_position,
                ) {
                    commands.entity(chicken_entity).despawn_recursive();
                }
            }
        }
    }
}
