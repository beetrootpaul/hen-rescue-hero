use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct BrpGameConfig {
    pub title: String,
    pub landscape_canvas_size: UVec2,
    pub portrait_canvas_size: UVec2,
    pub initial_canvas_zoom: u32,
}
