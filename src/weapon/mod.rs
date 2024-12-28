use crate::{Damage, Title, ID};
use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File};

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
    pub id: WeaponID,
    pub title: Title,
    pub kind: WeaponKind,
    pub weight: WeaponWeight,
    pub hand: WeaponHand,
    pub damage: WeaponDamage,
}
impl Weapon {
    pub fn default() -> Self {
        Self {
            id: WeaponID::default(),
            title: Title("Default Weapon".to_string()),
            kind: WeaponKind::default(),
            weight: WeaponWeight::default(),
            hand: WeaponHand::default(),
            damage: WeaponDamage::default(),
        }
    }
}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub enum WeaponKind {
    #[default]
    DEFAULT,
    CANE,
    PACIFIER,
}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub enum WeaponWeight {
    #[default]
    DEFAULT,
    FEATHER,
    LIGHT,
    MIDDLE,
    HEAVY,
    ULTRA,
}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub enum WeaponHand {
    #[default]
    DEFAULT,
    ONE,
    TWO,
    SPECIAL,
}

#[derive(Component, Default, Clone, Serialize, Deserialize, Deref, DerefMut)]
pub struct WeaponDamage(pub Damage);

#[derive(Component, Default, Clone, Copy, Serialize, Deserialize, Deref, DerefMut)]
pub struct WeaponID(pub ID);
