use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn name(&self) -> &str {
        "Settings Plugin"
    }

    fn build(&self, app: &mut App) {
        app.insert_resource(Resolutions::init())
            .insert_resource(Settings::load());
    }
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Settings {
    pub resolution: Vec2,
    pub monitor: usize,
}
impl Settings {
    fn load() -> Self {
        let input_path = format!("{}/ron/settings.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(input_path.clone()).expect("Failed opening file");
        let settings: Settings = match from_reader(f) {
            Ok(x) => {
                info!("[INITIALIZED] Settings");
                x
            }
            Err(e) => {
                eprintln!("[ERROR] Could not deserialize {}. \n{}", input_path, e);
                Self::default()
            }
        };
        settings
    }
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct Resolutions {
    sd: Vec2,  // 480p
    hd: Vec2,  // 1080p
    uhd: Vec2, // 2160p
}
impl Resolutions {
    fn init() -> Self {
        Resolutions {
            sd: Vec2::new(640., 480.),
            hd: Vec2::new(1920., 1080.),
            uhd: Vec2::new(3840., 2160.),
        }
    }
}
