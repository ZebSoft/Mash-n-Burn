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
            // You can't switch lanes any further
            if game.car_target_x == 0.0f32 && keyboard_input.just_pressed(KeyCode::Left) || game.car_target_x == 2.0f32  && keyboard_input.just_pressed(KeyCode::Right) {
                continue;
            }

            game.stationary_since = 0.0f32;
            game.rotating_since = 0.0f32;

            let rnd: i32 = rng.gen_range(1..=3);

            commands.spawn(AudioBundle {
                source: asset_server.load(format!("audio/Tyre{rnd}.ogg")),
                settings: PlaybackSettings::ONCE,
                ..default()
            });

            if keyboard_input.just_pressed(KeyCode::Left){
                game.car_direction = CarDirection::Left;
                game.car_target_x -= 1.0f32;
            }

            if keyboard_input.just_pressed(KeyCode::Right){
                game.car_direction = CarDirection::Right;
                game.car_target_x += 1.0f32;
            }

            game.car_target_x = game.car_target_x.clamp(0.0f32, 2.0f32);
        }
        
        if matches!(game.car_direction, CarDirection::Left) || matches!(game.car_direction, CarDirection::Right)
        {
            game.rotating_since += time.delta_seconds() * game.rotation_speed;
            
            // Hack for lerp, it lerps values kinda weird ( it finishes around 180 ms ? ), or I don't have the capacity to understand right now.
            if game.rotating_since >= 0.15f32 {
                game.car_direction = CarDirection::Center;
                continue;
            }

            let target_position = Vec3::new(game.car_target_x, game.car_position.y, game.car_position.z);
            let target_rotation = if matches!(game.car_direction, CarDirection::Left) { left_rotation } else { right_rotation };

            game.car_position = game.car_position.lerp(target_position, game.rotating_since * 1.1f32);
            game.car_rotation = game.car_rotation.lerp(target_rotation, game.rotating_since);
        }

        // Rotate towards center
        if matches!(game.car_direction, CarDirection::Center) {
            game.stationary_since += time.delta_seconds() * game.rotation_speed;
            game.stationary_since = game.stationary_since.clamp(0.0f32, 1.0f32);
            
            game.car_rotation = game.car_rotation.lerp(center_rotation, game.stationary_since);
        }

        transform.rotation = game.car_rotation;
        transform.translation = game.car_position;
    }
}
