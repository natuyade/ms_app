use bevy::prelude::*;

use crate::{AppState, Cell, ClearLayer, MapInfo};

pub fn setup_gameclear( mut commands: Commands, asset_server: Res<AssetServer>, mut next_state: ResMut<NextState<AppState>> ) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.2, 0.5)),
        ClearLayer,
        children![(
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(4.0),
                ..default()
            },
            Text::new("GameClear!"),
            TextFont {
                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                font_size: 64.0,
                ..default()
            },
        )],
        )).with_children(|parent| {

        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                bottom: Val::Percent(4.0),
                width: Val::Px(320.0),
                height: Val::Px(64.0),
                ..default()
            },
            Button,
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        )).with_children(|button| {
            button.spawn((
                Text::new("もういっかい"),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.8, 0.0)),
            ));
        });
    });
}

pub fn clean_gameclear( mut commands: Commands, query: Query<Entity, With<ClearLayer>> ) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}

pub fn title_button(
    mapinfo: Res<MapInfo>,
    mut ints_query: Query<(&Interaction, &mut BackgroundColor),(With<Button>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut color_query: Query<&mut TextColor, With<Cell>>
) {
    for (ints, mut bg_color) in &mut ints_query {
        for mut text_color in &mut color_query {
            if *ints == Interaction::Pressed {
                next_state.set(AppState::Title);
            }

            if *ints == Interaction::None {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                **text_color = Color::srgb(0.0, 0.8, 0.0);
            }

            if mapinfo.map_width == 6 && mapinfo.map_height == 6 && mapinfo.bomb_percent == 6 {
                if *ints == Interaction::Hovered {
                    *bg_color = BackgroundColor(Color::srgb(1.0, 0.0, 1.0));
                    **text_color = Color::srgb(0.0, 0.0, 0.0);
                }
            } else {
                if *ints == Interaction::Hovered {
                    *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                }
            }
        }
    }
}