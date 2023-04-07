use std::ops::{Mul, Sub};

use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::{Canvas, GAME_AREA_TILES};
use position::Position;
use sprites::{Sprites, TILE_SIZE};

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
}

pub struct RobotSystems;

impl RobotSystems {
    const SPEED_PER_SECOND: f32 = 200.0;

    pub fn spawn(mut commands: Commands) {
        commands.spawn(RobotBundle {
            token: RobotToken,
            position: Position(
                uvec2(0, GAME_AREA_TILES.y - 2)
                    .as_ivec2()
                    .mul(TILE_SIZE.as_ivec2())
                    .sub(ivec2(0, 2))
                    .as_vec2(),
            ),
            direction: RobotDirection::None,
        });
    }

    pub fn update(
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
        }
    }

    pub fn draw(
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
