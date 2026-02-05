use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{NUMBER_OF_STARS, STAR_SIZE, components::Star, resources::StarSpawnTimer};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * (window.width() - STAR_SIZE) + STAR_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - STAR_SIZE) + STAR_SIZE / 2.0;

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/star.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Star,
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.is_finished() {
        let window = window_query.single().unwrap();

        let random_x = random::<f32>() * (window.width() - STAR_SIZE) + STAR_SIZE / 2.0;
        let random_y = random::<f32>() * (window.height() - STAR_SIZE) + STAR_SIZE / 2.0;

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/star.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Star,
        ));
    }
}
