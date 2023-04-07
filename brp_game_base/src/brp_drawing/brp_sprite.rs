use bevy::utils::HashMap;
use BrpColor;
use {BrpAssetPath, Rect};

pub struct BrpSprite {
    pub image_path: BrpAssetPath,
    pub source_rect: Rect,
    pub color_replacements: HashMap<BrpColor, BrpColor>,
}
