use bevy::math::vec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use collider::Collider;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;
use position::Position;
use sprite::Sprite;

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

    pub fn s_draw(q: Query<&Robot>, mut draw_queue: ResMut<BrpDrawQueue>, canvas: Canvas) {
        for robot in q.iter() {
            if robot.is_overheated() {
                let xy = vec2(0.0, -30.0 * robot.overheating_elapsed().as_secs_f32());
                draw_queue.enqueue(BrpDrawCommand::Sprite(
                    canvas.xy_of_position_within_game_area(Position(xy)),
                    Sprite::OverheatedBg.into(),
                    false,
                ));
                break;
            }
        }
    }
}
