mod game_scene;
mod menu_scene;
mod explanation_scene;
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
    InMenu,
    InExplanation,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa::default())
        .insert_resource(Score::default())

        .init_resource::<Game>()

        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window { 
                    resolution: (405.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }
        ))

        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::InMenu), menu_scene::setup)
        .add_systems(Update, menu_scene::button_system.run_if(in_state(GameState::InMenu)))
        .add_systems(OnExit(GameState::InMenu), menu_scene::teardown)

        .add_systems(OnEnter(GameState::InExplanation), explanation_scene::setup)
        .add_systems(Update, explanation_scene::button_system.run_if(in_state(GameState::InExplanation)))
        .add_systems(OnExit(GameState::InExplanation), explanation_scene::teardown)

        .add_systems(OnEnter(GameState::Playing), game_scene::setup)
        .add_systems(Update,(car::update, street::update, coin::update,  coin::check_collision, obstacle::check_collision, game::scoreboard).run_if(in_state(GameState::Playing)))
        .add_systems(Update, obstacle::spawn_obstacle.run_if(on_timer(Duration::from_secs_f32(4.0))))

        .add_systems(Update, game::gameover_keyboard.run_if(in_state(GameState::GameOver)))
        
        .add_systems(OnEnter(GameState::GameOver), game::show_text)
        .add_systems(OnExit(GameState::GameOver), game::teardown)
    
        .run();
}