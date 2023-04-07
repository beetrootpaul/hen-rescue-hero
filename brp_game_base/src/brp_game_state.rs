use bevy::prelude::*;

#[derive(States, Default, PartialEq, Eq, Hash, Clone, Debug)]
pub enum BrpGameState {
    #[default]
    Loading,
    InGame,
}
