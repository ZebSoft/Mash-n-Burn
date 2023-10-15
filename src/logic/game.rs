use bevy::prelude::*;

use crate::{entities::*, GameState};

pub fn scoreboard(
    score: Res<Score>,
    mut coin_query: Query<(&mut Text, With<Cointext>, Without<Besttext>)>,
    mut best_query: Query<&mut Text, With<Besttext>>,
) {
    let (mut text, _, _) = coin_query.single_mut();
    text.sections[0].value = format!("Coin: {}", score.value);

    let mut best_text = best_query.single_mut();
    best_text.sections[0].value = format!("Best: {}", score.best);
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn show_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "press key to restart",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                        ..default()
                    },
                ),
                ..default()
            });
        });
}

pub fn gameover_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    next_state.set(GameState::Playing);
}
