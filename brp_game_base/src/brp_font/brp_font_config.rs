use bevy::math::{ivec2, IVec2, UVec2};
use bevy::prelude::Resource;
use bevy::utils::HashMap;

use brp_font::BrpFontGlyph;
use Rect;
use {BrpAssetPath, BrpColor};

#[derive(Resource)]
pub struct BrpFontConfig {
    pub image_path: Option<BrpAssetPath>,
    pub glyph_size: UVec2,
    pub glyph_jump_to_next: IVec2,
    pub source_color_font: BrpColor,
    pub source_color_transparent_1: Option<BrpColor>,
    pub source_color_transparent_2: Option<BrpColor>,
    pub glyph_to_source_rect: HashMap<BrpFontGlyph, Rect>,
}

impl BrpFontConfig {
    pub fn rect_of(&self, text: &str) -> IVec2 {
        ivec2(
            self.glyph_jump_to_next.x * text.len() as i32,
            self.glyph_jump_to_next.y,
        )
    }
}
