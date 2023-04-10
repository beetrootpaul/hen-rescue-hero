use bevy::prelude::*;

use collider::Collider;
use game_objects::nest::NestToken;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;
use game_objects::score::Score;
use position::Position;

type IsNestAndNotRobot = (With<NestToken>, Without<Robot>);

pub struct ChickensGoToNestEcs;

impl ChickensGoToNestEcs {
    pub fn s_perform(
        mut q_robot: Query<(&mut Robot, &mut Collider, &Position, &mut PileOfChickens)>,
        q_nest: Query<(&Collider, &Position), IsNestAndNotRobot>,
        mut score: ResMut<Score>,
    ) {
        for (mut robot, mut robot_collider, robot_position, mut pile) in q_robot.iter_mut() {
            for (nest_collider, nest_position) in q_nest.iter() {
                if Collider::are_colliding(
                    robot_collider.as_ref(),
                    robot_position,
                    nest_collider,
                    nest_position,
                ) {
                    let rescued_amount = pile.take_all();
                    score.add_to_rescued_chickens(rescued_amount);
                    robot.update_for_pile(pile.as_ref());
                    robot_collider.rect = robot.collider_rect_for(&pile);
                }
            }
        }
    }
}
