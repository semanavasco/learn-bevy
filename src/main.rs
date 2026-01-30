mod components;
pub mod enemy;
mod events;
mod player;
pub mod score;
pub mod sets;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

use crate::sets::MovementSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<GameOver>()
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(ScorePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, confine_entity_movement.after(MovementSet))
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
