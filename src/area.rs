use std::{fmt::Display, fs::File};

use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

pub struct AreaPlugin;
impl Plugin for AreaPlugin {
    fn name(&self) -> &str {
        "AreaPlugin"
    }

    fn build(&self, app: &mut App) {
        app.init_resource::<Areas>()
            .init_resource::<CurrentArea>()
            .add_systems(Startup, setup);
    }
}

fn setup(mut areas: ResMut<Areas>) {
    *areas = Areas::init();
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentArea(pub Area);

#[derive(Resource, Default, Deref, DerefMut, Serialize, Deserialize)]
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

#[derive(Component, Serialize, Deserialize, Default, Deref, DerefMut)]
pub struct Area {
    pub name: String,
    #[deref]
    pub kind: AreaKind,
    pub chance_loot: Chance,
    pub chance_enemy: Chance,
}

#[derive(Default, Serialize, Deserialize)]
pub enum AreaKind {
    #[default]
    Default,
    Forest,
    Desert,
    Swamp,
}

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Serialize, Deserialize)]
pub struct Chance(pub f32);
