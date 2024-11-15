use bevy::prelude::*;

use crate::{
    character::CharacterBundle, loading::SpriteAssets, AppState, SpawnLocations, CHARACTER_SCALE,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLoot>()
            .add_event::<SpawnPlayer>()
            .add_systems(
                OnEnter(AppState::Playing),
                (evw_spawn_player, spawn_loot_info),
            )
            .add_systems(
                Update,
                (evr_spawn_player, update_loot_info).run_if(in_state(AppState::Playing)),
            );
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

#[derive(Event, Deref)]
pub struct SpawnPlayer(pub usize);

pub fn evr_spawn_player(
    mut commands: Commands,
    mut evr_spawn_player: EventReader<SpawnPlayer>,
    sprite_assets: Res<SpriteAssets>,
    spawn_locations: Res<SpawnLocations>,
) {
    for ev in evr_spawn_player.read() {
        commands.spawn((
            PlayerBundle::default(),
            SpriteBundle {
                texture: sprite_assets.old_man.clone(),
                transform: Transform {
                    translation: spawn_locations.characters[**ev],
                    scale: Vec3::splat(CHARACTER_SCALE),
                    ..default()
                },
                ..default()
            },
        ));
        info!("[EVENT] [READ] SpawnPlayer({})", **ev);
    }
}

fn evw_spawn_player(mut evw_spawn_player: EventWriter<SpawnPlayer>) {
    evw_spawn_player.send(SpawnPlayer(0));
    info!("[EVENT] [WRITE] SpawnPlayer({})", 0);
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
