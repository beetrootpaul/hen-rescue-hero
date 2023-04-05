extern crate bevy;
extern crate bevy_pixels;

pub use color::BrpColor;
pub use drawing::{BrpDrawCommand, BrpDrawQueue};
pub use game_base::BrpGameBase;
pub use game_config::BrpGameConfig;
pub use rect::{rect, Rect};

pub type BrpSystemSet = bevy_pixels::PixelsSet;

mod color;
mod drawing;
mod game_base;
mod game_config;
mod rect;
