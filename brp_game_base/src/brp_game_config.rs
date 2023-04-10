use bevy::prelude::*;

use BrpColor;

#[derive(Resource, Clone)]
pub struct BrpGameConfig {
    pub title: String,
    pub canvas_margin_color: BrpColor,
    pub square_canvas_size: UVec2,
    pub landscape_canvas_size: UVec2,
    pub portrait_canvas_size: UVec2,
    #[cfg(not(target_arch = "wasm32"))]
    pub initial_canvas_zoom: u32,
    #[cfg(target_arch = "wasm32")]
    pub html_canvas_selector: String,
}
