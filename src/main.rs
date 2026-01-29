use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub const CHARACTERS_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_message::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_entity)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Confined {
    size: f32,
}

#[derive(Component)]
pub struct Star;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Message)]
pub struct GameOver {
    pub score: u32,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/ball_blue_large.png")),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player,
        Confined {
            size: CHARACTERS_SIZE,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * (window.width() - CHARACTERS_SIZE) + CHARACTERS_SIZE / 2.0;
        let random_y =
            random::<f32>() * (window.height() - CHARACTERS_SIZE) + CHARACTERS_SIZE / 2.0;

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
            Confined {
                size: CHARACTERS_SIZE,
            },
        ));
    }
}

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

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
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

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    let half_enemy_size = CHARACTERS_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;

        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn(AudioPlayer::new(sound_effect));
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_message_writer: MessageWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = CHARACTERS_SIZE / 2.0;
            let enemy_radius = CHARACTERS_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");

                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioPlayer::new(sound_effect));
                commands.entity(player_entity).despawn();

                game_over_message_writer.write(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < CHARACTERS_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star!");

                score.value += 1;

                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn(AudioPlayer::new(sound_effect));
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
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

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.is_finished() {
        let window = window_query.single().unwrap();

        let random_x = random::<f32>() * (window.width() - CHARACTERS_SIZE) + CHARACTERS_SIZE / 2.0;
        let random_y =
            random::<f32>() * (window.height() - CHARACTERS_SIZE) + CHARACTERS_SIZE / 2.0;

        commands.spawn((
            Sprite::from_image(asset_server.load("sprites/ball_red_large.png")),
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
            Confined {
                size: CHARACTERS_SIZE,
            },
        ));
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

pub fn update_high_scores(
    mut game_over_message_reader: MessageReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_message_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
