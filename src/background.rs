use bevy::prelude::*;

use crate::{loading::SpriteAssets, AppState, BG_RATIO};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundCount>()
            .add_event::<SpawnBackground>()
            .add_systems(OnEnter(AppState::Playing), spawn_initial_backgrounds)
            .add_systems(
                Update,
                (move_backgrounds, evr_spawn_background, despawn_background)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Resource, Default)]
pub struct BackgroundCount(pub i32);

#[derive(Component, Default)]
pub struct Background {
    pub id: i32,
}

#[derive(Event)]
pub struct SpawnBackground;

pub fn spawn_initial_backgrounds(mut evw_spawn_background: EventWriter<SpawnBackground>) {
    for _i in 0..7 {
        evw_spawn_background.send(SpawnBackground);
        info!("[WRITE] Spawn Background.");
    }
}

pub fn evr_spawn_background(
    mut evr_spawn_background: EventReader<SpawnBackground>,
    query_background: Query<(&Background, &Transform)>,
    mut bg_count: ResMut<BackgroundCount>,
    sprite_assets: Res<SpriteAssets>,
    mut commands: Commands,
) {
    for _ev in evr_spawn_background.read() {
        if bg_count.0 < 7 {
            for _i in 0..7 {
                let x: f32 = match bg_count.0 {
                    0 => -800.,
                    1 => -480.,
                    2 => -160.,
                    3 => 160.,
                    4 => 480.,
                    5 => 800.,
                    _ => 1120.,
                };
                commands.spawn((
                    SpriteBundle {
                        texture: sprite_assets.tree.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, 0., 1.),
                            scale: Vec3::splat(BG_RATIO),
                            ..default()
                        },
                        ..default()
                    },
                    Background { id: bg_count.0 },
                ));
                info!("[SPAWNED] Background ID: {}", bg_count.0);
                bg_count.0 += 1;
            }
        } else {
            for (bg, tf) in query_background.iter() {
                if bg.id == bg_count.0 - 1 {
                    let x: f32 = tf.translation.x + 319.; // 1 pixel overlap
                    commands.spawn((
                        SpriteBundle {
                            texture: sprite_assets.tree.clone(),
                            transform: Transform {
                                translation: Vec3::new(x, 0., 1.),
                                scale: Vec3::splat(BG_RATIO),
                                ..default()
                            },
                            ..default()
                        },
                        Background { id: bg_count.0 },
                    ));
                    info!("[SPAWNED] Background ID: {}", bg_count.0);
                    bg_count.0 += 1;
                } else {
                    continue;
                }
            }
        }
    }
}

pub fn move_backgrounds(mut query_background: Query<(&Background, &mut Transform)>) {
    for (_bg, mut tf) in query_background.iter_mut() {
        tf.translation.x -= 1.;
    }
}

pub fn despawn_background(
    mut commands: Commands,
    mut evw_spawn_background: EventWriter<SpawnBackground>,
    query_background: Query<(Entity, &Background, &Transform)>,
) {
    for (entity, background, transform) in query_background.iter() {
        if transform.translation.x < -1120. {
            commands.entity(entity).despawn_recursive();
            info!("[DESPAWN] Background ID: {}", background.id);
            evw_spawn_background.send(SpawnBackground);
        }
    }
}
