use bevy::prelude::*;

use crate::entities::{Game, Score, CarSoundMarker, Scoretext, Besttext, MashMeterText};

pub fn update (mut game: ResMut<Game>, time: Res<Time>, mut score: ResMut<Score>, car_sound_controller: Query<&AudioSink, With<CarSoundMarker>>){
    game.time_tracker_counter += time.delta_seconds();
    
    game.engine_speed += time.delta_seconds() * 0.01f32;
    game.obstacle_speed += time.delta_seconds();
    game.street_speed += time.delta_seconds() * 0.25f32;

    if game.time_tracker_counter > 1.0f32 {
        game.time_tracker_counter = 0.0f32;
        score.value += (game.engine_speed * 10.0f32).round() as i32;

        if score.value > score.best {
            score.best = score.value;
        }
    }

    if let Ok(sink) = car_sound_controller.get_single() {
        sink.set_speed(game.engine_speed);
    }
}

pub fn scoreboard(
    score: Res<Score>,
    mut score_query: Query<(&mut Text, With<Scoretext>, Without<Besttext>, Without<MashMeterText>)>,
    mut best_query: Query<(&mut Text, With<Besttext>, Without<Scoretext>, Without<MashMeterText>)>,
    mut mash_query: Query<(&mut Text, With<MashMeterText>, Without<Besttext>, Without<Scoretext>)>
) {
    let (mut score_text, _, _, _) = score_query.single_mut();
    score_text.sections[0].value = format!("Score: {}", score.value);

    
    let (mut best_text, _, _, _) = best_query.single_mut();
    best_text.sections[0].value = format!("Best: {}", score.best);

    let text_color: Color = match score.mash_meter_counter {
        0..=2 => Color::GREEN,
        3..=5 => Color::YELLOW,
        6..=7 => Color::ORANGE_RED,
        8..=10 => Color::RED,
        _ => Color::WHITE
    };

    let (mut mash_text, _, _, _) = mash_query.single_mut();
    mash_text.sections[0].value = format!("MashoMeter: {}", score.mash_meter_counter);
    mash_text.sections[0].style.color = text_color;
}
