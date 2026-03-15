use bevy::asset::AssetServer;
use bevy::camera::{Camera, Camera2d, Camera3d, PerspectiveProjection, Projection};
use bevy::color::Color;
use bevy::light::PointLight;
use bevy::prelude::{default, Commands, Res, SceneRoot, Transform};
use crate::minesweepish::ms_main::TitleModel;

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 1.8,
            near: 0.1,
            far: 64.0,
            ..default()
        }),
        Camera {
            order: 0,
            ..default()
        },
        //Transform::from_xyz(-1.0, 9.74, -16.953),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        SceneRoot(asset_server.load("models/title_screen/title.gltf#Scene0")),
        Transform::from_xyz(0.0, -2.0, 0.0),
        TitleModel,
    )).with_children(|model|{
        model.spawn((
            PointLight {
                intensity: 1600.0,
                range: 100.0,
                color: Color::srgb(1.0, 0.5, 0.2),
                shadows_enabled: false,
                radius: 0.2,
                ..default()
            },
            Transform::from_xyz(0.5, 12.2, -17.9),
            TitleModel,
        ));
        model.spawn((
            PointLight {
                intensity: 10000.0,
                range: 100.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                shadows_enabled: false,
                radius: 100.0,
                ..default()
            },
            Transform::from_xyz(-1.5, 12.9, -19.5),
            TitleModel,
        ));
        model.spawn((
            PointLight {
                intensity: 10000.0,
                range: 100.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                shadows_enabled: false,
                radius: 100.0,
                ..default()
            },
            Transform::from_xyz(-1.5, 12.9, -22.5),
            TitleModel,
        ));
        model.spawn((
            PointLight {
                intensity: 10000.0,
                range: 100.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                shadows_enabled: false,
                radius: 100.0,
                ..default()
            },
            Transform::from_xyz(-7.5, 12.9, -19.5),
            TitleModel,
        ));
        model.spawn((
            PointLight {
                intensity: 10000.0,
                range: 100.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                shadows_enabled: false,
                radius: 100.0,
                ..default()
            },
            Transform::from_xyz(-7.5, 12.9, -22.5),
            TitleModel,
        ));
    });

    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
    ));
}