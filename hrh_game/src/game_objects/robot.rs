use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue, Rect};
use canvas::Canvas;
use collider::Collider;
use game_objects::pile_of_chickens::PileOfChickens;
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

#[derive(Component)]
pub enum RobotState {
    Good,
    Tired,
    VeryTired,
}

impl RobotState {
    pub fn for_pile(pile: &PileOfChickens) -> Self {
        match pile.amount() {
            0..=5 => RobotState::Good,
            6..=8 => RobotState::Tired,
            _ => RobotState::VeryTired,
        }
    }

    pub fn body_offset(&self) -> IVec2 {
        match *self {
            RobotState::Good => ivec2(0, 0),
            RobotState::Tired => ivec2(0, 1),
            RobotState::VeryTired => ivec2(0, 2),
        }
    }
}

#[derive(Bundle)]
struct RobotBundle {
    token: RobotToken,
    position: Position,
    direction: RobotDirection,
    pile_of_chickens: PileOfChickens,
    collider: Collider,
    state: RobotState,
}

pub struct RobotEcs;

impl RobotEcs {
    const SPEED_PER_SECOND: f32 = 200.0;
    const BOUNDARY_OFFSET_LEFT: f32 = 10.0;
    const BOUNDARY_OFFSET_RIGHT: f32 = -10.0;

    pub fn ss_spawn(mut commands: Commands) {
        let robot_position = Position(
            ivec2(
                Canvas::GAME_AREA_SIZE.x as i32 / 2,
                (Canvas::GAME_AREA_TILES.y as i32 - 2) * Sprites::TILE_ISIZE.y - 2,
            )
            .as_vec2(),
        );
        let pile = PileOfChickens::default();
        let state = RobotState::for_pile(&pile);
        let collider = Collider {
            rect: Robot::collider_rect_for(-8, 17, &pile, &state),
        };
        commands.spawn(RobotBundle {
            token: RobotToken,
            position: robot_position,
            direction: RobotDirection::None,
            pile_of_chickens: pile,
            collider,
            state,
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
        query: Query<(&Position, &RobotState), With<RobotToken>>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (position, state) in query.iter() {
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position),
                Sprites::RobotLeg.into(),
            ));

            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position) + state.body_offset(),
                Sprites::RobotBody.into(),
            ));

            let face_sprite = match state {
                RobotState::Good => Sprites::RobotFace1,
                RobotState::Tired => Sprites::RobotFace2,
                RobotState::VeryTired => Sprites::RobotFace3,
            };
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position) + state.body_offset(),
                face_sprite.into(),
            ));
        }
    }
}

pub struct Robot;

impl Robot {
    pub fn collider_rect_for(
        x: i32,
        w: u32,
        pile_of_chickens: &PileOfChickens,
        robot_state: &RobotState,
    ) -> Rect {
        let chicken_amount = pile_of_chickens.amount();
        let top = match chicken_amount {
            0 => -13,
            _ => chicken_amount as i32 * -3 - 16,
        };
        let height = match chicken_amount {
            0 => 4,
            _ => chicken_amount * 3 + 7,
        };
        Rect {
            left_top: ivec2(x, top) + robot_state.body_offset(),
            size: uvec2(w, height),
        }
    }
}
