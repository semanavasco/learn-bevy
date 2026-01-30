use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;
use crate::events::*;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn confine_entity(
    mut entity_query: Query<(&mut Transform, &Confined)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut entity_transform, confined) in entity_query.iter_mut() {
        let window = window_query.single().unwrap();

        let half_size = confined.size / 2.0;
        let x_max = window.width() - half_size;
        let y_max = window.height() - half_size;

        let mut translation = entity_transform.translation;

        translation.x = translation.x.clamp(half_size, x_max);
        translation.y = translation.y.clamp(half_size, y_max);

        entity_transform.translation = translation;
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(mut game_over_message_reader: MessageReader<GameOver>) {
    for event in game_over_message_reader.read() {
        println!("Your final score is: {}", event.score);
    }
}
