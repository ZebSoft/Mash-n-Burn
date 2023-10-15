use bevy::prelude::*;

use crate::{entities::*, GameState};

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
    mut commands: Commands, 
    entities:  Query<Entity, Or<(With<Node>, With<Camera3d>, With<AudioSink>, With<Obstacle>, With<Player>, With<PointLight>)>>
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(GameState::InMenu);
}
