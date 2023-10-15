use bevy::prelude::*;

#[derive(Component)]
pub struct Street;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct Cointext;

#[derive(Component)]
pub struct Besttext;

#[derive(Resource, Default)]
pub struct Score {
    pub value:i32,
    pub best:i32
}

#[derive(Component)]
pub struct CarSoundMarker;

#[derive(Clone, Copy, Debug)]
pub enum CarDirection {
    Center,
    Left,
    Right
}

impl Default for CarDirection {
    fn default() -> Self {
        CarDirection::Center
    }
}

#[derive(Resource, Default)]
pub struct Game {
    pub obstacle_speed: f32,
    pub street_speed: f32,
    pub engine_speed: f32,
    pub rotation_speed: f32,
    pub car_position: Vec3,
    pub car_rotation: Quat,
    pub rotating_since: f32,
    pub stationary_since: f32,
    pub car_direction: CarDirection,
}