mod scene;
mod entities;
mod logic;

use bevy::prelude::{*, IntoSystemConfigs};
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use logic::*;
use entities::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa::default())
        .insert_resource(Score::default())

        .init_resource::<Game>()

        .add_plugins(DefaultPlugins)
        //bevy itself
        // .add_plugins(DefaultPlugins.set(Window {
        //     resolution: (400.0, 600.0).into(),
        //     ...default()
        // })))

        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), scene::setup)
        .add_systems(Update,(move_car, move_street, move_coin, move_obstacle, collision_coin, collision_obstacle, scoreboard).run_if(in_state(GameState::Playing)))

        .add_systems(Update, spawn_obstacle.run_if(on_timer(Duration::from_secs_f32(4.0))))

        .add_systems(Update, gameover_keyboard.run_if(in_state(GameState::GameOver)))
        
        .add_systems(OnEnter(GameState::GameOver), show_text)
        .add_systems(OnExit(GameState::GameOver), teardown)
    
        .run();
}