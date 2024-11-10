use bevy::prelude::*;

use crate::{
    character::CharacterBundle, loading::SpriteAssets, AppState, CHARACTER_LAYER, CHARACTER_RATIO,
    PLAYER_X,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLoot>()
            .add_systems(OnEnter(AppState::Playing), (spawn_player, spawn_loot_info))
            .add_systems(Update, update_loot_info.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PlayerLoot(pub i32);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    character: CharacterBundle,
}

#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Component, Clone, Default)]
pub struct LootCountText;

pub fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        PlayerBundle::default(),
        SpriteBundle {
            texture: sprite_assets.old_man.clone(),
            transform: Transform {
                translation: Vec3::new(PLAYER_X, 0., CHARACTER_LAYER),
                scale: Vec3::splat(CHARACTER_RATIO),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn spawn_loot_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PixelifySans-Regular.ttf");
    let style = TextStyle {
        font,
        font_size: 100.,
        ..default()
    };
    commands.spawn(Text2dBundle {
        text: Text::from_section("LOOT: ", style.clone()).with_justify(JustifyText::Center),
        transform: Transform::from_xyz(-800., -400., 5.),
        ..default()
    });

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("", style).with_justify(JustifyText::Right),
            transform: Transform::from_xyz(-650., -400., 5.),
            ..default()
        },
        LootCountText,
    ));
}

pub fn update_loot_info(
    mut query_text: Query<&mut Text, With<LootCountText>>,
    player_loot: Res<PlayerLoot>,
) {
    if player_loot.is_changed() {
        for mut text in query_text.iter_mut() {
            text.sections[0].value = player_loot.0.to_string();
        }
    }
}
