use crate::{chance::Weightings, AppState, Title};
use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File};

pub struct AreaPlugin;
impl Plugin for AreaPlugin {
    fn name(&self) -> &str {
        "Area Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(Areas::init())
            .init_resource::<CurrentArea>()
            .init_resource::<CurrentAreaSet>()
            .add_event::<SetArea>()
            .add_systems(Update, evr_set_area.run_if(in_state(AppState::Playing)));
    }
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
            string.push_str(&area.title);
            string.push_str(&", ");
        }

        write!(f, "{}", string)
    }
}

#[derive(Component, Clone, Serialize, Deserialize, Default)]
pub struct Area {
    pub title: Title,
    pub kind: AreaKind,
    pub weightings: Weightings,
}
impl Area {
    pub fn forest() -> Self {
        Area {
            title: Title("Default Area".to_string()),
            kind: AreaKind::default(),
            weightings: Weightings::default(),
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
