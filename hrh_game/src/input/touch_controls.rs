use bevy::prelude::*;

use brp_game_base::{BrpDrawingInfo, BrpGameState};
use canvas::Canvas;
use collider::Collider;
#[cfg(debug_assertions)]
use collider::CollidersDebugConfig;
use game_objects::pile_of_chickens::PileOfChickens;
use game_objects::robot::Robot;
use input::InputMode;

pub struct TouchControlsEcs;

impl TouchControlsEcs {
    pub fn s_handle_touch_input(
        input_mode: Res<InputMode>,
        touches: Res<Touches>,
        brp_drawing_info: Res<BrpDrawingInfo>,
        // mut touch_buttons_query: Query<(&mut TouchButton, &Position)>,
        // current_state: Res<State<BrpGameState>>,
        // mut next_state: ResMut<NextState<BrpGameState>>,
        // mut q_robot: Query<(&mut Robot, &mut Collider, &PileOfChickens)>,
    ) {
        if !input_mode.is_input_blocked() {
            for touch in touches.iter() {
                let touch_xy =
                    brp_drawing_info.real_viewport_xy_to_canvas_xy(touch.position().as_ivec2());

                warn!("{}x{}", touch_xy.x, touch_xy.y);
            }

            // let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
            // let right =
            //     keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);
            //
            // match current_state.0 {
            //     BrpGameState::Loading => {},
            //     BrpGameState::Menu => {
            //         if left || right {
            //             next_state.set(BrpGameState::InGame);
            //         }
            //     },
            //     BrpGameState::InGame
            //     | BrpGameState::DebugPause
            //     | BrpGameState::DebugPauseResumeFor1Frame => {
            //         for (mut robot, mut robot_collider, pile_of_chicken) in q_robot.iter_mut() {
            //             robot.update_direction(left, right);
            //             robot_collider.rect = robot.collider_rect_for(pile_of_chicken);
            //         }
            //     },
            // }
        }
    }
}
