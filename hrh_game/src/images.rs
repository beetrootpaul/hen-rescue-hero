use brp_game_base::{BrpAssetPath, BrpImageAssets};

pub struct Images;

impl Images {
    pub const FONT: BrpAssetPath = "font.png";
    pub const SPRITE_SHEET: BrpAssetPath = "sprite_sheet.png";
    pub const OVERHEATED_BG: BrpAssetPath = "overheated_bg.png";
}

impl From<Images> for BrpImageAssets {
    fn from(_images: Images) -> Self {
        let mut brp_image_assets = BrpImageAssets::default();
        brp_image_assets.add(Images::FONT);
        brp_image_assets.add(Images::SPRITE_SHEET);
        brp_image_assets.add(Images::OVERHEATED_BG);
        brp_image_assets
    }
}
