use bevy::prelude::*;

use crate::{MapInfo, SettingButton, SettingType, TitleLayer};

pub fn setup_title(mut commands: Commands, asset_server: Res<AssetServer>, mapinfo: Res<MapInfo>) {

    commands
        // wrapper
        .spawn((
            Node {
                position_type: PositionType::Relative,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            TitleLayer,
        ))

        .with_children(|parent| {

            // title logo
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    top: Val::Percent(28.0),
                    ..default()
                },
                Text::new("MineSweepish"),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                TextShadow {
                    offset: Vec2::new(2.0, 2.0),
                    color: Color::srgb(1.0, 0.0, 1.0),
                }
            ));
            // start button
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        top: Val::Percent(70.0),
                        width: Val::Px(240.0),
                        height: Val::Px(64.0),
                        ..default()
                    },
                    Button,
                    BackgroundColor(Color::srgb(0.0, 0.4, 0.4)),
                    children![(
                        Text::new("スタート"),
                        TextFont {
                            font: asset_server.load("fonts/unifont-17.0.03.otf"),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.0, 1.0)),
                        TextShadow {
                            offset: Vec2::new(0.0,2.0),
                            color: Color::BLACK,
                        },
                    )],
                ));

            // setting buttons
            let map_width = mapinfo.map_width;
            let map_height = mapinfo.map_height;
            let bomb_per = mapinfo.bomb_percent;

            // wrapper
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    left: Val::Percent(70.0),
                    width: Val::Px(200.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                )).with_children(|wrapper|{

                // size width
                wrapper.spawn((
                    Node {
                        position_type: PositionType::Relative,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        left: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.6, 0.6)),
                )).with_children(|container|{

                    // setting type
                    container.spawn((
                        Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            left: Val::Px(0.0),
                            width: Val::Px(72.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        children![(
                            Text::new("Cell WidthSize"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                        )],
                        ));

                    // setting buttons
                    container
                        .spawn((
                            Node {
                                position_type: PositionType::Relative,
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                right: Val::Px(0.0),
                                width: Val::Px(128.0),
                                height: Val::Px(32.0),
                                ..default()
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
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::ZERO,
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    border: UiRect {
                                        bottom: Val::Px(2.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Button,
                                SettingType::Width,
                                SettingButton::TenDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                BorderColor::all(Color::BLACK),
                                children![(
                            Text::new("-10"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::Px(24.0),
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::Width,
                                SettingButton::OneDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("-1"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::Px(24.0),
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::Width,
                                SettingButton::OneUp,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("+1"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::ZERO,
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::Width,
                                SettingButton::TenUp,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("+10"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                        });

                });


                // size height
                wrapper.spawn((
                    Node {
                        position_type: PositionType::Relative,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        left: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.6, 0.6)),
                )).with_children(|container|{

                    // setting type
                    container.spawn((
                        Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            left: Val::Px(0.0),
                            width: Val::Px(72.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        children![(
                            Text::new("Cell HeightSize"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                        )],
                    ));

                    // setting buttons
                    container
                        .spawn((
                            Node {
                                position_type: PositionType::Relative,
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                right: Val::Px(0.0),
                                width: Val::Px(128.0),
                                height: Val::Px(32.0),
                                ..default()
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
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::ZERO,
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    border: UiRect {
                                        bottom: Val::Px(2.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Button,
                                SettingType::Height,
                                SettingButton::TenDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                BorderColor::all(Color::BLACK),
                                children![(
                            Text::new("-10"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::Px(24.0),
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::Height,
                                SettingButton::OneDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("-1"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::Px(24.0),
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::Height,
                                SettingButton::OneUp,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("+1"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::ZERO,
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::Height,
                                SettingButton::TenUp,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("+10"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                        });

                });

                // bomb percent
                wrapper.spawn((
                    Node {
                        position_type: PositionType::Relative,
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        left: Val::Px(0.0),
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.6, 0.6)),
                )).with_children(|container|{

                    // setting type
                    container.spawn((
                        Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            left: Val::Px(0.0),
                            width: Val::Px(72.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        children![(
                            Text::new("Bomb Percent"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                        )],
                    ));

                    // setting buttons
                    container
                        .spawn((
                            Node {
                                position_type: PositionType::Relative,
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                right: Val::Px(0.0),
                                width: Val::Px(128.0),
                                height: Val::Px(32.0),
                                ..default()
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
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::ZERO,
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    border: UiRect {
                                        bottom: Val::Px(2.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Button,
                                SettingType::BombPercent,
                                SettingButton::TenDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                BorderColor::all(Color::BLACK),
                                children![(
                            Text::new("-10"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    left: Val::Px(24.0),
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::BombPercent,
                                SettingButton::OneDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("-1"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::Px(24.0),
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::BombPercent,
                                SettingButton::OneUp,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("+1"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                            button.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    right: Val::ZERO,
                                    width: Val::Px(24.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                Button,
                                SettingType::BombPercent,
                                SettingButton::TenUp,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                children![(
                            Text::new("+10"),
                            TextFont {
                                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                                font_size: 16.0,
                                ..default()
                            },
                            )],
                            ));
                        });

                });
            });
        });
}

use crate::AppState;

pub fn start_button(
    mut ints_query: Query<(&Interaction, &Children, &mut BackgroundColor), (With<Button>, Without<SettingButton>)>,
    mut text_query: Query<&mut TextShadow>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (ints, children, mut color) in &mut ints_query {

        let button_text = children[0];

        if let Ok(mut shadow) = text_query.get_mut(button_text) {

            if *ints == Interaction::Hovered {
                *color = BackgroundColor(Color::srgb(0.0, 0.6, 0.6));
                shadow.offset = Vec2::new(0.0,0.0);
                shadow.color = Color::NONE;
            }
            if *ints == Interaction::Pressed {
                next_state.set(AppState::Playing);
            }
            if *ints == Interaction::None {
                *color = BackgroundColor(Color::srgb(0.0, 0.4, 0.4));
                shadow.offset = Vec2::new(0.0,2.0);
                shadow.color = Color::BLACK;
            }

        }
    }
}

pub fn map_setting(
    // Changedでinteraction処理がされたentityだけを指定でき,離されたintsも受け取れるため処理が毎フレーム行われない.
    mut buttons_query: Query<(&Interaction, &SettingType, &SettingButton, &mut BackgroundColor, &mut Node), Changed<Interaction>>,
    mut text_query: Query<(&SettingType, &mut Text), (With<SettingType>, Without<SettingButton>)>,
    mut settings: ResMut<MapInfo>,
) {
    for (ints, types, buttons, mut bgcolor, mut node) in &mut buttons_query {
        if *ints == Interaction::Pressed {
            use crate::SettingButton::*;
            use crate::SettingType::*;

            match (types, buttons) {
                (Width, TenDown) => settings.map_width = (settings.map_width - 10).max(1).min(64),
                (Width, OneDown) => settings.map_width = (settings.map_width - 1).max(1).min(64),
                (Width, OneUp) => settings.map_width = (settings.map_width + 1).max(1).min(64),
                (Width, TenUp) => settings.map_width = (settings.map_width + 10).max(1).min(64),

                (Height, TenDown) => settings.map_height = (settings.map_height - 10).max(1).min(64),
                (Height, OneDown) => settings.map_height = (settings.map_height - 1).max(1).min(64),
                (Height, OneUp) => settings.map_height = (settings.map_height + 1).max(1).min(64),
                (Height, TenUp) => settings.map_height = (settings.map_height + 10).max(1).min(64),

                (BombPercent, TenDown) => settings.bomb_percent = (settings.bomb_percent - 10).max(1).min(99),
                (BombPercent, OneDown) => settings.bomb_percent = (settings.bomb_percent - 1).max(1).min(99),
                (BombPercent, OneUp) => settings.bomb_percent = (settings.bomb_percent + 1).max(1).min(99),
                (BombPercent, TenUp) => settings.bomb_percent = (settings.bomb_percent + 10).max(1).min(99),
            }

            for (types, mut text) in &mut text_query {
                match types {
                    Width => *text = Text::from(settings.map_width.to_string()),
                    Height => *text = Text::from(settings.map_height.to_string()),
                    BombPercent => *text = Text::from(settings.bomb_percent.to_string()),
                }
            }
        }

        if *ints == Interaction::Hovered {
            *bgcolor = BackgroundColor(Color::srgb(0.0, 0.6, 0.0));
            node.border = UiRect { bottom: Val::Px(2.0), ..default() };
        }
        if *ints == Interaction::None {
            *bgcolor = BackgroundColor(Color::srgb(0.0, 0.4, 0.0));
            node.border = UiRect { ..default() };
        }

    }
}

pub fn clean_title(mut commands: Commands, query: Query<Entity, With<TitleLayer>>) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}
