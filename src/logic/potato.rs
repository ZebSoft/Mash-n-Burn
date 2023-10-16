use bevy::{audio::VolumeLevel, prelude::*};
use rand::Rng;

use crate::{entities::*, GameState};

pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Potato>>,
    game: Res<Game>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut transform) in position.iter_mut() {
        transform.translation = transform.translation
            + Vec3::new(0.0, 0.0, 1.0) * game.street_speed * time.delta_seconds();

        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }

        if transform.translation.z >= 0.5f32 && game.street_speed > 3.0f32 {
            // With a 0.50% chance, play a sound when passing by a potato
            if rng.gen_bool(0.005f64) {
                let random_passing_sound = rng.gen_range(1..=9);

                commands.spawn(AudioBundle {
                    source: asset_server.load(format!("audio/Passing{random_passing_sound}.ogg")),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Once,
                        volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.6)),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut position: Query<(Entity, &mut Transform, &mut Potato), With<Potato>>,
    player_position: Query<(&Transform, With<Player>), Without<Potato>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let (player_transfrom, _) = player_position.single();
    for (entity, mut transform, mut potato) in position.iter_mut() {
        potato.has_been_alive_for += time.delta_seconds();
        transform.translation.y = 0.5f32 + potato.has_been_alive_for.sin() / 5.0f32;

        if (transform.translation.x - player_transfrom.translation.x).abs() < 0.2 {
            if (transform.translation.z - player_transfrom.translation.z).abs() < 0.4 {
                commands.entity(entity).despawn_recursive();

                if potato.is_sweet_potato {
                    // Reduce the counter by half of it's size
                    score.mash_meter_counter -= 2;
                    score.mash_meter_counter = score.mash_meter_counter.max(0);

                    commands.spawn(AudioBundle {
                        source: asset_server.load(format!("audio/SweetPotato.ogg")),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            volume: bevy::audio::Volume::Relative(VolumeLevel::new(3.0)),
                            ..default()
                        },
                        ..default()
                    });
                } else {
                    score.mash_meter_counter += 1;

                    if score.mash_meter_counter > 10 {
                        next_state.set(GameState::GameOver)
                    }

                    let rnd = rng.gen_range(1..=5);

                    commands.spawn(AudioBundle {
                        source: asset_server.load(format!("audio/Hit{rnd}.ogg")),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.6)),
                            ..default()
                        },
                        ..default()
                    });
                }
            }
        }
    }
}
