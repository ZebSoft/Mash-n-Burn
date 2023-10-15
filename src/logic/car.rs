use bevy::prelude::*;
use rand::Rng;

use crate::entities::*;

pub fn update(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    let left_rotation: Quat = Quat::from_euler(EulerRot::XYZ, 0.0f32, 120.0f32, 0.0f32);
    let center_rotation: Quat = Quat::from_euler(EulerRot::XYZ, 0.0f32, 0.0f32, 0.0f32);
    let right_rotation: Quat = Quat::from_euler(EulerRot::XYZ, 0.0f32, -120.0f32, 0.0f32);

    let mut rng = rand::thread_rng();

    for mut transform in player.iter_mut() {

        if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::Right)
        {
            game.stationary_since = 0.0f32;

            let rnd: i32 = rng.gen_range(1..=3);

            commands.spawn(AudioBundle {
                source: asset_server.load(format!("audio/Tyre{rnd}.ogg")),
                settings: PlaybackSettings::ONCE,
                ..default()
            });
        }

        if (!keyboard_input.pressed(KeyCode::Left) && !keyboard_input.pressed(KeyCode::Right))
        || (keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Right))
        {
            game.rotating_since = 0.0f32;
            game.car_direction = CarDirection::Center;
        }
        else if keyboard_input.pressed(KeyCode::Left) {
            game.car_direction = CarDirection::Left
        } 
        else if keyboard_input.pressed(KeyCode::Right) {
            game.car_direction = CarDirection::Right
        };
        
        if matches!(game.car_direction, CarDirection::Left)
        {
            game.rotating_since += time.delta_seconds() * game.rotation_speed;
            game.rotating_since = game.rotating_since.clamp(0.0f32, 1.0f32);

            let mut x = transform.translation.x - 0.03;

            if x < 0.0 {
                x = 0.0;
            }

            game.car_position = Vec3::new(x, transform.translation.y, transform.translation.z);
            game.car_rotation = game.car_rotation.slerp(left_rotation, game.rotating_since);

        } 
        
        if matches!(game.car_direction, CarDirection::Right)
        {
            game.rotating_since += time.delta_seconds() * game.rotation_speed;
            game.rotating_since = game.rotating_since.clamp(0.0f32, 1.0f32);
            
            let mut x = transform.translation.x + 0.03;
            if x > 2.0 {
                x = 2.0;
            }

            game.car_position = Vec3::new(x, transform.translation.y, transform.translation.z);
            game.car_rotation = game.car_rotation.slerp(right_rotation, game.rotating_since);
        }

        // Rotate towards center
        if matches!(game.car_direction, CarDirection::Center) {
            game.stationary_since += time.delta_seconds() * game.rotation_speed;
            game.stationary_since = game.stationary_since.clamp(0.0f32, 1.0f32);
            
            game.car_rotation = game.car_rotation.slerp(center_rotation, game.stationary_since);
        }

        transform.rotation = game.car_rotation;
        transform.translation = game.car_position;
        
    }
}
