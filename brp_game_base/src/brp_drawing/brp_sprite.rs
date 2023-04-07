use bevy::math::IVec2;
use bevy::utils::HashMap;
use BrpColor;
use {BrpAssetPath, Rect};

pub struct BrpSprite {
    pub image_path: BrpAssetPath,
    //
    // source_rect.left_top + anchor --> a point to be used for positioning
    pub source_rect: Rect,
    pub anchor: IVec2,
    //
    pub color_replacements: HashMap<BrpColor, BrpColor>,
}
