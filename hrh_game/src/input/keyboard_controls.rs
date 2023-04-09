use bevy::prelude::*;

use collider::Collider;
#[cfg(debug_assertions)]
use collider::CollidersDebugConfig;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::{Robot, RobotDirection, RobotState, RobotToken};

pub struct KeyboardControlsEcs;

type RobotColliderRelatedParts<'a, 'b, 'c> = (&'a mut Collider, &'b PileOfChickens);

impl KeyboardControlsEcs {
    pub fn s_handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<
            (
                &mut RobotDirection,
                &RobotState,
                Option<RobotColliderRelatedParts>,
            ),
            With<RobotToken>,
        >,
        #[cfg(debug_assertions)] mut config: ResMut<CollidersDebugConfig>,
    ) {
        let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

        for (mut direction, robot_state, robot_collider_related_parts) in query.iter_mut() {
            *direction = match (robot_state, left, right) {
                (RobotState::Overheated, _, _) => direction.to_staying(),
                (_, true, false) => RobotDirection::LeftMoving,
                (_, false, true) => RobotDirection::RightMoving,
                (_, _, _) => direction.to_staying(),
            };
            if let Some((mut robot_collider, pile_of_chickens)) = robot_collider_related_parts {
                robot_collider.rect =
                    Robot::collider_rect_for(pile_of_chickens, robot_state, direction.as_ref());
            }
        }

        // "c" = toggle debug draw of [c]olliders
        #[cfg(debug_assertions)]
        if keyboard_input.just_pressed(KeyCode::C) {
            config.is_debug_draw_enabled = !config.is_debug_draw_enabled;
        }
    }
}
