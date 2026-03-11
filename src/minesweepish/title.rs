use bevy::audio::{Volume, PlaybackMode::*};
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::minesweepish::ms_main::{BgmState, TitleButtonType, MapInfo, SettingButton, SettingType, SoundsLoader, TitleLayer, VolumeSetting, VolumeValue};

pub fn setup_title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mapinfo: Res<MapInfo>,
    volume: Res<VolumeValue,>
) {

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
                    top: Val::Percent(24.0),
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
                    TitleButtonType::StartButton,
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
                    width: Val::Px(240.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
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
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                )).with_children(|container|{

                    // setting type
                    container.spawn((
                        Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(112.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        children![(
                            Text::new("Cell\nWidthSize"),
                            TextLayout::new_with_justify(Justify::Center),
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
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                )).with_children(|container|{

                    // setting type
                    container.spawn((
                        Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(112.0),
                            height: Val::Px(32.0),
                            margin: UiRect::all(Val::ZERO),
                            ..default()
                        },
                        children![(
                            Text::new("Cell\nHeightSize"),
                            TextLayout::new_with_justify(Justify::Center),
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
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.5)),
                )).with_children(|container|{

                    // setting type
                    container.spawn((
                        Node {
                            position_type: PositionType::Relative,
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(112.0),
                            height: Val::Px(32.0),
                            ..default()
                        },
                        children![(
                            Text::new("Bomb\nPercent"),
                            TextLayout::new_with_justify(Justify::Center),
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
                                        bottom: Val::Px(0.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Button,
                                SettingType::BombPercent,
                                SettingButton::TenDown,
                                BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
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

    // bgm toggle button
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(0.),
            left: Val::Px(0.),
            width: Val::Px(256.),
            height: Val::Px(64.),
            ..default()
        },
        Button,
        TitleButtonType::BgmToggleButton,
        BackgroundColor(Color::srgb(0.,1.,0.)),
        children![(
            Text::new("BGM (OFF)"),
            TextFont {
                font: asset_server.load("fonts/unifont-17.0.03.otf"),
                font_size: 48.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.0, 1.0)),
            TextShadow {
                offset: Vec2::new(2.0, 2.0),
                color: Color::srgb(1.0, 0.0, 1.0),
            }
        )],
        ));

    // sounds settings
    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(432.),
            left: Val::Px(8.),
            width: Val::Px(128.),
            height: Val::Px(32.),
            ..default()
        },
        BackgroundColor (Color::srgb(0.,1.,1.)),
        RelativeCursorPosition::default(),
        Button,
        VolumeSetting::BGM,
        children![(
            Node {
                width: Val::Percent(volume.bgm * 100.),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor (Color::srgb(0.,1.,0.)),
        )],
        ));

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            top: Val::Px(480.),
            left: Val::Px(8.),
            width: Val::Px(128.),
            height: Val::Px(32.),
            ..default()
        },
        BackgroundColor (Color::srgb(0.,1.,1.)),
        RelativeCursorPosition::default(),
        Button,
        VolumeSetting::SE,
        children![(
            Node {
                width: Val::Percent(volume.se * 100.),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor (Color::srgb(1.,1.,1.)),
        )],
    ));
/*
bgmの再生
最初のボリューム調整
タイトルにbgmとseのボリューム設定を追加する
*/
}

use crate::minesweepish::ms_main::AppState;

pub fn volume_slider(
    //relative cursor positionは始点と終点が-0.5,0.5に固定されたマウス座標を取得できる
    cursor_query: Query<(&Interaction, &VolumeSetting, &RelativeCursorPosition, &Children), With<Button>>,
    mut node_query: Query<&mut Node>,
    mut volume: ResMut<VolumeValue>,
) {
    for (ints, vlm_type, cursor_pos, children) in &cursor_query {
        if *ints == Interaction::Pressed {
            match vlm_type {
                VolumeSetting::BGM => {
                    let Some(pos) = cursor_pos.normalized else { continue };
                    // i32をはさんで小数点丸め
                    let slider_width = ((pos.x + 0.5) * 100.).clamp(0., 100.) as i32;
                    if let Ok(mut node) = node_query.get_mut(children[0]) {
                        node.width = Val::Percent(slider_width as f32);
                        volume.bgm = slider_width as f32 / 100.;
                    }
                }
                VolumeSetting::SE => {
                    let Some(pos) = cursor_pos.normalized else { continue };
                    let slider_width = ((pos.x + 0.5) * 100.).clamp(0., 100.) as i32;
                    if let Ok(mut node) = node_query.get_mut(children[0]) {
                        node.width = Val::Percent(slider_width as f32);
                        volume.se = slider_width as f32 / 100.;
                    }
                }
            }
        }
    }
}

// change volume bgm
pub fn volume_settings(
    volume: Res<VolumeValue>,
    bgm: Res<BgmState>,
    mut audio_query: Query<&mut AudioSink>,
) {
    if volume.is_changed() {
        let Some(entity) = bgm.playingbgm else { return };
        let Ok(mut sink) = audio_query.get_mut(entity) else { return } ;
        sink.set_volume(Volume::Linear(volume.bgm));
    }
}

pub fn title_buttons(
    mut commands: Commands,
    sounds: Res<SoundsLoader>,
    volume: Res<VolumeValue>,
    mut bgm_stats: ResMut<BgmState>,
    mut ints_query: Query<(&Interaction, &TitleButtonType, &Children, &mut BackgroundColor), (With<Button>, Without<SettingButton>, Changed<Interaction>)>,
    mut text_query: Query<(&mut TextShadow, &mut Text)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (ints, title_buttons, children, mut bgcolor) in &mut ints_query {

        if let Ok((mut shadow, mut text)) = text_query.get_mut(children[0]) {
            match *ints {
                Interaction::Hovered => {
                    match title_buttons {
                        TitleButtonType::StartButton => {
                            *bgcolor = BackgroundColor(Color::srgb(0.0, 0.6, 0.6));
                            shadow.offset = Vec2::new(0.0,0.0);
                            shadow.color = Color::NONE;
                        }
                        TitleButtonType::BgmToggleButton => {
                            if bgm_stats.playingbgm.is_some() {
                                *bgcolor = BackgroundColor(Color::srgb(0.,1.,0.));
                                shadow.offset = Vec2::new(0.0,0.0);
                                shadow.color = Color::NONE;
                            } else {
                                *bgcolor = BackgroundColor(Color::srgb(0.,0.,1.));
                                shadow.offset = Vec2::new(0.0,0.0);
                                shadow.color = Color::NONE;
                            }
                        }
                    }
                },
                Interaction::Pressed => {
                    match title_buttons {
                        TitleButtonType::StartButton => {
                            commands.spawn((
                                AudioPlayer::new(sounds.start.clone()),
                                PlaybackSettings {
                                    mode: Despawn,
                                    volume: Volume::Linear(volume.se),
                                    ..default()
                                },
                            ));
                            next_state.set(AppState::Playing);
                        }
                        TitleButtonType::BgmToggleButton => {
                            if let Some(entity) = bgm_stats.playingbgm {
                                commands.entity(entity).despawn();
                                bgm_stats.playingbgm = None;
                                **text = "BGM (OFF)".to_string();
                            } else {
                                let entity = commands.spawn((
                                    AudioPlayer::new(sounds.bgm.clone()),
                                    PlaybackSettings {
                                        mode: Loop,
                                        volume: Volume::Linear(volume.bgm),
                                        ..default()
                                    },
                                )).id();
                                bgm_stats.playingbgm = Some(entity);
                                **text = "BGM (ON)".to_string();
                            }
                        }
                    }
                },
                Interaction::None => {
                    match title_buttons {
                        TitleButtonType::StartButton => {
                            *bgcolor = BackgroundColor(Color::srgb(0.0, 0.4, 0.4));
                            shadow.offset = Vec2::new(0.0,2.0);
                            shadow.color = Color::BLACK;
                        }
                        TitleButtonType::BgmToggleButton => {
                            if bgm_stats.playingbgm.is_some() {
                                    *bgcolor = BackgroundColor(Color::srgb(0.8,0.8,0.8));
                                    shadow.offset = Vec2::new(0.0,2.0);
                                    shadow.color = Color::BLACK;
                            } else {
                                    *bgcolor = BackgroundColor(Color::srgb(1.,1.,1.));
                                    shadow.offset = Vec2::new(0.0,2.0);
                                    shadow.color = Color::BLACK;
                            }
                        }
                    }
                },
            }
        }
    }
}

pub fn map_setting(
    mut commands: Commands,
    sounds: Res<SoundsLoader>,
    volume: Res<VolumeValue>,
    // Changedでinteraction処理がされたentityだけを指定でき,離されたintsも受け取れるため処理が毎フレーム行われない.
    mut buttons_query: Query<(&Interaction, &SettingType, &SettingButton, &mut BorderColor, &mut BackgroundColor, &mut Node), Changed<Interaction>>,
    mut text_query: Query<(&SettingType, &mut Text), (With<SettingType>, Without<SettingButton>)>,
    mut settings: ResMut<MapInfo>,
) {
    for (ints, types, buttons, mut bordercolor, mut bgcolor, mut node) in &mut buttons_query {
        if *ints == Interaction::Pressed {
            commands.spawn((
                AudioPlayer::new(sounds.setting.clone()),
                PlaybackSettings {
                    mode: Despawn,
                    volume: Volume::Linear(volume.se),
                    ..default()
                },
            ));

            use crate::minesweepish::ms_main::{SettingButton::*, SettingType::*};

            match (types, buttons) {
                (Width, TenDown) => settings.map_width = (settings.map_width - 10).clamp(1, 99),
                (Width, OneDown) => settings.map_width = (settings.map_width - 1).clamp(1, 99),
                (Width, OneUp) => settings.map_width = (settings.map_width + 1).clamp(1, 99),
                (Width, TenUp) => settings.map_width = (settings.map_width + 10).clamp(1, 99),

                (Height, TenDown) => settings.map_height = (settings.map_height - 10).clamp(1, 99),
                (Height, OneDown) => settings.map_height = (settings.map_height - 1).clamp(1, 99),
                (Height, OneUp) => settings.map_height = (settings.map_height + 1).clamp(1, 99),
                (Height, TenUp) => settings.map_height = (settings.map_height + 10).clamp(1, 99),

                (BombPercent, TenDown) => settings.bomb_percent = (settings.bomb_percent - 10).clamp(1, 99),
                (BombPercent, OneDown) => settings.bomb_percent = (settings.bomb_percent - 1).clamp(1, 99),
                (BombPercent, OneUp) => settings.bomb_percent = (settings.bomb_percent + 1).clamp(1, 99),
                (BombPercent, TenUp) => settings.bomb_percent = (settings.bomb_percent + 10).clamp(1, 99),
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
            *bordercolor = BorderColor::all(Color::BLACK);
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
