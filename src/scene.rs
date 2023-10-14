use crate::entities::{self, Besttext, Coin, Cointext, Player};

use bevy::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution};
use std::f32::consts::FRAC_PI_2;

use entities::Street;

pub const OBSTACLE_MODELS: &'static [&'static str] = &[
    "models/hatchbackSports.glb#Scene0",
    "models/police.glb#Scene0",
    "models/sedan.glb#Scene0",
    "models/tractor.glb#Scene0",
];

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 6.0, 3.0).looking_at(Vec3::new(1., 0., -2.), Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(1.0, 4.0, 0.0),
        ..default()
    });

    // street
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..3);

    for j in -9..2 {
        let mut children_list: Vec<Entity> = Vec::new();
        for i in 0..3 {
            let entity = commands
                .spawn((
                    Transform {
                        translation: Vec3::new(i as f32, 0.0, 0.0),
                        rotation: Quat::from_rotation_y(FRAC_PI_2),
                        ..Default::default()
                    },
                    GlobalTransform::IDENTITY,
                ))
                .with_children(|parent| {
                    parent.spawn(SceneBundle {
                        scene: asset_server.load("models/road_straight.glb#Scene0"),
                        ..default()
                    });
                })
                .id();
            children_list.push(entity);
            if i == 0 {
                let lamp = commands
                    .spawn((
                        Transform {
                            translation: Vec3::new(i as f32 - 0.45, 0.0, 0.0),
                            rotation: Quat::from_rotation_y(FRAC_PI_2),
                            ..Default::default()
                        },
                        GlobalTransform::IDENTITY,
                    ))
                    .with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/lamp.glb#Scene0"),
                            ..default()
                        });
                    })
                    .id();
                children_list.push(lamp);
            }
            if i == 2 {
                let lamp = commands
                    .spawn((
                        Transform {
                            translation: Vec3::new(i as f32 + 0.45, 0.0, 0.0),
                            rotation: Quat::from_rotation_y(-FRAC_PI_2),
                            ..Default::default()
                        },
                        GlobalTransform::IDENTITY,
                    ))
                    .with_children(|parent| {
                        parent.spawn(SceneBundle {
                            scene: asset_server.load("models/lamp.glb#Scene0"),
                            ..default()
                        });
                    })
                    .id();
                children_list.push(lamp);
            }
            commands
                .spawn((
                    Transform {
                        translation: Vec3::new(0.0, 0.0, j as f32),
                        ..Default::default()
                    },
                    GlobalTransform::IDENTITY,
                ))
                .insert(Street)
                .push_children(&children_list);
        }
        // coin
        if j < -1 {
            let ran_street = die.sample(&mut rng);
            commands
                .spawn((
                    Transform {
                        translation: Vec3::new(ran_street as f32, 0.0, j as f32),
                        scale: Vec3::new(0.5, 0.5, 0.5),
                        ..Default::default()
                    },
                    GlobalTransform::IDENTITY,
                ))
                .with_children(|parent| {
                    parent.spawn(SceneBundle {
                        scene: asset_server.load("models/coin.glb#Scene0"),
                        ..default()
                    });
                })
                .insert(Coin);
        }
    }

    //player
    commands
        .spawn((
            Transform {
                translation: Vec3::new(1.0, 0.0, 0.0),
                scale: Vec3::new(0.4, 0.4, 0.4),
                ..Default::default()
            },
            GlobalTransform::IDENTITY,
        ))
        .with_children(|parent| {
            parent.spawn(SceneBundle {
                scene: asset_server.load("models/taxi.glb#Scene0"),
                ..default()
            });
        })
        .insert(Player);

    // scoreboard
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Coin:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(Cointext);
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Best:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(25.0),
                ..default()
            },
            ..default()
        })
        .insert(Besttext);
}