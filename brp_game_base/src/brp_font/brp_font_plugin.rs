use bevy::math::{ivec2, uvec2};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use brp_font::brp_font_config::BrpFontConfig;
use BrpColor;

pub struct BrpFontPlugin;

impl Plugin for BrpFontPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BrpFontConfig {
            image_path: None,
            glyph_size: uvec2(3, 5),
            glyph_jump_to_next: ivec2(4, 0),
            source_color_font: BrpColor::Solid {
                r: 0xff,
                g: 0xff,
                b: 0xff,
            },
            source_color_transparent_1: None,
            source_color_transparent_2: None,
            glyph_to_source_rect: HashMap::new(),
        });
    }
}
