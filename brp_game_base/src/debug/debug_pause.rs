use bevy::prelude::*;

use {BrpGameState, BrpSystemSet};

pub struct DebugPausePlugin;

impl DebugPausePlugin {
    fn update_game_state(
        keyboard_input: Res<Input<KeyCode>>,
        current_state: Res<State<BrpGameState>>,
        mut next_state: ResMut<NextState<BrpGameState>>,
        mut was_resumed_for_1_frame: Local<bool>,
    ) {
        // d = enter [d]ebug pause
        if keyboard_input.just_pressed(KeyCode::D) {
            match *current_state {
                State(BrpGameState::InGame) => {
                    next_state.set(BrpGameState::DebugPause);
                },
                State(BrpGameState::DebugPause) => {
                    next_state.set(BrpGameState::InGame);
                },
                _ => {},
            };
            return;
        }
        // . = resume game for 1 frame
        if let State(BrpGameState::DebugPause) = *current_state {
            if keyboard_input.just_pressed(KeyCode::Period) {
                *was_resumed_for_1_frame = true;
                next_state.set(BrpGameState::InGame);
            }
            return;
        }

        if let State(BrpGameState::InGame) = *current_state {
            if *was_resumed_for_1_frame {
                *was_resumed_for_1_frame = false;
                next_state.set(BrpGameState::DebugPause);
            }
        }
    }
}

impl Plugin for DebugPausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::update_game_state.in_set(BrpSystemSet::Update));
    }
}
