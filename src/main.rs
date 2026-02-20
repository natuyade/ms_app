use bevy::prelude::*;

mod convert_num;
mod startup;
mod click_event;

use crate::startup::setup;
use crate::click_event::click_event;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(
        MapInfo {
            map_width: 0,
            map_height: 0,
            bomb_percent: 0,
            bomb_offset: vec![],
            hint_number: vec![],
        })
        .insert_resource(CellSize { cell_scale: 0, })
        .add_systems(Startup, setup)
        .add_systems(Update, click_event)
        .run();
}

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
    bomb_offset: Vec<Vec<usize>>,
    hint_number: Vec<Vec<usize>>,
}


impl MapInfo {
    fn new(base_size_x: i32, base_size_y: i32, base_percent: i32) -> Self {
        fn map_builder(base_size_x: i32, base_size_y: i32, offset: &[Vec<usize>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> = vec![vec![0; base_size_x as usize]; base_size_y as usize];

            let pop_x = 0i32;
            let pop_y = 0i32;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = pop_x + dx;
                    let ny = pop_y + dy;

                    if dx == 0 && dy == 0 {
                        continue
                    }

                    if nx >= 0 && ny >= 0 && nx <= base_size_x -1 && ny <= base_size_y -1 {
                    }
                }
            }

            for i in 0..offset.len() {
                let offset_x = offset[i][0];
                let offset_y = offset[i][1];
                map[offset_y][offset_x] = 9;
            }

            for y in map.iter() {
                for x in y.iter() {
                    print!("{}", x)
                }
                println!()
            }

            map
        }
        fn set_offset_random(base_size_x: i32, base_size_y: i32, percent: i32) -> Vec<Vec<usize>> {
            let mut offset: Vec<Vec<usize>> = vec![];
            let mut num_of_bomb = ((base_size_x * base_size_y) * percent / 100) as usize;
            let rand_x = fastrand::usize(0..base_size_x as usize);
            let rand_y = fastrand::usize(0..base_size_y as usize);
            offset.push(vec![rand_x, rand_y]);

            if num_of_bomb == 0 {
                num_of_bomb = 1;
            }
            while offset.len() != num_of_bomb {
                println!("{}:{}",offset.len(),num_of_bomb);
                let mut offset_bool:Vec<bool> = vec![];
                let gate_x = fastrand::usize(0..base_size_x as usize);
                let gate_y = fastrand::usize(0..base_size_y as usize);
                for i in 0..offset.len() {
                    if offset[i][0] != gate_x && offset[i][1] != gate_y&&offset.len() != num_of_bomb {
                        offset_bool.push(true);
                    }
                    if offset[i][0] == gate_x && offset[i][1] == gate_y&&offset.len() != num_of_bomb {
                        offset_bool.push(false);
                    }

                }
                if offset_bool.iter().all(|&bool|bool) == true {
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
            bomb_offset: set_bomb_offset,
            hint_number: set_hint_num,
        }
    }
}
