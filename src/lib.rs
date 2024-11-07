mod background;
mod character;
mod loading;
mod menu;
mod player;

use background::BackgroundPlugin;
use bevy::prelude::*;
use character::CharacterPlugin;
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
        ));
        app.insert_resource(Resolutions::default());
        app.insert_resource(Msaa::Off);
        app.insert_resource(ClearColor(Color::linear_rgb(0.1, 0.1, 0.1)));
        app.init_state::<AppState>();
        app.add_sub_state::<GameState>();
        app.add_systems(Startup, (spawn_camera, set_initial_resolution));
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

fn set_initial_resolution(mut query_window: Query<&mut Window>) {
    if let Ok(mut window) = query_window.get_single_mut() {
        window.resolution.set(1920., 1080.);
        info!("[MODIFIED] Window Resolution : 1080p");
    }
}

// GLOBAL CONSTANTS

pub const SPRITE_RATIO: f32 = 8.;
pub const BG_RATIO: f32 = 5.0;
