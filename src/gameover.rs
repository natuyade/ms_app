use bevy::prelude::*;
use crate::{AppState, Cell, FailedLayer};

pub fn setup_gameover( mut commands: Commands, asset_server: Res<AssetServer> ) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        FailedLayer,
        BackgroundColor(Color::srgba(0.3,0.0,0.0,0.5)),
        )).with_children(|parent| {

        // gameover text
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                top: Val::Percent(4.0),
                ..default()
            },
            Text::new("GAME OVER"),
            TextFont {
                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                font_size: 64.0,
                ..default()
            },
            ));

        // back button
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                bottom: Val::Percent(4.0),
                width: Val::Px(480.0),
                height: Val::Px(64.0),
                ..default()
            },
            Button,
            BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        )).with_children(|button| {
            button.spawn((
                Text::new("タイトルに戻る"),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.3, 0.3)),
            ));
        });
    });
}

pub fn back_button(
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
                **text_color = Color::srgb(1.0, 0.3, 0.3);
            }

            if *ints == Interaction::Hovered {
                *bg_color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }

        }
    }
}

pub fn clean_gameover( mut commands: Commands, query: Query<Entity, With<FailedLayer>>) {
    for entity in query {
        commands.entity(entity).despawn()
    }
}