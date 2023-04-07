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
