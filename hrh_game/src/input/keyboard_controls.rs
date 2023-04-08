use bevy::prelude::*;

#[cfg(debug_assertions)]
use collider::CollidersDebugConfig;
use game_objects::robot::{RobotDirection, RobotToken};

pub struct KeyboardControlsEcs;

impl KeyboardControlsEcs {
    pub fn s_handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut RobotDirection, With<RobotToken>>,
        #[cfg(debug_assertions)] mut config: ResMut<CollidersDebugConfig>,
    ) {
        let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

        for mut direction in query.iter_mut() {
            *direction = match (left, right) {
                (true, true) => RobotDirection::None,
                (true, false) => RobotDirection::Left,
                (false, true) => RobotDirection::Right,
                (false, false) => RobotDirection::None,
            };
        }

        // "c" = toggle debug draw of [c]olliders
        #[cfg(debug_assertions)]
        if keyboard_input.just_pressed(KeyCode::C) {
            config.is_debug_draw_enabled = !config.is_debug_draw_enabled;
        }
    }
}
