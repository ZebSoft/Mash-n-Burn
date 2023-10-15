use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::entities::{Besttext, Coin, Cointext, Obstacle, Player, Score, Street, Game, CarSoundMarker};
use crate::{scene, GameState};

pub fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    for mut transform in player.iter_mut() {

        let mut isTurning: bool = keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::Right);

        if keyboard_input.pressed(KeyCode::Left) {

            let mut x = transform.translation.x - 0.04;

            if x < 0.0 {
                x = 0.0
            };

            transform.translation = Vec3::new(x, transform.translation.y, transform.translation.z);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0f32, 45.0f32, 0.0f32);
        }

        if keyboard_input.pressed(KeyCode::Right) {

            let mut x = transform.translation.x + 0.04;
            if x > 2.0 {
                x = 2.0
            };

            transform.translation = Vec3::new(x, transform.translation.y, transform.translation.z);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0f32, -45.0f32, 0.0f32);
        }
        

        if !isTurning && (keyboard_input.just_released(KeyCode::Left) || keyboard_input.just_released(KeyCode::Right)) {
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0f32, 0.0f32, 0.0f32);
        }

        if isTurning {
            let rnd: i32 = rng.gen_range(1..=3);

            commands.spawn(AudioBundle {
                source: asset_server.load(format!("audio/Tyre{rnd}.ogg")),
                settings: PlaybackSettings::ONCE,
                ..default()
            });
        }
    }
}

pub fn move_street(
    mut commands: Commands,
    time: Res<Time>,
    mut position: Query<&mut Transform, With<Street>>,
    asset_server: Res<AssetServer>,
    game: Res<Game>
) {
    for mut transform in position.iter_mut() {
        transform.translation =
            transform.translation + Vec3::new(0.0, 0.0, 1.0) * game.street_speed * time.delta_seconds();
        if transform.translation.z > 2.0 {
            transform.translation.z -= 23.0;
            let mut rng = rand::thread_rng();
            let ran_ = rng.gen_range(0..10);
            if ran_ < 2 {
                let die = Uniform::from(0..3);
                let ran_street = die.sample(&mut rng);
                commands
                    .spawn(SceneBundle {
                            scene: asset_server.load("models/coin.glb#Scene0"),
                            transform: Transform {
                                translation: Vec3::new(ran_street as f32, 0.0, transform.translation.z),
                                scale: Vec3::new(0.5, 0.5, 0.5),
                                ..Default::default()
                            },
                            ..default()
                        })
                    .insert(Coin);
            }
        }
    }
}

pub fn move_coin(
    time: Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Coin>>,
    game: Res<Game>
) {
    for (entity, mut transform) in position.iter_mut() {
        transform.translation =
            transform.translation + Vec3::new(0.0, 0.0, 1.0) * game.street_speed * time.delta_seconds();
        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn collision_coin(
    mut commands: Commands,
    mut score: ResMut<Score>,
    position: Query<(Entity, &Transform), With<Coin>>,
    player_position: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    car_sound_controller: Query<&AudioSink, With<CarSoundMarker>>
) {
    let mut rng = rand::thread_rng();
    let player_transfrom = player_position.single();
    for (entity, transform) in position.iter() {
        if transform.translation.x == player_transfrom.translation.x {
            if (transform.translation.z - player_transfrom.translation.z).abs() < 0.4 {
                commands.entity(entity).despawn_recursive();
                score.value += 1;

                let rnd = rng.gen_range(1..=5);

                commands.spawn(AudioBundle {
                    source: asset_server.load(format!("audio/Hit{rnd}.ogg")),
                    settings: PlaybackSettings::ONCE,
                    ..default()
                });
                
                game.obstacle_speed *= 1.1f32;
                game.street_speed *= 1.1f32;
                game.engine_speed *= 1.01f32;
                
                if let Ok(sink) = car_sound_controller.get_single() {
                    sink.set_speed(game.engine_speed);
                }
            }
        }
    }
}

pub fn scoreboard(
    score: Res<Score>,
    mut coin_query: Query<(&mut Text, With<Cointext>, Without<Besttext>)>,
    mut best_query: Query<&mut Text, With<Besttext>>,
) {
    let (mut text, _, _) = coin_query.single_mut();
    text.sections[0].value = format!("Coin: {}", score.value);

    let mut best_text = best_query.single_mut();
    best_text.sections[0].value = format!("Best: {}", score.best);
}

pub fn spawn_obstacle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..3);
    let ran_street = die.sample(&mut rng);

    let model = scene::OBSTACLE_MODELS[rng.gen_range(0..scene::OBSTACLE_MODELS.len())];
    commands
        .spawn(SceneBundle {
                scene: asset_server.load(model),
                transform:Transform {
                    translation: Vec3::new(ran_street as f32, 0.0, -10.0),
                    scale: Vec3::new(0.4, 0.4, 0.4),
                    rotation: Quat::from_rotation_y(PI),
                },
                ..default()
        })
        .insert(Obstacle);
}

pub fn move_obstacle(
    time: Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Obstacle>>,
    game: Res<Game>
) {
    for (entity, mut transform) in position.iter_mut() {
        transform.translation = transform.translation
            + Vec3::new(0.0, 0.0, 1.0) * game.obstacle_speed * time.delta_seconds();
        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
            //println!("despawn");
        }
    }
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn show_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "press key to restart",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                        ..default()
                    },
                ),
                ..default()
            });
        });
}

pub fn gameover_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
    
    next_state.set(GameState::Playing);
}

pub fn collision_obstacle(
    _commands: Commands,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    position: Query<(Entity, &Transform), With<Obstacle>>,
    player_position: Query<&Transform, With<Player>>,
) {
    let player_transfrom = player_position.single();
    for (_entity, transform) in position.iter() {
        if transform.translation.x == player_transfrom.translation.x {
            if (transform.translation.z - player_transfrom.translation.z).abs() < 0.4 {
                next_state.set(GameState::GameOver);

                if score.value > score.best {
                    score.best = score.value;
                    score.value = 0;
                }

                return;
            }
        }
    }
}
