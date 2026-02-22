use bevy::prelude::*;
use bevy::ui::PositionType::*;

use crate::{MapInfo, MapSettings, SettingButton, TitleLayer};

pub fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>, mapinfo: Res<MapInfo>) {
    commands.spawn(Camera2d);

    commands
        // rapper
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
            // title logo
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

            // setting buttons
            let map_width = mapinfo.map_width;
            let map_height = mapinfo.map_height;
            let bomb_per = mapinfo.bomb_percent;
            // size width
            parent
                .spawn((
                    Node {
                        position_type: Absolute,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        top: Val::Percent(48.0),
                        left: Val::Percent(70.0),
                        width: Val::Px(240.0),
                        height: Val::Px(32.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(map_width.to_string()),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                    ));

                    button.spawn((
                        Node {
                            position_type: Absolute,
                            left: Val::Px(0.0),
                            width: Val::Px(24.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        Text::new("-1"),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        Button,
                        SettingButton::OneDown,
                        BackgroundColor(Color::srgb(0.2, 1.0, 0.2)),
                    ));
                    button.spawn((
                        Node {
                            position_type: Absolute,
                            right: Val::Px(0.0),
                            width: Val::Px(24.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        Text::new("+1"),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        Button,
                        SettingButton::OneUp,
                        BackgroundColor(Color::srgb(0.2, 1.0, 0.2)),
                    ));
                });

            // start button
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
    ints_query: Query<&Interaction, (With<Button>, Without<SettingButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ints in &ints_query {
        if *ints == Interaction::Pressed {
            next_state.set(AppState::Playing);
        }
    }
}

pub fn map_setting(
    // Changedでinteraction処理がされたentityだけを指定でき,離されたintsも受け取れるため処理が毎フレーム行われない.
    buttons_query: Query<(&Interaction, &SettingButton), Changed<Interaction>>,
    mut settings: ResMut<MapSettings>,
) {
    for (ints, buttons) in &buttons_query {
        if *ints == Interaction::Pressed {
            match buttons {
                SettingButton::TenDown => settings.value_map_width -= 10,
                SettingButton::OneDown => settings.value_map_width -= 1,
                SettingButton::OneUp => settings.value_map_width += 1,
                SettingButton::TenUp => settings.value_map_width += 10,
            }
        }
        println!("{}", settings.value_map_width)
    }
}

pub fn clean_title(mut commands: Commands, query: Query<Entity, With<TitleLayer>>) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}
