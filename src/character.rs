use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system);
    }
}

fn dummy_system() {}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub character: Character,
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
            constitution: Constitution::default(),
            strength: Strength::default(),
            dexterity: Dexterity::default(),
            intelligence: Intelligence::default(),
            wisdom: Wisdom::default(),
            luck: Luck::default(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Character;

#[derive(Component, Clone, Default)]
pub struct Constitution {
    pub lvl: Level,
}

#[derive(Component, Clone, Default)]
pub struct Strength {
    pub lvl: Level,
}
#[derive(Component, Clone, Default)]
pub struct Dexterity {
    pub lvl: Level,
}
#[derive(Component, Clone, Default)]
pub struct Intelligence {
    pub lvl: Level,
}
#[derive(Component, Clone, Default)]
pub struct Wisdom {
    pub lvl: Level,
}
#[derive(Component, Clone, Default)]
pub struct Luck {
    pub lvl: Level,
}
#[derive(Component, Clone, Copy, Default)]
pub struct Level(pub i32);
