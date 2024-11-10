use std::fmt::Display;

use bevy::prelude::*;

use crate::{
    background::{Background, BackgroundCount},
    character::Character,
    loading::SpriteAssets,
    player::PlayerLoot,
    AppState, CHARACTER_LAYER, CHARACTER_RATIO, LOOT_LAYER, LOOT_RATIO, PLAYER_X,
};

pub struct EncounterPlugin;
impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEncounter>().add_systems(
            Update,
            (move_encounter, evr_spawn_encounter, collect_loot).run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Encounter {
    pub kind: EncounterKind,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Loot {
    pub kind: LootKind,
}

#[derive(Event)]
pub struct SpawnEncounter {
    pub kind: EncounterKind,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LootKind {
    Money,
}
impl Display for LootKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Money => write!(f, "Loot"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum EncounterKind {
    Loot,
    Combat,
}
impl Display for EncounterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Loot => write!(f, "Loot"),
            Self::Combat => write!(f, "Combat"),
        }
    }
}

pub fn evr_spawn_encounter(
    mut commands: Commands,
    bg_count: Res<BackgroundCount>,
    sprite_assets: Res<SpriteAssets>,
    query_background: Query<(&Background, &Transform)>,
    mut evr_spawn_encounter: EventReader<SpawnEncounter>,
) {
    for ev in evr_spawn_encounter.read() {
        let mut x = 0.;
        for (bg, tf) in query_background.iter() {
            if bg.id == **bg_count - 1 {
                x = tf.translation.x;
            } else {
                x = 1100.
            } // ToDo - there is a race condition here
              // this is a temp fix
              // spawning will need to happen AFTER bg count is updated, ALWAYS
              // use scheduling to do so
        }
        let (texture, scale, z) = match ev.kind {
            EncounterKind::Combat => (
                sprite_assets.old_man.clone(),
                CHARACTER_RATIO,
                CHARACTER_LAYER,
            ),
            EncounterKind::Loot => (sprite_assets.loot_money.clone(), LOOT_RATIO, LOOT_LAYER),
        };
        let entity = commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(x, 0., z),
                        scale: Vec3::splat(scale),
                        ..default()
                    },
                    ..default()
                },
                Encounter { kind: ev.kind },
            ))
            .id();
        match ev.kind {
            EncounterKind::Loot => commands.entity(entity).insert(Loot {
                kind: LootKind::Money,
            }),
            EncounterKind::Combat => commands.entity(entity).insert(Character),
        };

        info!("[SPAWNED] Encounter: {}", ev.kind);
    }
}

pub fn move_encounter(mut query_encounter: Query<&mut Transform, With<Encounter>>) {
    for mut tf in query_encounter.iter_mut() {
        tf.translation.x -= 1.;
    }
}

pub fn collect_loot(
    mut commands: Commands,
    query_encounter: Query<(Entity, &Loot, &Transform)>,
    mut player_loot: ResMut<PlayerLoot>,
) {
    for (entity, loot, tf) in query_encounter.iter() {
        if tf.translation.x <= PLAYER_X {
            commands.entity(entity).despawn_recursive();
            info!("[DESPAWNED] Encounter: {}", loot.kind);
            **player_loot += 1;
            info!("[COLLECTED] Loot: {}", loot.kind);
        }
    }
}
