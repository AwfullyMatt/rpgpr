use bevy::prelude::*;

use crate::{
    character::CharacterBundle, loading::CharacterAssets, AppState, SpawnLocations, CHARACTER_SCALE,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLoot>()
            .add_event::<SpawnPlayer>()
            .add_systems(
                OnEnter(AppState::Playing),
                (evw_spawn_player, spawn_money_info),
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
pub struct MoneyCountText;

#[derive(Event, Deref)]
pub struct SpawnPlayer(pub usize);

pub fn evr_spawn_player(
    mut commands: Commands,
    mut evr_spawn_player: EventReader<SpawnPlayer>,
    character_assets: Res<CharacterAssets>,
    spawn_locations: Res<SpawnLocations>,
) {
    for ev in evr_spawn_player.read() {
        commands.spawn((
            PlayerBundle::default(),
            Sprite::from_image(character_assets.character_old_man_0.clone()),
            Transform {
                translation: spawn_locations.characters[**ev],
                scale: Vec3::splat(CHARACTER_SCALE),
                ..default()
            },
            // BEVY 15 MIGRATION
            //SpriteBundle {
            //texture: character_assets.character_old_man_0.clone(),
            //transform: Transform {
            //translation: spawn_locations.characters[**ev],
            //scale: Vec3::splat(CHARACTER_SCALE),
            //..default()
            //},
            //..default()
            //},
        ));
        info!("[EVENT] [READ] SpawnPlayer({})", **ev);
    }
}

fn evw_spawn_player(mut evw_spawn_player: EventWriter<SpawnPlayer>) {
    evw_spawn_player.send(SpawnPlayer(0));
    info!("[EVENT] [WRITE] SpawnPlayer({})", 0);
}

pub fn spawn_money_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Money: "),
        TextFont {
            font: asset_server.load("fonts/PixelifySans-Regular.ttf"),
            font_size: 100.,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            ..default()
        },
        MoneyCountText,
    ));
    // BEVY 15 MIGRATION
    //let font = asset_server.load("fonts/PixelifySans-Regular.ttf");
    // let style = TextStyle {
    //     font,
    //     font_size: 100.,
    //     ..default()
    // };
    // commands.spawn(Text2dBundle {
    //     text: Text::from_section("LOOT: ", style.clone()).with_justify(JustifyText::Center),
    //     transform: Transform::from_xyz(-800., -400., 5.),
    //     ..default()
    // });
    //
    // commands.spawn((
    //     Text2dBundle {
    //         text: Text::from_section("", style).with_justify(JustifyText::Right),
    //         transform: Transform::from_xyz(-650., -400., 5.),
    //         ..default()
    //     },
    //     LootCountText,
    // ));
}

pub fn update_loot_info(
    mut query_text: Query<&mut TextSpan, With<MoneyCountText>>,
    player_loot: Res<PlayerLoot>,
) {
    if player_loot.is_changed() {
        for mut text in &mut query_text {
            **text = format!("{}", **player_loot);
        }
    }
}
