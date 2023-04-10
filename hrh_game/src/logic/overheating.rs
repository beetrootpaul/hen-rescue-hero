use bevy::prelude::*;

use collider::Collider;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;

pub struct OverheatingEcs;

impl OverheatingEcs {
    pub fn s_advance_timer(
        mut q: Query<(&mut Robot, &mut PileOfChickens, &mut Collider)>,
        time: Res<Time>,
    ) {
        for (mut robot, mut pile, mut robot_collider) in q.iter_mut() {
            if robot.is_overheated() {
                robot.update_overheating(time.delta());
                if robot.just_finished_overheating() {
                    pile.take_all();
                    robot.update_for_pile(&pile);
                    robot_collider.rect = robot.collider_rect_for(&pile);
                }
            }
        }
    }
}
