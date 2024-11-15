mod background;
mod character;
mod encounter;
mod loading;
mod menu;
mod player;

use background::BackgroundPlugin;
use bevy::{
    prelude::*,
    window::{WindowResized, WindowTheme},
};
use character::CharacterPlugin;
use encounter::EncounterPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "RPGPR".to_string(),
                        canvas: Some("#rpgpr".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        window_theme: Some(WindowTheme::Dark),
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
        app.add_plugins((
            MenuPlugin,
            LoadingPlugin,
            BackgroundPlugin,
            PlayerPlugin,
            CharacterPlugin,
            EncounterPlugin,
        ));
        app.insert_resource(Resolutions::default());
        app.insert_resource(Msaa::Off);
        app.insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1)));
        app.init_resource::<SpawnLocations>();
        app.init_state::<AppState>();
        app.add_sub_state::<GameState>();
        app.add_systems(Startup, (spawn_camera, set_initial_resolution));
        app.add_systems(Update, initialize_spawn_locations);
    }
}

// GLOBAL RESOURCES

#[derive(Resource)]
pub struct Resolutions {
    sd: Vec2,  // 480p
    hd: Vec2,  // 1080p
    uhd: Vec2, // 2160p
}
impl Resolutions {
    fn default() -> Self {
        Resolutions {
            sd: Vec2::new(640., 480.),
            hd: Vec2::new(1920., 1080.),
            uhd: Vec2::new(3840., 2160.),
        }
    }
}

#[derive(Resource, Default)]
pub struct SpawnLocations {
    characters: [Vec3; 3],
    backgrounds: [Vec3; 9],
    encounters: [Vec3; 3],
    despawns: [f32; 2],
}

#[derive(States, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Playing,
    Settings,
}

#[derive(SubStates, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[source(AppState = AppState::Playing)]
pub enum GameState {
    #[default]
    Home,
    Combat,
}

// SETUP

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        IsDefaultUiCamera,
    ));
}

fn set_initial_resolution(mut query_window: Query<&mut Window, Changed<Window>>) {
    if let Ok(mut window) = query_window.get_single_mut() {
        window.resolution.set(1920., 1080.);
        info!("[MODIFIED] Window Resolution : 1080p");
    }
}

fn initialize_spawn_locations(
    mut evr_window_resized: EventReader<WindowResized>,
    mut spawn_locations: ResMut<SpawnLocations>,
) {
    for ev in evr_window_resized.read() {
        let x = ev.width;
        let middle = 0.;
        let lane_one = 0.;
        let lane_two = 200.;
        let lane_three = -200.;
        let character_one = Vec3::new(middle - (x / 2.) + 200., lane_one, CHARACTER_LAYER);
        let character_two = Vec3::new(middle - (x / 2.) + 200., lane_two, CHARACTER_LAYER);
        let character_three = Vec3::new(middle - (x / 2.) + 200., lane_three, CHARACTER_LAYER);
        let characters_array = [character_one, character_two, character_three];
        spawn_locations.characters = characters_array;
        let background_gap = 320.;
        let backgrounds_one = Vec3::new(middle, middle, BACKGROUND_LAYER);
        let backgrounds_two = Vec3::new(middle - background_gap, middle, BACKGROUND_LAYER);
        let backgrounds_three = Vec3::new(middle - (background_gap * 2.), middle, BACKGROUND_LAYER);
        let backgrounds_four = Vec3::new(middle - (background_gap * 3.), middle, BACKGROUND_LAYER);
        let backgrounds_five = Vec3::new(middle + background_gap, middle, BACKGROUND_LAYER);
        let backgrounds_six = Vec3::new(middle + (background_gap * 2.), middle, BACKGROUND_LAYER);
        let backgrounds_seven = Vec3::new(middle + (background_gap * 3.), middle, BACKGROUND_LAYER);
        let backgrounds_eight = Vec3::new(middle + (background_gap * 4.), middle, BACKGROUND_LAYER);
        let backgrounds_nine = Vec3::new(
            middle + (background_gap * 4.5) - 2., // 2 pixels underlay to prevent bg gaps
            middle,
            BACKGROUND_LAYER,
        );
        let backgrounds_array = [
            backgrounds_one,
            backgrounds_two,
            backgrounds_three,
            backgrounds_four,
            backgrounds_five,
            backgrounds_six,
            backgrounds_seven,
            backgrounds_eight,
            backgrounds_nine,
        ];
        spawn_locations.backgrounds = backgrounds_array;
        let encounter_one = Vec3::new(x + 100., lane_one, ENCOUNTER_LAYER);
        let encounter_two = Vec3::new(x + 100., lane_two, ENCOUNTER_LAYER);
        let encounter_three = Vec3::new(x + 100., lane_three, ENCOUNTER_LAYER);
        let encounter_array = [encounter_one, encounter_two, encounter_three];
        spawn_locations.encounters = encounter_array;
        let despawn_left = -(ev.width / 2.) - (background_gap / 2.);
        let despawn_right = (ev.width * 2.) + (background_gap / 2.);
        let despawn_array = [despawn_left, despawn_right];
        spawn_locations.despawns = despawn_array;
        info!("[INITIALIZED] [RESOURCE] Spawn Locations");
    }
}

// GLOBAL CONSTANTS

pub const CHARACTER_SCALE: f32 = 8.;
pub const CHARACTER_LAYER: f32 = 2.;
pub const ENCOUNTER_SCALE: f32 = 4.;
pub const ENCOUNTER_LAYER: f32 = 1.;
pub const BACKGROUND_SCALE: f32 = 5.;
pub const BACKGROUND_LAYER: f32 = 0.;
