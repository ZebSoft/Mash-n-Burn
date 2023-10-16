use bevy::prelude::*;

use crate::{entities::*, GameState};

pub fn show_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "You're Mashed Out!",
                    TextStyle {
                        font: asset_server.load("fonts/Blazed.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.1, 0.1),
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press Space to restart.",
                    TextStyle {
                        font: asset_server.load("fonts/NunitoSans_Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
        });
}

pub fn gameover_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands, 
    entities:  Query<Entity, Or<(With<Node>, With<Camera3d>, With<AudioSink>, With<Obstacle>, With<Player>, With<PointLight>, With<Transform>, With<Street>, With<Obstacle>, With<Potato>)>>
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(GameState::InMenu);
}
