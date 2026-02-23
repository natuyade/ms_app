use bevy::prelude::*;
use bevy::ui::PositionType::*;

use crate::{MapInfo, SettingButton, SettingType, TitleLayer};

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
                        top: Val::Percent(54.0),
                        left: Val::Percent(70.0),
                        width: Val::Px(80.0),
                        height: Val::Px(32.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(map_width.to_string()),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        SettingType::Width,
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
                        SettingType::Width,
                        SettingButton::OneDown,
                        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
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
                        SettingType::Width,
                        SettingButton::OneUp,
                        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                    ));
                });

            // size height
            parent
                .spawn((
                    Node {
                        position_type: Absolute,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        top: Val::Percent(62.0),
                        left: Val::Percent(70.0),
                        width: Val::Px(80.0),
                        height: Val::Px(32.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(map_height.to_string()),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        SettingType::Height,
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
                        SettingType::Height,
                        SettingButton::OneDown,
                        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
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
                        SettingType::Height,
                        SettingButton::OneUp,
                        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                    ));
                });

            // bomb percent
            parent
                .spawn((
                    Node {
                        position_type: Absolute,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        top: Val::Percent(70.0),
                        left: Val::Percent(70.0),
                        width: Val::Px(80.0),
                        height: Val::Px(32.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(bomb_per.to_string()),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 24.0,
                            ..default()
                        },
                        SettingType::BombPercent,
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
                        SettingType::BombPercent,
                        SettingButton::OneDown,
                        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
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
                        SettingType::BombPercent,
                        SettingButton::OneUp,
                        BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
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
    mut buttons_query: Query<(&Interaction, &SettingType, &SettingButton), Changed<Interaction>>,
    mut text_query: Query<(&SettingType, &mut Text), (With<SettingType>, Without<SettingButton>)>,
    mut settings: ResMut<MapInfo>,
) {
    for (ints, types, buttons) in &mut buttons_query {
        if *ints == Interaction::Pressed {
            let now_type;
            let now_button;

            match types {
                SettingType::Width => now_type = 1,
                SettingType::Height => now_type = 2,
                SettingType::BombPercent => now_type = 3,
            }

            match buttons {
                SettingButton::TenDown => now_button = 1,
                SettingButton::OneDown => now_button = 2,
                SettingButton::OneUp => now_button = 3,
                SettingButton::TenUp => now_button = 4,
            }

            if now_type == 1 {
                match now_button {
                    1 => settings.map_width -= 10,
                    2 => settings.map_width -= 1,
                    3 => settings.map_width += 1,
                    4 => settings.map_width += 10,
                    _ => settings.map_width += 0,
                }
            }

            if now_type == 2 {
                match now_button {
                    1 => settings.map_height -= 10,
                    2 => settings.map_height -= 1,
                    3 => settings.map_height += 1,
                    4 => settings.map_height += 10,
                    _ => settings.map_height += 0,
                }
            }

            if now_type == 3 {
                match now_button {
                    1 => settings.bomb_percent -= 10,
                    2 => settings.bomb_percent -= 1,
                    3 => settings.bomb_percent += 1,
                    4 => settings.bomb_percent += 10,
                    _ => settings.bomb_percent += 0,
                }
            }

            for (types, mut text) in &mut text_query {
                match types {
                    SettingType::Width => *text = Text::from(settings.map_width.to_string()),
                    SettingType::Height => *text = Text::from(settings.map_height.to_string()),
                    SettingType::BombPercent => {
                        *text = Text::from(settings.bomb_percent.to_string())
                    }
                }
            }
            println!("{}", settings.map_width);
        }
    }
}

pub fn clean_title(mut commands: Commands, query: Query<Entity, With<TitleLayer>>) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}

fn update_number() {}
