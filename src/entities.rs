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

impl Default for Score{
    fn default() -> Self {
        Self {
            value:0,
            best:0,
        }
    }
}