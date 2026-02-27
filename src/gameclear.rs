use bevy::prelude::*;

use crate::ClearLayer;

pub fn setup_gameclear( mut commands: Commands, asset_server: Res<AssetServer> ) {
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ClearLayer,
        children![(
            Text::new("GameClear!"),
            TextFont {
                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                font_size: 64.0,
                ..default()
            },
        )]
        ));
}