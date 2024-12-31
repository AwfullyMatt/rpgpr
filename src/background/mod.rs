use bevy::prelude::*;

use crate::{
    area::{AreaKind, CurrentArea},
    loading::SpriteAssets,
    AppState, SpawnLocations, BACKGROUND_SCALE,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBackground>()
            .add_systems(OnEnter(AppState::Playing), spawn_initial_backgrounds)
            .add_systems(
                Update,
                (move_backgrounds, evr_spawn_background, despawn_background)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Background;

#[derive(Event, Deref)]
pub struct SpawnBackground(pub usize);

pub fn spawn_initial_backgrounds(mut evw_spawn_background: EventWriter<SpawnBackground>) {
    // TODO: the number of bgs/screen should be determined by screen size
    let events = vec![
        SpawnBackground(0),
        SpawnBackground(1),
        SpawnBackground(2),
        SpawnBackground(3),
        SpawnBackground(4),
        SpawnBackground(5),
        SpawnBackground(6),
        SpawnBackground(7),
    ];
    evw_spawn_background.send_batch(events);
}

pub fn evr_spawn_background(
    mut evr_spawn_background: EventReader<SpawnBackground>,
    spawn_locations: Res<SpawnLocations>,
    sprite_assets: Res<SpriteAssets>,
    current_area: Res<CurrentArea>,
    mut commands: Commands,
    //areas: Res<Areas>,
) {
    for ev in evr_spawn_background.read() {
        use AreaKind::*;

        commands.spawn((
            SpriteBundle {
                texture: match current_area.0.kind {
                    Forest => sprite_assets.forest_0.clone(),
                    _ => sprite_assets.forest_1.clone(),
                },
                transform: Transform {
                    translation: spawn_locations.backgrounds[**ev],
                    scale: Vec3::splat(BACKGROUND_SCALE),
                    ..default()
                },
                ..default()
            },
            Background,
        ));
        info!("[EVENT] [READ] SpawnBackground({})", **ev);
    }
}

pub fn move_backgrounds(mut query_background: Query<(&Background, &mut Transform)>) {
    for (_bg, mut tf) in query_background.iter_mut() {
        tf.translation.x -= 1.;
    }
}

// also sends event to spawn next background
pub fn despawn_background(
    mut commands: Commands,
    spawn_locations: Res<SpawnLocations>,
    mut evw_spawn_background: EventWriter<SpawnBackground>,
    query_background: Query<(Entity, &Transform)>,
) {
    for (entity, transform) in query_background.iter() {
        if transform.translation.x < spawn_locations.despawns[0] {
            commands.entity(entity).despawn_recursive();
            info!("[DESPAWNED] [ENTITY] Background");
            evw_spawn_background.send(SpawnBackground(8));
        }
    }
}
