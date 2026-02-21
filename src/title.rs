use bevy::prelude::*;
use bevy::ui::PositionType::*;

use crate::TitleLayer;

pub fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                position_type: Relative,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            TitleLayer,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    top: Val::Percent(28.0),
                    ..Default::default()
                },
                Text::new("MineSweepish"),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 64.0,
                    ..Default::default()
                },
            ));
            parent
                .spawn((
                    Node {
                        position_type: Absolute,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        top: Val::Percent(70.0),
                        width: Val::Px(240.0),
                        height: Val::Px(64.0),
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Node {
                            ..Default::default()
                        },
                        Text::new("スタート"),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.0)),
                    ));
                });
        });
}

use crate::AppState;

pub fn title_start(
    ints_query: Query<&Interaction, With<Button>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ints in &ints_query {
        if *ints == Interaction::Pressed {
            next_state.set(AppState::Playing);
        }
    }
}

pub fn clean_title(mut commands: Commands, query: Query<Entity, With<TitleLayer>>) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}
