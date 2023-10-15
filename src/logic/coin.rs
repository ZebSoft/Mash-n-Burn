use bevy::prelude::*;
use rand::Rng;

use crate::entities::*;

pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Coin>>,
    game: Res<Game>,
) {
    for (entity, mut transform) in position.iter_mut() {
        transform.translation = transform.translation
            + Vec3::new(0.0, 0.0, 1.0) * game.street_speed * time.delta_seconds();
        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    position: Query<(Entity, &Transform), With<Coin>>,
    player_position: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    car_sound_controller: Query<&AudioSink, With<CarSoundMarker>>,
) {
    let mut rng = rand::thread_rng();
    let player_transfrom = player_position.single();
    for (entity, transform) in position.iter() {
        if (transform.translation.x - player_transfrom.translation.x).abs() < 0.2 {
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
