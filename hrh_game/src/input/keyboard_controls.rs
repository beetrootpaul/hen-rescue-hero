use bevy::prelude::*;
use robot::{RobotDirection, RobotToken};

pub struct KeyboardControlsSystems;

impl KeyboardControlsSystems {
    pub fn handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut RobotDirection, With<RobotToken>>,
    ) {
        let left = keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::Right);

        for mut direction in query.iter_mut() {
            *direction = match (left, right) {
                (true, true) => RobotDirection::None,
                (true, false) => RobotDirection::Left,
                (false, true) => RobotDirection::Right,
                (false, false) => RobotDirection::None,
            };
        }
    }
}
