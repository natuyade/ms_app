use crate::{Cell, CellSize, MapInfo, OpenState};
use bevy::log::tracing::span::Entered;
use bevy::prelude::*;

use crate::GameLayer;
pub fn setup_ms(
    mut mapinfo: ResMut<MapInfo>,
    mut cellsize: ResMut<CellSize>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let size_x = 15;
    let size_y = 15;
    let bomb_percent = 5;
    let cell_size = 50;

    let map_build = MapInfo::new(size_x, size_y, bomb_percent);

    *mapinfo = map_build;
    *cellsize = CellSize {
        cell_scale: cell_size,
    };

    for y in 0..size_y {
        for x in 0..size_x {
            commands.spawn((
                Text2d::new("⬛"),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 48.0,
                    ..default()
                },
                Transform::from_xyz(
                    (x * cell_size + (cell_size / 2) - (size_x * cell_size / 2)) as f32,
                    -(y * cell_size + (cell_size / 2) - (size_y * cell_size / 2)) as f32,
                    0.0,
                ),
                // floorは小数点以下を切り捨て,minusに丸めるメソッド
                Cell {
                    cell_x: x,
                    cell_y: y,
                },
                OpenState { opened: false },
                GameLayer,
            ));
        }
    }
}

pub fn clean_ms(mut commands: Commands, query: Query<Entity, With<GameLayer>>) {
    for entity in &query {
        commands.entity(entity).despawn()
    }
}
