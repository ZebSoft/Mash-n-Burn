use bevy::prelude::*;
use rand::{Rng, distributions::Uniform, prelude::Distribution};

use crate::entities::*;


pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut position: Query<&mut Transform, With<Street>>,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
) {
    for mut transform in position.iter_mut() {
        transform.translation = transform.translation
            + Vec3::new(0.0, 0.0, 1.0) * game.street_speed * time.delta_seconds();
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
