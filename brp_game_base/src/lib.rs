extern crate bevy;
extern crate bevy_pixels;

pub use color::BrpColor;
pub use drawing::{BrpDrawCommand, BrpDrawQueue};
pub use game_base::BrpGameBase;
pub use game_config::BrpGameConfig;
pub use game_state::BrpGameState;
pub use rect::{rect, Rect};

pub use crate::assets::{BrpAssetPath, BrpImageAssets};

pub type BrpSystemSet = bevy_pixels::PixelsSet;

mod assets;
mod color;
mod drawing;
mod game_base;
mod game_config;
mod game_state;
mod rect;
