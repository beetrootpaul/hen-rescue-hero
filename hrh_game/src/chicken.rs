use bevy::math::vec2;
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue};
use canvas::Canvas;
use position::Position;
use sprites::Sprites;

#[derive(Resource)]
pub struct ChickenSpawnTimer(pub Timer);

#[derive(Component)]
pub struct ChickenToken;

#[derive(Bundle)]
struct ChickenBundle {
    token: ChickenToken,
    position: Position,
}

pub struct ChickenSystems;

impl ChickenSystems {
    const SPEED_PER_SECOND: f32 = 100.0;
    const SPAWN_Y: f32 = 20.0;
    const DESPAWN_Y: f32 = Canvas::GAME_AREA_SIZE.y as f32 - 30.0;

    pub fn spawn(time: Res<Time>, mut timer: ResMut<ChickenSpawnTimer>, mut commands: Commands) {
        if timer.0.tick(time.delta()).just_finished() {
            commands.spawn(ChickenBundle {
                token: ChickenToken,
                position: Position(vec2(0.0, Self::SPAWN_Y)),
            });
        }
    }

    pub fn update(
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

    pub fn draw(
        query: Query<&Position, With<ChickenToken>>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for position in query.iter() {
            let brp_sprite = Sprites::Chicken.into();
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(position),
                brp_sprite,
            ));
        }
    }
}
