use bevy::prelude::*;

mod click_event;
mod convert_num;
mod setup_msmap;
mod start_button;
mod title;
mod gameover;

use crate::click_event::click_event;
use crate::setup_msmap::{clean_ms, setup_ms};
use crate::title::{clean_title, map_setting, setup_title, start_button};
use crate::gameover::{clean_gameover, setup_gameover, back_button};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Title,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .insert_resource(MapInfo {
            map_width: 10,
            map_height: 10,
            bomb_percent: 10,
            hint_number: vec![],
        })
        .insert_resource(CellSize { cell_scale: 0 })
        .add_systems(OnEnter(AppState::Title), setup_title)
        .add_systems(OnExit(AppState::Title), clean_title)
        .add_systems(OnEnter(AppState::Playing), setup_ms)
        .add_systems(OnExit(AppState::Playing), clean_ms)
        .add_systems(OnEnter(AppState::GameOver), setup_gameover)
        .add_systems(OnExit(AppState::GameOver), clean_gameover)
        .add_systems(
            Update,
            (
                start_button.run_if(in_state(AppState::Title)),
                map_setting.run_if(in_state(AppState::Title)),
                click_event.run_if(in_state(AppState::Playing)),
                back_button.run_if(in_state(AppState::GameOver)),
            ),
        )
        .run();
}

#[derive(Component)]
struct TitleLayer;

#[derive(Component)]
struct GameLayer;

#[derive(Component)]
struct FailedLayer;

#[derive(Component)]
struct Cell {
    cell_x: i32,
    cell_y: i32,
}

#[derive(Component)]
struct OpenState {
    opened: bool,
}

#[derive(Resource)]
struct CellSize {
    cell_scale: i32,
}

#[derive(Resource)]
struct MapInfo {
    map_width: i32,
    map_height: i32,
    bomb_percent: i32,
    hint_number: Vec<Vec<usize>>,
}

#[derive(Component)]
enum SettingType {
    Width,
    Height,
    BombPercent,
}
#[derive(Component)]
enum SettingButton {
    OneUp,
    OneDown,
    TenUp,
    TenDown,
}

impl MapInfo {
    fn new(base_size_x: i32, base_size_y: i32, base_percent: i32) -> Self {
        fn map_builder(base_size_x: i32, base_size_y: i32, offset: &[Vec<i32>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> =
                vec![vec![0; base_size_x as usize]; base_size_y as usize];

            for i in 0..offset.len() {
                let offset_x = offset[i][0];
                let offset_y = offset[i][1];
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = offset_x + dx;
                        let ny = offset_y + dy;

                        if nx >= 0 && ny >= 0 && nx <= base_size_x - 1 && ny <= base_size_y - 1 {
                            map[ny as usize][nx as usize] += 1;
                        }
                    }
                }
            }

            for i in 0..offset.len() {
                let offset_x = offset[i][0];
                let offset_y = offset[i][1];
                map[offset_y as usize][offset_x as usize] = 9;
            }

            for y in map.iter() {
                for x in y.iter() {
                    print!("{}", x)
                }
                println!()
            }

            map
        }
        fn set_offset_random(base_size_x: i32, base_size_y: i32, percent: i32) -> Vec<Vec<i32>> {
            let mut offset: Vec<Vec<i32>> = vec![];
            let mut num_of_bomb = ((base_size_x * base_size_y) * percent / 100) as usize;
            let rand_x = fastrand::i32(0..base_size_x);
            let rand_y = fastrand::i32(0..base_size_y);
            offset.push(vec![rand_x, rand_y]);

            if num_of_bomb == 0 {
                num_of_bomb = 1;
            }
            while offset.len() != num_of_bomb {
                let mut offset_bool: Vec<bool> = vec![];
                let gate_x = fastrand::i32(0..base_size_x);
                let gate_y = fastrand::i32(0..base_size_y);
                for i in 0..offset.len() {
                    if offset[i][0] != gate_x
                        && offset[i][1] != gate_y
                        && offset.len() != num_of_bomb
                    {
                        offset_bool.push(true);
                    }
                    if offset[i][0] == gate_x
                        && offset[i][1] == gate_y
                        && offset.len() != num_of_bomb
                    {
                        offset_bool.push(false);
                    }
                }
                if offset_bool.iter().all(|&bool| bool) == true {
                    offset.push(vec![gate_x, gate_y])
                }
            }

            offset
        }

        let set_bomb_offset = set_offset_random(base_size_x, base_size_y, base_percent);
        let set_hint_num = map_builder(base_size_x, base_size_y, &set_bomb_offset);
        Self {
            map_width: base_size_x,
            map_height: base_size_y,
            bomb_percent: base_percent,
            hint_number: set_hint_num,
        }
    }
}
