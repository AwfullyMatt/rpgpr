use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Menu)
                .load_collection::<SpriteAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/old_man.png")]
    pub old_man: Handle<Image>,

    #[asset(path = "sprites/tree.png")]
    pub tree: Handle<Image>,

    #[asset(path = "sprites/loot_money.png")]
    pub loot_money: Handle<Image>,
}
