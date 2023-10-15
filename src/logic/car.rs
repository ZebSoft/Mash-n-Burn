use bevy::prelude::*;
use rand::Rng;

use crate::{ entities::*, GameState };

pub fn update(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    time: Res<Time>
) {
    let left_rotation = Vec3 {
        x: 0.0f32,
        y: 45.0f32,
        z: 0.0f32,
    };
    let right_rotation = Vec3 {
        x: 0.0f32,
        y: -45.0f32,
        z: 0.0f32,
    };
    let center = Vec3::ZERO;

    let mut rng = rand::thread_rng();

    for mut transform in player.iter_mut() {
        if
            keyboard_input.just_pressed(KeyCode::Left) ||
            keyboard_input.just_pressed(KeyCode::Right)
        {
            game.car_direction = if keyboard_input.just_pressed(KeyCode::Left) {
                CarDirection::Left
            } else {
                CarDirection::Right
            };

            game.car_rotation = Vec3::ZERO;

            let rnd: i32 = rng.gen_range(1..=3);

            commands.spawn(AudioBundle {
                source: asset_server.load(format!("audio/Tyre{rnd}.ogg")),
                settings: PlaybackSettings::ONCE,
                ..default()
            });
        }

        if
            matches!(game.car_direction, CarDirection::Left) &&
            keyboard_input.pressed(KeyCode::Left)
        {
            let mut x = transform.translation.x - 0.03;

            if x < 0.0 {
                x = 0.0;
            }

            game.car_position = Vec3::new(x, transform.translation.y, transform.translation.z);
            game.car_rotation.lerp(left_rotation, game.car_turning_for);
        } else if
            matches!(game.car_direction, CarDirection::Right) &&
            keyboard_input.pressed(KeyCode::Right)
        {
            let mut x = transform.translation.x + 0.03;
            if x > 2.0 {
                x = 2.0;
            }

            game.car_position = Vec3::new(x, transform.translation.y, transform.translation.z);
            game.car_rotation.lerp(right_rotation, game.car_turning_for);
        } else if
            keyboard_input.just_released(KeyCode::Left) ||
            keyboard_input.just_released(KeyCode::Right)
        {
            game.car_rotation.lerp(center, game.car_going_straight_for);

            game.car_going_straight_for += time.delta_seconds() * game.rotation_speed;
            game.car_going_straight_for = game.car_going_straight_for.max(1.0f32);

            game.car_turning_for = 0.0f32;
            game.car_direction = CarDirection::Center;
        }

        if !matches!(game.car_direction, CarDirection::Center) {
            game.car_turning_for += time.delta_seconds() * game.rotation_speed;
            game.car_turning_for = game.car_turning_for.max(1.0f32);

            game.car_going_straight_for = 0.0f32;
        }

        transform.translation = game.car_position;
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            game.car_rotation.x,
            game.car_rotation.y,
            game.car_rotation.z
        );
    }
}
