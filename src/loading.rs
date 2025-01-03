use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn name(&self) -> &str {
        "Loading Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Menu)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("dynamic.assets.ron")
                .load_collection::<BackgroundAssets>()
                .load_collection::<CharacterAssets>()
                .load_collection::<ItemAssets>()
                .load_collection::<IconAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(key = "character_old_man_0")]
    pub character_old_man_0: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct BackgroundAssets {
    #[asset(key = "background_forest_0")]
    pub background_forest_0: Handle<Image>,
    #[asset(key = "background_forest_1")]
    pub background_forest_1: Handle<Image>,
    #[asset(key = "background_forest_2")]
    pub background_forest_2: Handle<Image>,
    #[asset(key = "background_forest_3")]
    pub background_forest_3: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(key = "item_money_0")]
    pub item_money_0: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct IconAssets {
    #[asset(key = "icon_heart_0")]
    pub icon_heart_0: Handle<Image>,
}
