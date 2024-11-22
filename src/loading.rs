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
    // [BACKGROUNDS]
    #[asset(path = "sprites\\backgrounds\\background_forest_0.png")]
    pub forest_0: Handle<Image>,

    #[asset(path = "sprites\\backgrounds\\background_forest_1.png")]
    pub forest_1: Handle<Image>,

    #[asset(path = "sprites\\backgrounds\\background_forest_2.png")]
    pub forest_2: Handle<Image>,

    #[asset(path = "sprites\\characters\\character_old_man.png")]
    pub character_old_man: Handle<Image>,

    #[asset(path = "sprites\\items\\item_money.png")]
    pub loot_money: Handle<Image>,
}
