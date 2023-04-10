use std::ops::Range;

use bevy::math::vec2;
use bevy::prelude::*;
use rand::Rng;

use brp_game_base::{rect, BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use collider::Collider;
use position::Position;
use sprite::Sprite;

#[derive(Resource)]
pub struct ChickenSpawnTimer(pub Timer);

#[derive(Component)]
pub struct ChickenToken;

#[derive(Bundle)]
struct ChickenBundle {
    token: ChickenToken,
    position: Position,
    collider: Collider,
}

pub struct ChickenEcs;

impl ChickenEcs {
    const SPAWN_INTERVAL: f32 = 0.3;

    const NO_SPAWN_BORDER_W: f32 = Sprite::TILE_ISIZE.y as f32 * 4.0;
    const SPAWN_X_RANGE: Range<f32> = (6.0 + Self::NO_SPAWN_BORDER_W)
        ..(Canvas::GAME_AREA_SIZE.x as f32 - 6.0 - Self::NO_SPAWN_BORDER_W);
    const SPAWN_Y: f32 = 0.0;
    const DESPAWN_Y: f32 = Canvas::GAME_AREA_SIZE.y as f32 + 11.0;

    const SPEED_PER_SECOND: f32 = 80.0;

    pub fn r_spawn_timer() -> ChickenSpawnTimer {
        ChickenSpawnTimer(Timer::from_seconds(
            Self::SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }

    pub fn s_spawn(time: Res<Time>, mut timer: ResMut<ChickenSpawnTimer>, mut commands: Commands) {
        if timer.0.tick(time.delta()).just_finished() {
            commands.spawn(ChickenBundle {
                token: ChickenToken,
                position: Position(vec2(
                    rand::thread_rng().gen_range(Self::SPAWN_X_RANGE),
                    Self::SPAWN_Y,
                )),
                collider: Collider {
                    rect: rect(12, 11).at(-6, -11),
                },
            });
        }
    }

    pub fn s_reset(
        mut timer: ResMut<ChickenSpawnTimer>,
        q: Query<Entity, With<ChickenToken>>,
        mut commands: Commands,
    ) {
        timer.0.reset();
        for entity in q.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    pub fn s_update(
        time: Res<Time>,
        mut query: Query<(Entity, &mut Position), With<ChickenToken>>,
        mut commands: Commands,
    ) {
        let diff = Self::SPEED_PER_SECOND * time.delta_seconds();
        for (entity, mut position) in query.iter_mut() {
            position.0.y += diff;
            if position.0.y >= Self::DESPAWN_Y {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn s_draw(
        query: Query<&Position, With<ChickenToken>>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for position in query.iter() {
            let brp_sprite = Sprite::Chicken.into();
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position),
                brp_sprite,
                false,
            ));
        }
    }
}
