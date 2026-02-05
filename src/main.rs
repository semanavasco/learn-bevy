mod components;
mod events;
mod game;
mod main_menu;
pub mod sets;
mod systems;

use bevy::prelude::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use sets::MovementSet;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, confine_entity_movement.after(MovementSet))
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
