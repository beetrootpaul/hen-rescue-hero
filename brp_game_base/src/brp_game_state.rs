use bevy::prelude::*;

#[derive(States, Default, PartialEq, Eq, Hash, Clone, Debug)]
pub enum BrpGameState {
    #[default]
    Loading,
    InGame,
    #[cfg(debug_assertions)]
    DebugPause,
    #[cfg(debug_assertions)]
    DebugPauseResumeFor1Frame,
}

pub struct BrpGameStateEcs;

impl BrpGameStateEcs {
    pub fn c_is_game_loaded(current_state: Res<State<BrpGameState>>) -> bool {
        !matches!(*current_state, State(BrpGameState::Loading))
    }
}
