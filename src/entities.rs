use bevy::prelude::*;

#[derive(Component)]
pub struct Street;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Potato{
    pub is_sweet_potato:bool,
    pub has_been_alive_for:f32
}

impl Default for Potato {
    fn default() -> Self {
        Self { is_sweet_potato: false, has_been_alive_for: 0.0f32 }
    }
}

#[derive(Component)]
pub struct Cointext;

#[derive(Component)]
pub struct Besttext;

#[derive(Component)]
pub struct MashMeterText;

#[derive(Resource, Default)]
pub struct Score {
    pub mash_meter_counter:i32,
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
    pub time_tracker_counter:f32,
    pub obstacle_speed: f32,
    pub street_speed: f32,
    pub engine_speed: f32,
    pub rotation_speed: f32,
    pub car_position: Vec3,
    pub car_rotation: Quat,
    pub rotating_since: f32,
    pub stationary_since: f32,
    pub car_direction: CarDirection,
    pub car_target_x: f32,
}