use bevy::prelude::*;

use crate::{character::CharacterBundle, loading::SpriteAssets, AppState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_player);
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    character: CharacterBundle,
}

#[derive(Component, Clone, Default)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        PlayerBundle::default(),
        SpriteBundle {
            texture: sprite_assets.old_man.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 2.),
                scale: Vec3::new(8., 8., 8.),
                ..default()
            },
            ..default()
        },
    ));
}
