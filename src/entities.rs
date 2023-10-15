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

#[derive(Resource)]
pub struct Score {
    pub value:i32,
    pub best:i32
}

#[derive(Component)]
pub struct CarSoundMarker;

#[derive(Resource, Default)]
pub struct Game {
    pub obstacle_speed:f32,
    pub street_speed:f32,
    pub engine_speed:f32,
    pub car_sound: Option<Entity>
}


impl Default for Score{
    fn default() -> Self {
        Self {
            value:0,
            best:0,
        }
    }
}