use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;

use brp_game_base::{BrpDrawCommand, BrpDrawQueue, Rect};
use canvas::Canvas;
use collider::Collider;
use game_objects::pile_of_chickens::PileOfChickens;
use position::Position;
use sprites::Sprites;

#[derive(Bundle)]
struct RobotBundle {
    token: RobotToken,
    position: Position,
    direction: RobotDirection,
    speed: RobotSpeed,
    pile_of_chickens: PileOfChickens,
    collider: Collider,
    state: RobotState,
}

#[derive(Component)]
pub struct RobotToken;

#[derive(Component, PartialEq, Eq, Hash, Clone, Debug)]
pub enum RobotDirection {
    LeftStaying,
    LeftMoving,
    RightStaying,
    RightMoving,
}

impl RobotDirection {
    pub fn is_right(&self) -> bool {
        match *self {
            RobotDirection::LeftStaying => false,
            RobotDirection::LeftMoving => false,
            RobotDirection::RightStaying => true,
            RobotDirection::RightMoving => true,
        }
    }
    pub fn to_staying(&self) -> RobotDirection {
        match *self {
            RobotDirection::LeftStaying => RobotDirection::LeftStaying,
            RobotDirection::LeftMoving => RobotDirection::LeftStaying,
            RobotDirection::RightStaying => RobotDirection::RightStaying,
            RobotDirection::RightMoving => RobotDirection::RightStaying,
        }
    }
}

#[derive(Component)]
pub struct RobotSpeed(pub f32);

impl RobotSpeed {
    pub fn for_state(state: &RobotState) -> Self {
        match *state {
            RobotState::Good => RobotSpeed(100.0),
            RobotState::Tired => RobotSpeed(80.0),
            _ => RobotSpeed(60.0),
        }
    }
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
            6..=10 => RobotState::Tired,
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

pub struct RobotEcs;

impl RobotEcs {
    const BOUNDARY_OFFSET_LEFT: f32 = 10.0;
    const BOUNDARY_OFFSET_RIGHT: f32 = -10.0;

    pub fn ss_spawn(mut commands: Commands) {
        let pile = PileOfChickens::default();
        let robot_position = Position(
            ivec2(
                Canvas::GAME_AREA_SIZE.x as i32 / 2,
                (Canvas::GAME_AREA_TILES.y as i32 - 2) * Sprites::TILE_ISIZE.y - 2,
            )
            .as_vec2(),
        );
        let state = RobotState::for_pile(&pile);
        let direction = RobotDirection::LeftStaying;
        let collider = Collider {
            rect: Robot::collider_rect_for(&pile, &state, &direction),
        };
        let speed = RobotSpeed::for_state(&state);
        commands.spawn(RobotBundle {
            token: RobotToken,
            position: robot_position,
            direction,
            speed,
            pile_of_chickens: pile,
            collider,
            state,
        });
    }

    pub fn s_update(
        time: Res<Time>,
        mut query: Query<(&mut Position, &RobotDirection, &RobotSpeed), With<RobotToken>>,
    ) {
        for (mut position, direction, speed) in query.iter_mut() {
            let diff = speed.0 * time.delta_seconds();
            match direction {
                RobotDirection::LeftMoving => position.0.x -= diff,
                RobotDirection::RightMoving => position.0.x += diff,
                _ => {},
            }
            position.0.x = position.0.x.clamp(
                Self::BOUNDARY_OFFSET_LEFT,
                Canvas::GAME_AREA_SIZE.x as f32 + Self::BOUNDARY_OFFSET_RIGHT,
            );
        }
    }

    pub fn s_draw(
        query: Query<(&Position, &RobotState, &RobotDirection), With<RobotToken>>,
        mut draw_queue: ResMut<BrpDrawQueue>,
        canvas: Canvas,
    ) {
        for (position, state, direction) in query.iter() {
            let flip = direction.is_right();

            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position),
                Sprites::RobotLeg.into(),
                flip,
            ));

            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position) + state.body_offset(),
                Sprites::RobotBody.into(),
                flip,
            ));

            let face_sprite = match state {
                RobotState::Good => Sprites::RobotFace1,
                RobotState::Tired => Sprites::RobotFace2,
                RobotState::VeryTired => Sprites::RobotFace3,
            };
            let flip_offset = match flip {
                true => ivec2(8, 0),
                false => IVec2::ZERO,
            };
            draw_queue.enqueue(BrpDrawCommand::Sprite(
                canvas.xy_of_position_within_game_area(*position)
                    + state.body_offset()
                    + flip_offset,
                face_sprite.into(),
                flip,
            ));
        }
    }
}

pub struct Robot;

impl Robot {
    pub fn collider_rect_for(
        pile_of_chickens: &PileOfChickens,
        robot_state: &RobotState,
        robot_direction: &RobotDirection,
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
        let collider_rect = Rect {
            left_top: ivec2(-8, top) + robot_state.body_offset(),
            size: uvec2(17, height),
        };
        if robot_direction.is_right() {
            collider_rect.move_by(ivec2(-1, 0))
        } else {
            collider_rect
        }
    }
}
