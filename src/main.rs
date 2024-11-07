use bevy::app::{App, AppExit};
use rpgpr::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}

