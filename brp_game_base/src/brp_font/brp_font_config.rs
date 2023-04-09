use bevy::math::UVec2;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

use brp_font::BrpFontGlyph;
use Rect;
use {BrpAssetPath, BrpColor};

#[derive(Resource)]
pub struct BrpFontConfig {
    pub image_path: Option<BrpAssetPath>,
    pub glyph_size: UVec2,
    pub source_color_font: BrpColor,
    pub source_color_transparent_1: Option<BrpColor>,
    pub source_color_transparent_2: Option<BrpColor>,
    pub glyph_to_source_rect: HashMap<BrpFontGlyph, Rect>,
}
