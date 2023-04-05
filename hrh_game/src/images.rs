use brp_game_base::{BrpAssetPath, BrpImageAssets};

pub struct Images;

impl Images {
    pub const SPRITE_SHEET: BrpAssetPath = "sprite_sheet.png";
}

impl From<Images> for BrpImageAssets {
    fn from(_images: Images) -> Self {
        let mut brp_image_assets = BrpImageAssets::default();

        brp_image_assets.add(Images::SPRITE_SHEET);

        brp_image_assets
    }
}
