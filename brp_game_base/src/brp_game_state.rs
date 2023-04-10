use bevy::prelude::*;

#[derive(States, Default, PartialEq, Eq, Hash, Clone, Debug)]
pub enum BrpGameState {
    #[default]
    Loading,
    Menu,
    InGame,
    DebugPause,
    DebugPauseResumeFor1Frame,
}

pub struct BrpGameStateEcs;

impl BrpGameStateEcs {
    pub fn c_is_in_menu(current_state: Res<State<BrpGameState>>) -> bool {
        matches!(*current_state, State(BrpGameState::Menu))
    }
    pub fn c_is_game_loaded(current_state: Res<State<BrpGameState>>) -> bool {
        !matches!(*current_state, State(BrpGameState::Loading))
    }
}
