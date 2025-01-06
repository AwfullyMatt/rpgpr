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
            /* .add_systems(
                Update,
                menu_button_interaction.run_if(in_state(AppState::Menu)),
            ) */
            .add_systems(OnExit(AppState::Menu), despawn_main_menu);
    }
}

// NOTE: THESE ARE NORMALIZED 0.0-1.0
const BUTTON_BACKGROUND_COLOR_DEFAULT: Color = Color::srgb(0.6549, 0.6549, 0.6549);
const BUTTON_BORDER_COLOR_DEFAULT: Color = Color::srgb(0.2235, 0.2235, 0.2235);
const BUTTON_TEXT_COLOR_DEFAULT: Color = Color::srgb(0.2235, 0.2235, 0.2235);

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
    let parent_node: Node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };
    let child_node: Node = Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    commands.spawn(parent_node).with_children(|parent| {
        parent
            .spawn((
                Button,
                MainMenuButton::Play,
                child_node,
                BorderColor(BUTTON_BORDER_COLOR_DEFAULT),
                BorderRadius::MAX,
                BackgroundColor(BUTTON_BACKGROUND_COLOR_DEFAULT),
            ))
            .with_child((
                Text::new("PLAY"),
                TextFont {
                    font: asset_server.load("fonts/PixelifySans-Regular.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(BUTTON_TEXT_COLOR_DEFAULT),
            ));
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

fn menu_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match interaction {
            //TODO: Redo button/state logic to match Bevy 15
            Interaction::Pressed => {}
            Interaction::Hovered => {}
            Interaction::None => {}
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
