use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use collider::Collider;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;
use position::Position;
use sprite::Sprite;

#[derive(Bundle)]
struct RobotBundle {
    robot: Robot,
    position: Position,
    pile_of_chickens: PileOfChickens,
    collider: Collider,
}

pub struct RobotEcs;

impl RobotEcs {
    const BOUNDARY_OFFSET_LEFT: f32 = 10.0;
    const BOUNDARY_OFFSET_RIGHT: f32 = -10.0;

    pub fn ss_spawn(mut commands: Commands) {
        let pile_of_chickens = PileOfChickens::default();

        let mut robot = Robot::new();
        robot.update_for_pile(&pile_of_chickens);

        let collider = Collider {
            rect: robot.collider_rect_for(&pile_of_chickens),
        };

        commands.spawn(RobotBundle {
            robot,
            position: Position(
                ivec2(
                    Canvas::GAME_AREA_SIZE.x as i32 / 2,
                    (Canvas::GAME_AREA_TILES.y as i32 - 2) * Sprite::TILE_ISIZE.y - 2,
                )
                .as_vec2(),
            ),
            pile_of_chickens,
            collider,
        });
    }

    pub fn s_update(mut q: Query<(&mut Position, &Robot)>, time: Res<Time>) {
        for (mut position, robot) in q.iter_mut() {
            position.0 += robot.position_diff_after(time.delta());
            position.0.x = position.0.x.clamp(
                Self::BOUNDARY_OFFSET_LEFT,
                Canvas::GAME_AREA_SIZE.x as f32 + Self::BOUNDARY_OFFSET_RIGHT,
            );
        }
    }

    pub fn s_draw(
        query: Query<(&Position, &Robot)>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (position, robot) in query.iter() {
            let flip = robot.is_flipped();

            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position),
                Sprite::RobotLeg.into(),
                flip,
            ));

            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position) + robot.position_body_offset(),
                Sprite::RobotBody.into(),
                flip,
            ));

            let face_sprite = robot.face_sprite();
            let flip_offset = match flip {
                true => ivec2(8, 0),
                false => IVec2::ZERO,
            };
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position)
                    + robot.position_body_offset()
                    + flip_offset,
                face_sprite.into(),
                flip,
            ));
        }
    }
}
