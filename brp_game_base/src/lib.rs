extern crate bevy;
extern crate bevy_pixels;

pub use brp_color::BrpColor;
pub use brp_drawing::BrpSprite;
pub use brp_drawing::{BrpDrawCommand, BrpDrawQueue};
pub use brp_game_base::BrpGameBase;
pub use brp_game_config::BrpGameConfig;
pub use brp_game_state::BrpGameState;
pub use rect::{rect, Rect};

pub use crate::brp_assets::{BrpAssetPath, BrpImageAssets};

pub type BrpSystemSet = bevy_pixels::PixelsSet;

mod brp_assets;
mod brp_color;
mod brp_drawing;
mod brp_game_base;
mod brp_game_config;
mod brp_game_state;
mod rect;
