use bevy::prelude::*;
use crate::{Cell, CellSize, MapInfo, OpenState};
use crate::convert_num::num_convert;

pub fn click_event(cellsize: Res<CellSize>, mapinfo: Res<MapInfo>, mouse: Res<ButtonInput<MouseButton>>, mut cells: Query<(&Cell, &mut OpenState, &mut Text2d)>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {


        let window = windows.single().unwrap();

        let window_width = window.width();
        let window_height = window.height();

        let size_x = &mapinfo.map_width;
        let size_y = &mapinfo.map_height;

        let hint_num = &mapinfo.hint_number;

        let cell_size = cellsize.cell_scale;

        let mut queue: Vec<Vec<i32>> = vec![];

        if let Some(pos) = window.cursor_position() {

            let world_x = (pos[0] - window_width/2.0).floor() as i32;
            let world_y = (pos[1] - window_height/2.0).floor() as i32;

            let map_x = ((world_x as f32 + (size_x * cell_size / 2) as f32) / cell_size as f32).floor() as i32;
            let map_y = ((world_y as f32 + (size_y * cell_size / 2) as f32) / cell_size as f32).floor() as i32;

            if map_x >= 0 && map_y >= 0 && map_x <= size_x-1 && map_y <= size_y-1 {
                println!("マウス座標: {:?}__{:?}", world_x,world_y);
                println!("ワールド座標: {:?}__{:?}", map_x,map_y);
                queue.push(vec![map_x, map_y]);
                while !queue.is_empty() {
                    println!("queue: {:?}", queue);
                    let queue_pop = queue.pop().unwrap();
                    let pop_x = queue_pop[0];
                    let pop_y = queue_pop[1];
                    for (cell, mut state, mut text) in cells.iter_mut() {
                        if cell.cell_x == pop_x && cell.cell_y == pop_y {
                            if hint_num[pop_y as usize][pop_x as usize] != 0 {
                                if state.opened == false {
                                    state.opened = true;
                                    *text = Text2d::new(num_convert(hint_num[pop_y as usize][pop_x as usize]));
                                }
                                continue
                            }
                            if state.opened == false {
                                state.opened = true;
                                *text = Text2d::new(num_convert(hint_num[pop_y as usize][pop_x as usize]));
                                for dy in -1i32..=1 {
                                    for dx in -1i32..=1 {
                                        let nx = pop_x + dx;
                                        let ny = pop_y + dy;

                                        if dx == -1 && dy == -1 {
                                            continue
                                        }
                                        if dx == 1 && dy == -1 {
                                            continue
                                        }
                                        if dx == 0 && dy == 0 {
                                            continue
                                        }
                                        if dx == -1 && dy == 1 {
                                            continue
                                        }
                                        if dx == 1 && dy == 1 {
                                            continue
                                        }

                                        if nx >= 0 && ny >= 0 && nx <= size_x -1 && ny <= size_y -1 {
                                            queue.push(vec![nx, ny])
                                        }
                                    }
                                }
                            }
                        }
                    }

                    println!("queue_loop");
                }
            }
        }
    }
}
