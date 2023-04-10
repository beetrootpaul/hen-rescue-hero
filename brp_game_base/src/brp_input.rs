use bevy::app::{App, Plugin};
use bevy::prelude::Resource;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn __is_touch_available__() -> bool;
}

#[derive(Resource)]
pub struct BrpInputConfig {
    pub is_touch_available: bool,
}

pub struct BrpInputPlugin;

impl Plugin for BrpInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BrpInputConfig {
            #[cfg(not(target_arch = "wasm32"))]
            is_touch_available: false,
            #[cfg(target_arch = "wasm32")]
            is_touch_available: __is_touch_available__(),
        });
    }
}
