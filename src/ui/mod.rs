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
            .add_systems(Update, menu_button_system.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_main_menu);
    }
}

// NOTE: THESE ARE NORMALIZED 0.0-1.0
const BUTTON_BACKGROUND_COLOR_DEFAULT: Color = Color::srgb(0.2235, 0.2235, 0.2235);
const BUTTON_BACKGROUND_COLOR_PRESSED: Color = Color::srgb(0.6549, 0.6549, 0.6549);
const BUTTON_BACKGROUND_COLOR_HOVERED: Color = Color::srgb(0.7804, 0.7804, 0.7804);

const BUTTON_BORDER_COLOR_DEFAULT: Color = Color::srgb(0.0, 0.0, 0.0);
const BUTTON_BORDER_COLOR_PRESSED: Color = Color::srgb(0.0, 0.0, 0.0);
const BUTTON_BORDER_COLOR_HOVERED: Color = Color::srgb(0.0, 0.0, 0.0);

const BUTTON_TEXT_COLOR_DEFAULT: Color = Color::srgb(0.7804, 0.7804, 0.7804);
const BUTTON_TEXT_COLOR_PRESSED: Color = Color::srgb(1.0, 1.0, 1.0);
const BUTTON_TEXT_COLOR_HOVERED: Color = Color::srgb(0.3529, 0.3529, 0.3529);

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
    let parent_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceEvenly,
        ..default()
    };
    let child_node = Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    commands.spawn(parent_node).with_children(|parent| {
        for i in 0..3 {
            let text: Text = match i {
                0 => Text::new("PLAY"),
                1 => Text::new("SETTINGS"),
                _ => Text::new("EXIT"),
            };
            parent
                .spawn((
                    Button,
                    MainMenuButton::Play,
                    child_node.clone(),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(BUTTON_BACKGROUND_COLOR_DEFAULT),
                ))
                .with_child((
                    text,
                    TextFont {
                        font: asset_server.load("fonts/PixelifySans-Regular.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(BUTTON_TEXT_COLOR_DEFAULT),
                ));
        }
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

fn menu_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &MainMenuButton,
        ),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut text_color_query: Query<&mut TextColor>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color, mut border_color, children, mmb) in
        &mut interaction_query
    {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *text_color = BUTTON_TEXT_COLOR_PRESSED.into();
                *background_color = BUTTON_BACKGROUND_COLOR_PRESSED.into();
                border_color.0 = BUTTON_BORDER_COLOR_PRESSED.into();

                match mmb {
                    MainMenuButton::Play => {
                        if current_state.get() == &AppState::Menu {
                            next_state.set(AppState::Playing);
                        }
                    }
                    MainMenuButton::Settings => {
                        if current_state.get() == &AppState::Menu {
                            next_state.set(AppState::Settings);
                        }
                    }
                    MainMenuButton::Exit => {
                        if current_state.get() == &AppState::Menu {
                            next_state.set(AppState::Exit);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *text_color = BUTTON_TEXT_COLOR_HOVERED.into();
                *background_color = BUTTON_BACKGROUND_COLOR_HOVERED.into();
                border_color.0 = BUTTON_BORDER_COLOR_HOVERED.into();
            }
            Interaction::None => {
                *text_color = BUTTON_TEXT_COLOR_DEFAULT.into();
                *background_color = BUTTON_BACKGROUND_COLOR_DEFAULT.into();
                border_color.0 = BUTTON_BORDER_COLOR_DEFAULT.into();
            }
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
