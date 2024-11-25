use std::{fmt::Display, fs::File};

use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use crate::{AppState, Weighting};

pub struct AreaPlugin;
impl Plugin for AreaPlugin {
    fn name(&self) -> &str {
        "AreaPlugin"
    }

    fn build(&self, app: &mut App) {
        app.init_resource::<Areas>()
            .init_resource::<CurrentArea>()
            .init_resource::<CurrentAreaSet>()
            .add_systems(Startup, setup)
            .add_systems(Update, evr_set_area.run_if(in_state(AppState::Playing)));
    }
}

fn setup(mut areas: ResMut<Areas>) {
    *areas = Areas::init();
}

#[derive(Resource, Default)]
pub struct CurrentArea(pub Area);
impl CurrentArea {}

#[derive(Resource, Default)]
pub struct CurrentAreaSet(pub Vec<Area>);

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Areas(pub Vec<Area>);
impl Areas {
    fn init() -> Self {
        let input_path = format!("{}/ron/areas.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(input_path.clone()).expect("Failed opening file");
        let areas: Areas = match from_reader(f) {
            Ok(x) => {
                info!("[INITIALIZED] Areas: {}", x);
                x
            }
            Err(e) => {
                eprintln!("[ERROR] Could not deserialize {}. \n{}", input_path, e);
                Areas::default()
            }
        };
        areas
    }
}
impl Display for Areas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for area in self.0.iter() {
            string.push_str(&area.name);
            string.push_str(&", ");
        }

        write!(f, "{}", string)
    }
}

#[derive(Component, Clone, Serialize, Deserialize, Default)]
pub struct Area {
    pub name: String,
    pub kind: AreaKind,
    pub weighting_loot: Weighting,
    pub weighting_enemy: Weighting,
}
impl Area {
    pub fn forest() -> Self {
        Area {
            name: "Default Forest".to_string(),
            kind: AreaKind::Forest,
            weighting_loot: Weighting::default(),
            weighting_enemy: Weighting::default(),
        }
    }
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AreaKind {
    #[default]
    Default,
    Forest,
    Desert,
    Swamp,
}

#[derive(Event)]
pub struct SetArea(pub AreaKind);

pub fn evr_set_area(
    mut evr_set_area: EventReader<SetArea>,
    mut current_area: ResMut<CurrentArea>,
    mut current_area_set: ResMut<CurrentAreaSet>,
    areas: Res<Areas>,
) {
    for ev in evr_set_area.read() {
        current_area_set.0.clear();
        for area in areas.0.iter() {
            if area.kind == ev.0 {
                current_area_set.0.push(area.clone());
            }
        }
        current_area.0 = current_area_set
            .0
            .first()
            .expect("[ERROR] Invalid Area Set")
            .clone();
    }
}
