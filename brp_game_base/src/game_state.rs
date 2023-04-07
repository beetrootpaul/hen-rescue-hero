use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Hash, Clone, Debug, Default)]
pub enum BrpGameState {
    #[default]
    Loading,
    InGame,
}
