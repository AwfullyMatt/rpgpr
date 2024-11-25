use std::{
    fs::File,
    io::{self, Result},
};

use bevy::prelude::*;
use log::info;
use ron::{de::from_reader, ser::to_writer};
use serde::{Deserialize, Serialize};

use crate::{area::Area, AppState};

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn name(&self) -> &str {
        "Save Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(SavedArea(Area::forest()))
            .add_event::<Save>()
            .add_systems(OnEnter(AppState::Exit), evw_save)
            .add_systems(Update, evr_save);
    }
}

pub trait Saveable {
    fn save(&self, filename: &str) -> Result<()>;
    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Event)]
pub struct Save;

#[derive(Resource, Default, Deserialize, Serialize)]
pub struct SavedArea(pub Area);
impl Saveable for SavedArea {
    fn save(&self, filename: &str) -> Result<()> {
        let path = format!("{}/ron/{}", env!("CARGO_MANIFEST_DIR"), filename);
        let file = File::create(path)?;
        to_writer(file, self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let file = File::open(filename)?;
        from_reader(file).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

fn evr_save(mut evr_save: EventReader<Save>, area: ResMut<SavedArea>) {
    for _ev in evr_save.read() {
        info!("[EVENT] [READ] Save Game");
        let _ = area.save("saved_area.ron");
    }
}

fn evw_save(mut evw_save: EventWriter<Save>) {
    evw_save.send(Save);
    info!("[EVENT] [WRITE] Save Game.");
}
