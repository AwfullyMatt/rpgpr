use crate::AppState;
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn name(&self) -> &str {
        "Menu Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(Update, escape_to_menu.run_if(in_state(AppState::Settings)))
            .add_systems(Update, menu_button_press.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_main_menu);
    }
}

// attached to all main menu components for cleanup
#[derive(Component, Clone, Copy)]
pub struct CleanupMainMenu;

#[derive(Component, Clone, Copy)]
pub enum MainMenuButton {
    Play,
    Settings,
    Exit,
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let node_style = Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };
    let button_style = Style {
        width: Val::Px(200.),
        height: Val::Px(50.),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_bundle = ButtonBundle {
        style: button_style.clone(),
        border_color: BorderColor(Color::BLACK),
        border_radius: BorderRadius::MAX,
        background_color: Color::WHITE.into(),
        ..default()
    };
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.,
        color: Color::BLACK,
    };
    commands
        .spawn(NodeBundle {
            style: node_style.clone(),
            ..default()
        })
        .insert(CleanupMainMenu)
        .with_children(|parent| {
            parent
                .spawn(button_bundle.clone())
                .insert(MainMenuButton::Play)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", text_style.clone()));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(button_bundle.clone())
                .insert(MainMenuButton::Settings)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Settings", text_style.clone()));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(button_bundle.clone())
                .insert(MainMenuButton::Exit)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Exit", text_style.clone()));
                });
        });

    info!("[SPAWNED] Main Menu Entities");
}

fn despawn_main_menu(
    mut commands: Commands,
    mut query_main_menu: Query<Entity, With<CleanupMainMenu>>,
) {
    for entity in query_main_menu.iter_mut() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Main Menu Entities.");
    }
}

fn menu_button_press(
    mut query_button_interaction: Query<
        (&Interaction, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mmb) in &mut query_button_interaction {
        match interaction {
            Interaction::Pressed => match mmb {
                MainMenuButton::Play => {
                    next_state.set(AppState::Playing);
                    info!("[MODIFIED] Appstate >> Playing");
                }
                MainMenuButton::Settings => {
                    next_state.set(AppState::Settings);
                    info!("[MODIFIED] Appstate >> Settings");
                }
                MainMenuButton::Exit => {
                    next_state.set(AppState::Exit);
                    info!("[MODIFIED] Appstate >> Exit");
                }
            },
            _ => {}
        }
    }
}

fn escape_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            AppState::Settings => {
                next_state.set(AppState::Menu);
                info!("[MODIFIED] Appstate >> Settings");
            }
            _ => {}
        }
    }
}
