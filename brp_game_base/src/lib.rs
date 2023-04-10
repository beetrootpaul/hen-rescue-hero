extern crate bevy;
extern crate bevy_pixels;
#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

pub use brp_color::BrpColor;
pub use brp_drawing::BrpSprite;
pub use brp_drawing::{BrpCanvasVariant, BrpCurrentCanvasVariant};
pub use brp_drawing::{BrpDrawCommand, BrpDrawQueue};
pub use brp_font::{BrpFontConfig, BrpFontGlyph};
pub use brp_game_base::BrpGameBase;
pub use brp_game_config::BrpGameConfig;
pub use brp_game_state::{BrpGameState, BrpGameStateEcs};
pub use rect::{rect, Rect};

pub use crate::brp_assets::{BrpAssetPath, BrpImageAssets};

pub type BrpSystemSet = bevy_pixels::PixelsSet;

mod brp_assets;
mod brp_color;
#[cfg(debug_assertions)]
mod brp_debug;
mod brp_drawing;
mod brp_font;
mod brp_game_base;
mod brp_game_config;
mod brp_game_state;
mod brp_input;
mod rect;
