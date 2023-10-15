use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::{Rng, distributions::Uniform, prelude::Distribution};

use crate::entities::*;


pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut position: Query<&mut Transform, With<Street>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
                
                let potato_index = rng.gen_range(0..=12);

                // If the index is 0, it's our sweet potato priest!!!
                let file_name = if potato_index == 0 { format!("images/SweetPotato.png") } else { format!("images/Potato{potato_index}.png") };

                let material = StandardMaterial { 
                    alpha_mode: AlphaMode::Blend,
                    base_color_texture: Some(asset_server.load(file_name)),
                    ..default()
                };
    
                commands
                    .spawn(
                        PbrBundle {
                            mesh: meshes.add(shape::Plane::from_size(1.0).into()),
                            transform: Transform {
                                translation: Vec3::new(ran_street as f32, 0.5, transform.translation.z),
                                rotation: Quat::from_euler(EulerRot::XYZ, FRAC_PI_2 - FRAC_PI_2 / 5.0f32, 0.0f32, 0.0f32),
                                scale: Vec3::new(0.5, 0.5, 0.5),
                                ..Default::default()
                            },
                            material: materials.add(material),
                            ..default()
                        })
                    .insert(Potato { is_sweet_potato: potato_index == 0, ..default() });
            }
        }
    }
}
