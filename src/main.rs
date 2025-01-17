mod game_scene;
mod menu_scene;
mod explanation_scene;
mod entities;
mod logic;

use wasm_bindgen::prelude::*;
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

#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa::default())
        .insert_resource(Score::default())

        .init_resource::<Game>()

        .insert_resource(ClearColor(Color::rgb(0.1, 0.5, 0.8)))

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

        // In Menu
        .add_systems(OnEnter(GameState::InMenu), menu_scene::setup)
        .add_systems(Update, menu_scene::button_system.run_if(in_state(GameState::InMenu)))
        .add_systems(OnExit(GameState::InMenu), menu_scene::teardown)

        // In Explanation
        .add_systems(OnEnter(GameState::InExplanation), explanation_scene::setup)
        .add_systems(Update, explanation_scene::button_system.run_if(in_state(GameState::InExplanation)))
        .add_systems(OnExit(GameState::InExplanation), explanation_scene::teardown)

        // Playing
        .add_systems(OnEnter(GameState::Playing), game_scene::setup)
        .add_systems(Update,(car::update, street::update, potato::update, potato::check_collision, obstacle::check_collision, scoring::update, scoring::scoreboard).run_if(in_state(GameState::Playing)))
        .add_systems(Update, obstacle::spawn_obstacle.run_if(on_timer(Duration::from_secs_f32(4.0))))
        
        // Game Over
        .add_systems(Update, game::gameover_keyboard.run_if(in_state(GameState::GameOver)))
        .add_systems(OnEnter(GameState::GameOver), game::show_text)
    
        .run();
}