use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{ distributions::Uniform, prelude::Distribution, Rng };

use crate::{ entities::*, scene, GameState };

pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    mut position: Query<(Entity, &mut Transform), With<Obstacle>>,
    game: Res<Game>
) {
    for (entity, mut transform) in position.iter_mut() {
        transform.translation =
            transform.translation +
            Vec3::new(0.0, 0.0, 1.0) * game.obstacle_speed * time.delta_seconds();
        if transform.translation.z >= 1.0 {
            commands.entity(entity).despawn_recursive();
            //println!("despawn");
        }
    }
}

pub fn check_collision(
    _commands: Commands,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    position: Query<(Entity, &Transform), With<Obstacle>>,
    player_position: Query<&Transform, With<Player>>
) {
    let player_transfrom = player_position.single();
    for (_entity, transform) in position.iter() {
        if (transform.translation.x - player_transfrom.translation.x).abs() < 0.7 {
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

pub fn spawn_obstacle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..3);
    let ran_street = die.sample(&mut rng);

    let model = scene::OBSTACLE_MODELS[rng.gen_range(0..scene::OBSTACLE_MODELS.len())];
    commands
        .spawn(SceneBundle {
            scene: asset_server.load(model),
            transform: Transform {
                translation: Vec3::new(ran_street as f32, 0.0, -24.0),
                scale: Vec3::new(0.4, 0.4, 0.4),
                rotation: Quat::from_rotation_y(PI),
            },
            ..default()
        })
        .insert(Obstacle);
}

