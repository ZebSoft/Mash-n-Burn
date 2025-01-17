use crate::entities::{self, Besttext, CarSoundMarker, Potato, Scoretext, Game, Player, MashMeterText, Score};

use bevy::{prelude::*, audio::VolumeLevel};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use std::f32::consts::{FRAC_PI_2, PI};

use entities::Street;

pub const OBSTACLE_MODELS: &'static [&'static str] = &[
    "models/hatchbackSports.glb#Scene0",
    "models/police.glb#Scene0",
    "models/sedan.glb#Scene0",
    "models/tractor.glb#Scene0",
];

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>, mut game: ResMut<Game>, mut score: ResMut<Score>) {
    game.obstacle_speed = 2.0f32;
    game.street_speed = 1.5f32;
    
    game.rotation_speed = 0.75f32;
    
    game.engine_speed = 1.0f32;
    
    game.car_target_x = 1.0f32;
    game.car_rotation = Quat::IDENTITY;
    game.car_position = Vec3::new(1.0f32, 0.0f32, 0.0f32);

    score.mash_meter_counter = 0;
    score.value = 0;

    commands.spawn(AudioBundle {
        source: asset_server.load(format!("audio/hackyattacz2.wav")),
        settings: PlaybackSettings{
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.1)),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        AudioBundle {
            source: asset_server.load(format!("audio/Engine.wav")),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.2)),
                ..default()
            },
            ..default()
        },
        CarSoundMarker,
    ));

    //camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 5.0, 4.0).looking_at(Vec3::new(1., 0., -3.), Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1.0, 4.0, 0.0),
        ..default()
    });

    // street
    let mut rng = rand::thread_rng();
    let mut potato_rng = rand::thread_rng();
    let die = Uniform::from(0..3);

    for j in -21..2 {
        let mut children_list: Vec<Entity> = Vec::new();
        for i in 0..3 {
            let entity = commands
                .spawn(SceneBundle {
                    scene: asset_server.load("models/road_straight.glb#Scene0"),
                    transform: Transform {
                        translation: Vec3::new(i as f32, 0.0, 0.0),
                        rotation: Quat::from_rotation_y(FRAC_PI_2),
                        ..default()
                    },
                    ..default()
                })
                .id();

            children_list.push(entity);

            if i == 0 {
                let lamp = commands
                    .spawn(SceneBundle {
                        scene: asset_server.load("models/lamp.glb#Scene0"),
                        transform: Transform {
                            translation: Vec3::new(i as f32 - 0.45, 0.0, 0.0),
                            rotation: Quat::from_rotation_y(FRAC_PI_2),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                children_list.push(lamp);
            }
            if i == 2 {
                let lamp = commands
                    .spawn(SceneBundle {
                        scene: asset_server.load("models/lamp.glb#Scene0"),
                        transform: Transform {
                            translation: Vec3::new(i as f32 + 0.45, 0.0, 0.0),
                            rotation: Quat::from_rotation_y(-FRAC_PI_2),
                            ..default()
                        },
                        ..default()
                    })
                    .id();
                children_list.push(lamp);
            }
            commands
                .spawn(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, j as f32),
                        ..default()
                    },
                    ..default()
                })
                .insert(Street)
                .push_children(&children_list);
        }
        
        if j < -1 {
            let ran_street = die.sample(&mut rng);
            
            let potato_index = rng.gen_range(1..=12);
            
            let material = StandardMaterial { 
                alpha_mode: AlphaMode::Blend,
                base_color_texture: Some(asset_server.load(format!("images/Potato{potato_index}.png"))),
                ..default()
            };

            commands
                .spawn(
                    PbrBundle {
                        mesh: meshes.add(shape::Plane::from_size(1.0).into()),
                        transform: Transform {
                            translation: Vec3::new(ran_street as f32, 0.5, j as f32),
                            rotation: Quat::from_euler(EulerRot::XYZ, FRAC_PI_2 - FRAC_PI_2 / 5.0f32, 0.0f32, 0.0f32),
                            scale: Vec3::new(0.5, 0.5, 0.5),
                            ..Default::default()
                        },
                        material: materials.add(material),
                        ..default()
                    })
                .insert(Potato{..default()});
        }
    }

    //player
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/taxi.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(1.0, 0.0, 0.0),
                scale: Vec3::new(0.4, 0.4, 0.4),
                ..default()
            },
            ..default()
        })
        .insert(Player);

    // scoreboard
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Score:",
                TextStyle {
                    font: asset_server.load("fonts/Blazed.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
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
        .insert(Scoretext);

    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Best:",
                TextStyle {
                    font: asset_server.load("fonts/Blazed.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(1.0, 0.9, 0.0),
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

        commands.spawn(TextBundle {
            text: Text::from_section(
                "Mash-o-Meter:",
                TextStyle {
                    font: asset_server.load("fonts/Blazed.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(1.0, 0.8, 0.5),
                    ..default()
                },
            ).with_alignment(TextAlignment::Center),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                left: Val::Px(60.0),
                ..default()
            },
            ..default()
        })
        .insert(MashMeterText);

}
