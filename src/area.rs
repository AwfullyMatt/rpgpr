use std::{fmt::Display, fs::File};

use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use crate::{Chance, Weighting};

pub struct AreaPlugin;
impl Plugin for AreaPlugin {
    fn name(&self) -> &str {
        "AreaPlugin"
    }

    fn build(&self, app: &mut App) {
        app.init_resource::<Areas>()
            .init_resource::<CurrentArea>()
            .init_resource::<CurrentAreaSet>()
            .add_systems(Startup, setup);
    }
}

fn setup(mut areas: ResMut<Areas>) {
    *areas = Areas::init();
}

#[derive(Resource, Default)]
pub struct CurrentArea(pub Area);

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

pub fn ev_set_area(
    mut evr_set_area: EventReader<SetArea>,
    mut current_area: ResMut<CurrentArea>,
    areas: Res<Areas>,
) {
    for ev in evr_set_area.read() {
        current_area.0 = match ev.0 {
            AreaKind::Default => Area::default(),
            AreaKind::Forest => match areas.0.iter().find(|a| a.kind == ev.0) {
                Some(area) => area.clone(),
                None => Area::default(),
            },
            _ => Area::default(),
        };
    }
}
