use bevy::prelude::*;

use collider::Collider;
use game_objects::chicken::ChickenToken;
use game_objects::pile_of_chickens::{AmountOfChickens, PileOfChickensToken};
use game_objects::robot::RobotToken;
use position::Position;

pub struct ChickensCatchingEcs;

impl ChickensCatchingEcs {
    pub fn s_catch_chickens(
        q_chicken: Query<(Entity, &Collider, &Position), With<ChickenToken>>,
        q_robot: Query<(&Collider, &Position, &Children), With<RobotToken>>,
        mut q_pile: Query<&mut AmountOfChickens, With<PileOfChickensToken>>,
        mut commands: Commands,
    ) {
        for (chicken_entity, chicken_collider, chicken_position) in q_chicken.iter() {
            for (robot_collider, robot_position, robot_children) in q_robot.iter() {
                if Collider::are_colliding(
                    robot_collider,
                    robot_position,
                    chicken_collider,
                    chicken_position,
                ) {
                    commands.entity(chicken_entity).despawn_recursive();

                    for robot_child in robot_children.iter() {
                        if let Ok(mut amount_of_chickens) = q_pile.get_mut(*robot_child) {
                            amount_of_chickens.increment();
                        }
                    }
                }
            }
        }
    }
}
