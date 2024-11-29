use std::{fmt::Display, fs::File};

use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn name(&self) -> &str {
        "Weapon Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(Weapons::init());
    }
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Weapons(pub Vec<Weapon>);
impl Weapons {
    fn init() -> Self {
        let input_path = format!("{}/ron/weapons.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(input_path.clone()).expect("Failed opening file");
        let weapons: Weapons = match from_reader(f) {
            Ok(x) => {
                info!("[INITIALIZED] Weapons: {}", x);
                x
            }
            Err(e) => {
                eprintln!("[ERROR] Could not deserialize {}. \n{}", input_path, e);
                Weapons::default()
            }
        };
        weapons
    }
}
impl Display for Weapons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for weapon in self.0.iter() {
            string.push_str(&weapon.title);
            string.push_str(&", ");
        }

        write!(f, "{}", string)
    }
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub title: String,
    pub kind: WeaponKind,
    pub weight: WeaponWeight,
    pub hand: WeaponHand,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum WeaponKind {
    #[default]
    DEFAULT,
    CANE,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum WeaponWeight {
    #[default]
    DEFAULT,
    FEATHER,
    LIGHT,
    MIDDLE,
    HEAVY,
    ULTRA,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum WeaponHand {
    #[default]
    ONE,
    TWO,
    SPECIAL,
}
