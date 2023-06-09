use bevy::prelude::*;

use brp_game_base::BrpGameState;
use collider::Collider;
#[cfg(debug_assertions)]
use collider::CollidersDebugConfig;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;
use input::InputMode;

pub struct ControlsEcs;

impl ControlsEcs {
    const LEFT_STICK_THRESHOLD: f32 = 0.5;
    const RIGHT_STICK_THRESHOLD: f32 = 0.5;

    #[allow(clippy::too_many_arguments)]
    pub fn s_handle_input(
        #[cfg(debug_assertions)] mut colliders_debug_config: ResMut<CollidersDebugConfig>,
        keyboard_input: Res<Input<KeyCode>>,
        gamepads: Res<Gamepads>,
        gamepad_button_inputs: Res<Input<GamepadButton>>,
        gamepad_axis: Res<Axis<GamepadAxis>>,
        input_mode: Res<InputMode>,
        current_state: Res<State<BrpGameState>>,
        mut next_state: ResMut<NextState<BrpGameState>>,
        mut q_robot: Query<(&mut Robot, &mut Collider, &PileOfChickens)>,
    ) {
        // "c" = toggle debug draw of [c]olliders
        #[cfg(debug_assertions)]
        if keyboard_input.just_pressed(KeyCode::C) {
            colliders_debug_config.is_debug_draw_enabled =
                !colliders_debug_config.is_debug_draw_enabled;
        }

        if !input_mode.is_input_blocked() {
            let mut left =
                keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
            let mut right =
                keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

            for gamepad in gamepads.iter() {
                let left_stick_x = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
                let left_stick_x_value = gamepad_axis.get(left_stick_x).unwrap_or(0.0);

                let right_stick_x = GamepadAxis::new(gamepad, GamepadAxisType::RightStickX);
                let right_stick_x_value = gamepad_axis.get(right_stick_x).unwrap_or(0.0);

                let dpad_left = GamepadButton::new(gamepad, GamepadButtonType::DPadLeft);
                let dpad_right = GamepadButton::new(gamepad, GamepadButtonType::DPadRight);

                left = left
                    || left_stick_x_value < -Self::LEFT_STICK_THRESHOLD
                    || right_stick_x_value < -Self::RIGHT_STICK_THRESHOLD
                    || gamepad_button_inputs.pressed(dpad_left);
                right = right
                    || left_stick_x_value > Self::LEFT_STICK_THRESHOLD
                    || right_stick_x_value > Self::RIGHT_STICK_THRESHOLD
                    || gamepad_button_inputs.pressed(dpad_right);
            }

            match current_state.0 {
                BrpGameState::Loading => {},
                BrpGameState::Menu => {
                    if left || right {
                        next_state.set(BrpGameState::InGame);
                    }
                },
                BrpGameState::InGame
                | BrpGameState::DebugPause
                | BrpGameState::DebugPauseResumeFor1Frame => {
                    for (mut robot, mut robot_collider, pile_of_chicken) in q_robot.iter_mut() {
                        robot.update_direction(left, right);
                        robot_collider.rect = robot.collider_rect_for(pile_of_chicken);
                    }
                },
            }
        }
    }
}
