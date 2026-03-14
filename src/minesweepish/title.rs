use bevy::audio::{Volume, PlaybackMode::*};
use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::minesweepish::ms_main::{BgmState, TitleButtonType, MapInfo, SettingButton, SettingType, SoundsLoader, TitleLayer, VolumeSetting, VolumeValue, ImageLoader, FontLoader, AtlasLayout};

pub fn setup_title(
    mut commands: Commands,
    mapinfo: Res<MapInfo>,
    volume: Res<VolumeValue>,
    font: Res<FontLoader>,
    image: Res<ImageLoader>,
    atlas_layout: Res<AtlasLayout>,
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
                    font: font.uni_font.clone(),
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
                        top: Val::Percent(70.0),
                        width: Val::Px(240.0),
                        height: Val::Px(64.0),
                        ..default()
                    },
                    Button,
                    TitleButtonType::StartButton,
                    children![
                    ImageNode::from_atlas_image(
                        image.start.clone(),
                        TextureAtlas::from(atlas_layout.start.clone()),
                    ),
                    ],
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
            ))
                .with_children(|wrapper|{
                    for (option_name, init_number, setting_type) in [
                        ("Cell\nWidthSize", map_width.to_string(), SettingType::Width),
                        ("Cell\nHeightSize", map_height.to_string(), SettingType::Height),
                        ("Bomb\nPercent", bomb_per.to_string(), SettingType::BombPercent)
                    ]
                    {
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
                        ))
                            .with_children(|container| {
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
                                        Text::new(option_name),
                                        TextLayout::new_with_justify(Justify::Center),
                                        TextFont {
                                            font: font.uni_font.clone(),
                                            font_size: 16.0,
                                            ..default()
                                        },
                                    )],
                                ));

                                // setting buttons
                                container.spawn((
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
                                            Text::new(init_number),
                                            TextFont {
                                                font: font.uni_font.clone(),
                                                font_size: 24.0,
                                                ..default()
                                            },
                                            setting_type,
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
                                                ..default()
                                            },
                                            Button,
                                            TitleButtonType::SettingButton,
                                            setting_type,
                                            SettingButton::TenDown,
                                            BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                            children![
                                                ImageNode::from_atlas_image(
                                                    image.setting.clone(),
                                                    TextureAtlas {
                                                        layout: atlas_layout.setting.clone(),
                                                        index: 0,
                                                    },
                                                ),
                                            ],
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
                                            TitleButtonType::SettingButton,
                                            setting_type,
                                            SettingButton::OneDown,
                                            BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                            children![
                                                ImageNode::from_atlas_image(
                                                    image.setting.clone(),
                                                    TextureAtlas {
                                                        layout: atlas_layout.setting.clone(),
                                                        index: 3,
                                                    },
                                                ),
                                            ],
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
                                            TitleButtonType::SettingButton,
                                            setting_type,
                                            SettingButton::OneUp,
                                            BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                            children![
                                                ImageNode::from_atlas_image(
                                                    image.setting.clone(),
                                                    TextureAtlas {
                                                        layout: atlas_layout.setting.clone(),
                                                        index: 6,
                                                    },
                                                ),
                                            ],
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
                                            TitleButtonType::SettingButton,
                                            setting_type,
                                            SettingButton::TenUp,
                                            BackgroundColor(Color::srgb(0.0, 0.4, 0.0)),
                                            children![
                                                ImageNode::from_atlas_image(
                                                    image.setting.clone(),
                                                    TextureAtlas {
                                                        layout: atlas_layout.setting.clone(),
                                                        index: 9,
                                                    },
                                                ),
                                            ],
                                        ));
                                    });
                            });

                    }
            });

            // bgm toggle button
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.),
                    left: Val::Px(8.),
                    width: Val::Px(64.),
                    height: Val::Px(64.),
                    ..default()
                },
                Button,
                TitleButtonType::BgmToggleButton,
                children![
                    ImageNode::from_atlas_image(
                        image.bgm.clone(),
                        TextureAtlas::from(atlas_layout.bgm.clone()),
                    ),
                ],
            ));

            // sounds settings
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(432.),
                    left: Val::Px(8.),
                    width: Val::Px(128.),
                    height: Val::Px(32.),
                    ..default()
                },
                BackgroundColor (Color::srgb(0.,0.,0.)),
                RelativeCursorPosition::default(),
                Button,
                VolumeSetting::BGM,
                Text::new("BGM"),
                TextFont {
                    font: font.uni_font.clone(),
                    font_size: 24.,
                    ..default()
                },
                TextColor(Color::srgb(1.,1.,0.)),
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
                    Text::new("BGM"),
                    TextFont {
                        font: font.uni_font.clone(),
                        font_size: 24.,
                        ..default()
                    },
                    TextColor(Color::srgb(1.,0.,1.)),
                    TextShadow {
                        offset: Vec2::new(1.,1.),
                        color: Color::BLACK,
                    },
                )],
            ));

            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(480.),
                    left: Val::Px(8.),
                    width: Val::Px(128.),
                    height: Val::Px(32.),
                    ..default()
                },
                BackgroundColor (Color::srgb(0.,0.,0.)),
                RelativeCursorPosition::default(),
                Button,
                VolumeSetting::SE,
                Text::new("SE"),
                TextFont {
                    font: font.uni_font.clone(),
                    font_size: 24.,
                    ..default()
                },
                TextColor(Color::srgb(1.,1.,0.)),
                children![(
                    Node {
                        width: Val::Percent(volume.se * 100.),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor (Color::srgb(0.,1.,1.)),
                    Text::new("SE"),
                    TextFont {
                        font: font.uni_font.clone(),
                        font_size: 24.,
                        ..default()
                    },
                    TextColor(Color::srgb(1.,0.,1.)),
                    TextShadow {
                        offset: Vec2::new(1.,1.),
                        color: Color::BLACK,
                    },
                )],
            ));
        });
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
    // Changedでinteraction処理がされたentityだけを指定でき,離されたintsも受け取れるため処理が毎フレーム行われない.
    mut ints_query: Query<(&Interaction, &TitleButtonType, &Children, Option<&SettingType>, Option<&SettingButton>), (With<Button>, Changed<Interaction>)>,
    mut mapinfo: ResMut<MapInfo>,
    mut text_query: Query<(&SettingType, &mut Text)>,
    mut image_query: Query<&mut ImageNode>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (ints, title_buttons, children, setting_type, setting_button) in &mut ints_query {
        if let Ok(mut image) = image_query.get_mut(children[0]) {

            if let Some(atlas) = &mut image.texture_atlas {
                match title_buttons {
                    // start button [index = 0~2]
                    TitleButtonType::StartButton => {
                        match *ints {

                            Interaction::None => {
                                atlas.index = 0;
                            }

                            Interaction::Hovered => {
                                atlas.index = 1;
                            }

                            Interaction::Pressed => {
                                atlas.index = 2;
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
                        }
                    }
                    // bgm button [index = normal:0~2, muted:3~5]
                    TitleButtonType::BgmToggleButton => {
                        match *ints {

                            Interaction::None => {
                                if bgm_stats.playingbgm.is_some() {
                                    atlas.index = 0;
                                } else {
                                    atlas.index = 3;
                                }
                            }

                            Interaction::Hovered => {
                                if bgm_stats.playingbgm.is_some() {
                                    atlas.index = 1;
                                } else {
                                    atlas.index = 4;
                                }
                            }

                            Interaction::Pressed => {
                                if let Some(entity) = bgm_stats.playingbgm {
                                    atlas.index = 5;
                                    commands.entity(entity).despawn();
                                    bgm_stats.playingbgm = None;
                                } else {
                                    atlas.index = 2;
                                    let entity = commands.spawn((
                                        AudioPlayer::new(sounds.bgm.clone()),
                                        PlaybackSettings {
                                            mode: Loop,
                                            volume: Volume::Linear(volume.bgm),
                                            ..default()
                                        },
                                    )).id();
                                    bgm_stats.playingbgm = Some(entity);
                                }
                            }
                        }
                    }
                    // bgm button [index = -10:0~2, -1:3~5, +1:6~8, +10:9~11]
                    TitleButtonType::SettingButton => {
                        use crate::minesweepish::ms_main::{SettingButton::*, SettingType::*};
                        let Some(types) = setting_type else { continue };
                        let Some(buttons) = setting_button else { continue };

                            if *ints == Interaction::None {
                                atlas.index = match buttons {
                                    TenDown => 0,
                                    OneDown => 3,
                                    OneUp => 6,
                                    TenUp => 9,
                                }
                            }

                            if *ints == Interaction::Hovered {
                                atlas.index = match buttons {
                                    TenDown => 1,
                                    OneDown => 4,
                                    OneUp => 7,
                                    TenUp => 10,
                                }
                            }

                            if *ints == Interaction::Pressed {

                                atlas.index = match buttons {
                                    TenDown => 2,
                                    OneDown => 5,
                                    OneUp => 8,
                                    TenUp => 11,
                                };

                                commands.spawn((
                                    AudioPlayer::new(sounds.setting.clone()),
                                    PlaybackSettings {
                                        mode: Despawn,
                                        volume: Volume::Linear(volume.se),
                                        ..default()
                                    },
                                ));

                                match (types, buttons) {
                                    (Width, TenDown) => mapinfo.map_width = (mapinfo.map_width - 10).clamp(1, 20),
                                    (Width, OneDown) => mapinfo.map_width = (mapinfo.map_width - 1).clamp(1, 20),
                                    (Width, OneUp) => mapinfo.map_width = (mapinfo.map_width + 1).clamp(1, 20),
                                    (Width, TenUp) => mapinfo.map_width = (mapinfo.map_width + 10).clamp(1, 20),

                                    (Height, TenDown) => mapinfo.map_height = (mapinfo.map_height - 10).clamp(1, 20),
                                    (Height, OneDown) => mapinfo.map_height = (mapinfo.map_height - 1).clamp(1, 20),
                                    (Height, OneUp) => mapinfo.map_height = (mapinfo.map_height + 1).clamp(1, 20),
                                    (Height, TenUp) => mapinfo.map_height = (mapinfo.map_height + 10).clamp(1, 20),

                                    (BombPercent, TenDown) => mapinfo.bomb_percent = (mapinfo.bomb_percent - 10).clamp(1, 99),
                                    (BombPercent, OneDown) => mapinfo.bomb_percent = (mapinfo.bomb_percent - 1).clamp(1, 99),
                                    (BombPercent, OneUp) => mapinfo.bomb_percent = (mapinfo.bomb_percent + 1).clamp(1, 99),
                                    (BombPercent, TenUp) => mapinfo.bomb_percent = (mapinfo.bomb_percent + 10).clamp(1, 99),
                                }

                                for (text_types, mut text) in &mut text_query {
                                    match text_types {
                                        Width => {
                                            **text = mapinfo.map_width.to_string()
                                        }
                                        Height => {
                                            **text = mapinfo.map_height.to_string()
                                        }
                                        BombPercent => {
                                            **text = mapinfo.bomb_percent.to_string()
                                        }
                                    }
                                }
                            }
                    }
                }
            };
        }
    }
}

pub fn clean_title(mut commands: Commands, query: Query<Entity, With<TitleLayer>>) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}
