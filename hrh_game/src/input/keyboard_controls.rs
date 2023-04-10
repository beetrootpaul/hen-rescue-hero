use bevy::prelude::*;

use collider::Collider;
#[cfg(debug_assertions)]
use collider::CollidersDebugConfig;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;

pub struct KeyboardControlsEcs;

impl KeyboardControlsEcs {
    pub fn s_handle_keyboard_input(
        keyboard_input: Res<Input<KeyCode>>,
        mut q_robot: Query<(&mut Robot, &mut Collider, &PileOfChickens)>,
        #[cfg(debug_assertions)] mut config: ResMut<CollidersDebugConfig>,
    ) {
        let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

        for (mut robot, mut robot_collider, pile_of_chicken) in q_robot.iter_mut() {
            robot.update_direction(left, right);
            robot_collider.rect = robot.collider_rect_for(pile_of_chicken);
        }

        // "c" = toggle debug draw of [c]olliders
        #[cfg(debug_assertions)]
        if keyboard_input.just_pressed(KeyCode::C) {
            config.is_debug_draw_enabled = !config.is_debug_draw_enabled;
        }
    }
}
