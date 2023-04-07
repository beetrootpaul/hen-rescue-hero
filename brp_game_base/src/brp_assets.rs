use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::HashMap;

use brp_game_state::BrpGameState;

pub type BrpAssetPath = &'static str;

#[derive(Resource, Default)]
pub struct BrpImageAssets(HashMap<BrpAssetPath, Option<Handle<Image>>>);

impl BrpImageAssets {
    pub fn add(&mut self, image_path: BrpAssetPath) {
        self.0.insert(image_path, None);
    }
    pub fn get(&self, image_path: BrpAssetPath) -> Handle<Image> {
        self.0.get(image_path)
            .expect("should have been asked for a handle for a path which was used earlier for asset loading")
            .clone()
            .expect("should have been asked for a handle after all assets are loaded")
    }
}

pub struct BrpAssetSystems;

impl BrpAssetSystems {
    pub fn start_loading(mut image_assets: ResMut<BrpImageAssets>, asset_server: Res<AssetServer>) {
        let paths: Vec<BrpAssetPath> = image_assets.0.keys().copied().collect();
        for path in paths {
            image_assets.0.insert(path, Some(asset_server.load(path)));
        }
    }

    pub fn wait_for_loading_to_complete(
        image_assets: Res<BrpImageAssets>,
        asset_server: Res<AssetServer>,
        mut next_state: ResMut<NextState<BrpGameState>>,
    ) {
        let are_images_loaded =
            image_assets
                .0
                .values()
                .all(|maybe_image_handle| match maybe_image_handle {
                    None => false,
                    Some(image_handle) => {
                        asset_server.get_load_state(image_handle.clone()) == LoadState::Loaded
                    },
                });
        if are_images_loaded {
            next_state.set(BrpGameState::InGame);
        }
    }
}
