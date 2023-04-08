use bevy::math::ivec2;
use bevy::prelude::*;

use brp_game_base::{rect, BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use collider::Collider;
use position::Position;
use sprites::Sprites;

#[derive(Component)]
pub struct RobotToken;

#[derive(Component, PartialEq, Eq, Hash, Clone, Debug)]
pub enum RobotDirection {
    None,
    Left,
    Right,
}

#[derive(Bundle)]
struct RobotBundle {
    token: RobotToken,
    position: Position,
    direction: RobotDirection,
    collider: Collider,
}

pub struct RobotEcs;

impl RobotEcs {
    const SPEED_PER_SECOND: f32 = 200.0;
    const BOUNDARY_OFFSET_LEFT: f32 = 10.0;
    const BOUNDARY_OFFSET_RIGHT: f32 = -10.0;

    pub fn ss_spawn(mut commands: Commands) {
        commands.spawn(RobotBundle {
            token: RobotToken,
            position: Position(
                ivec2(
                    Canvas::GAME_AREA_SIZE.x as i32 / 2,
                    (Canvas::GAME_AREA_TILES.y as i32 - 2) * Sprites::TILE_ISIZE.y - 2,
                )
                .as_vec2(),
            ),
            direction: RobotDirection::None,
            collider: Collider {
                rect: rect(16, 4).at(-8, -13),
            },
        });
    }

    pub fn s_update(
        time: Res<Time>,
        mut query: Query<(&mut Position, &RobotDirection), With<RobotToken>>,
    ) {
        let diff = Self::SPEED_PER_SECOND * time.delta_seconds();
        for (mut position, direction) in query.iter_mut() {
            match direction {
                RobotDirection::Left => position.0.x -= diff,
                RobotDirection::Right => position.0.x += diff,
                RobotDirection::None => {},
            }
            position.0.x = position.0.x.clamp(
                Self::BOUNDARY_OFFSET_LEFT,
                Canvas::GAME_AREA_SIZE.x as f32 + Self::BOUNDARY_OFFSET_RIGHT,
            );
        }
    }

    pub fn s_draw(
        query: Query<&Position, With<RobotToken>>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for position in query.iter() {
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(position),
                Sprites::RobotLeg.into(),
            ));
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(position),
                Sprites::RobotBody.into(),
            ));
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(position),
                Sprites::RobotFace1.into(),
            ));
        }
    }
}
