use std::{fmt::Display, fs::File};

use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use crate::Title;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Characters::init());
    }
}

#[derive(Resource, Clone, Default, Deserialize, Serialize)]
pub struct Characters(pub Vec<CharacterBundle>);
impl Characters {
    fn init() -> Self {
        let input_path = format!("{}/ron/characters.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(input_path.clone()).expect("Failed opening file");
        let characters: Characters = match from_reader(f) {
            Ok(x) => {
                info!("[INITIALIZED] Characters: {}", x);
                x
            }
            Err(e) => {
                eprintln!("[ERROR] Could not deserialize {}. \n{}", input_path, e);
                Self::default()
            }
        };
        characters
    }
}
impl Display for Characters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for character in self.0.iter() {
            string.push_str(&character.title);
            string.push_str(&", ");
        }

        write!(f, "{}", string)
    }
}

#[derive(Bundle, Clone, Deserialize, Serialize)]
pub struct CharacterBundle {
    pub character: Character,
    pub title: Title,
    pub constitution: Constitution,
    pub strength: Strength,
    pub dexterity: Dexterity,
    pub intelligence: Intelligence,
    pub wisdom: Wisdom,
    pub luck: Luck,
}
impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            character: Character,
            title: Title::default(),
            constitution: Constitution::default(),
            strength: Strength::default(),
            dexterity: Dexterity::default(),
            intelligence: Intelligence::default(),
            wisdom: Wisdom::default(),
            luck: Luck::default(),
        }
    }
}

#[derive(Component, Clone, Deserialize, Serialize)]
pub struct Character;

#[derive(Component, Clone, Default, Deserialize, Serialize)]
pub struct Constitution(pub Level);

#[derive(Component, Clone, Default, Deserialize, Serialize)]
pub struct Strength(pub Level);
#[derive(Component, Clone, Default, Deserialize, Serialize)]
pub struct Dexterity(pub Level);
#[derive(Component, Clone, Default, Deserialize, Serialize)]
pub struct Intelligence(pub Level);
#[derive(Component, Clone, Default, Deserialize, Serialize)]
pub struct Wisdom(pub Level);
#[derive(Component, Clone, Default, Deserialize, Serialize)]
pub struct Luck(pub Level);
#[derive(Component, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Level(pub i32);
