use bevy::prelude::*;

use crate::AppState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dummy_system)
            .add_systems(OnEnter(AppState::Menu), spawn_main_menu)
            .add_systems(Update, escape_to_menu)
            .add_systems(Update, menu_button_press.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_main_menu);
    }
}

#[derive(Component, Clone, Copy)]
pub struct CleanupMainMenu;

#[derive(Component, Clone, Copy)]
pub enum MainMenuButton {
    Play,
    Settings,
    Exit,
}

fn dummy_system() {}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(CleanupMainMenu)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .insert(MainMenuButton::Play)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .insert(MainMenuButton::Settings)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .insert(MainMenuButton::Exit)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                    ));
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
    mut ev_app_exit: EventWriter<AppExit>,
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
                    ev_app_exit.send(AppExit::Success);
                    info!("[EXIT] Gracefully Exiting App");
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
