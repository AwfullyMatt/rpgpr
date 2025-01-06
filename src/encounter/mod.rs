use crate::{
    character::Character,
    loading::{CharacterAssets, ItemAssets},
    player::PlayerLoot,
    AppState, SpawnLocations, CHARACTER_LAYER, CHARACTER_SCALE, ENCOUNTER_LAYER, ENCOUNTER_SCALE,
};
use bevy::prelude::*;
use std::fmt::Display;

pub struct EncounterPlugin;
impl Plugin for EncounterPlugin {
    fn name(&self) -> &str {
        "Encounter Plugin"
    }

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
    pub lane: usize,
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
    spawn_locations: Res<SpawnLocations>,
    character_assets: Res<CharacterAssets>,
    item_assets: Res<ItemAssets>,
    mut evr_spawn_encounter: EventReader<SpawnEncounter>,
) {
    for ev in evr_spawn_encounter.read() {
        let x = spawn_locations.encounters[ev.lane].x;
        let (texture, scale, z) = match ev.kind {
            EncounterKind::Combat => (
                character_assets.character_old_man_0.clone(),
                CHARACTER_SCALE,
                CHARACTER_LAYER,
            ),
            EncounterKind::Loot => (
                item_assets.item_money_0.clone(),
                ENCOUNTER_SCALE,
                ENCOUNTER_LAYER,
            ),
        };
        let transform = Transform {
            translation: Vec3::new(x, 0., z),
            scale: Vec3::splat(scale),
            ..default()
        };
        let encounter = Encounter { kind: ev.kind };
        let entity = commands
            .spawn((Sprite::from_image(texture), transform, encounter))
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
    spawn_locations: Res<SpawnLocations>,
) {
    for (entity, loot, tf) in query_encounter.iter() {
        if tf.translation.x <= spawn_locations.characters[0].x {
            commands.entity(entity).despawn_recursive();
            info!("[DESPAWNED] Encounter: {}", loot.kind);
            **player_loot += 1;
            info!("[COLLECTED] Loot: {}", loot.kind);
        }
    }
}
