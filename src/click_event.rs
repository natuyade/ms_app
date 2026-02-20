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

        let mut queue: Vec<Vec<usize>> = vec![];

        let cell_size = cellsize.cell_scale;

        if let Some(pos) = window.cursor_position() {

            let world_x = (pos[0] - window_width/2.0).floor() as i32;
            let world_y = (pos[1] - window_height/2.0).floor() as i32;

            let map_x = ((world_x as f32 + (size_x * cell_size / 2) as f32) / cell_size as f32).floor() as i32;
            let map_y = ((world_y as f32 + (size_y * cell_size / 2) as f32) / cell_size as f32).floor() as i32;

            if map_x >= 0 && map_y >= 0 && map_x <= size_x-1 && map_y <= size_y-1 {
                queue.push(vec![map_x as usize, map_y as usize]);
                println!("queue: {:?}", queue);
                let queue_pop = queue.pop().unwrap();
                let pop_x = queue_pop[0];
                let pop_y = queue_pop[1];
                for (cell, mut state, mut text) in cells.iter_mut() {
                    if cell.cell_x == map_x && cell.cell_y == map_y {
                        if state.opened == false {
                            *text = Text2d::new(num_convert(hint_num[pop_y][pop_x]));
                        }
                    }
                }
            }
            //while !queue.is_empty() {
            /*
                            if hint_num[pop_y][pop_x] != 0 {
                                continue
                            }
                            for dy in -1i32..=1 {
                                for dx in -1i32..=1 {
                                    let nx = pop_x as i32 + dx;
                                    let ny = pop_y as i32 + dy;
            
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
                                            queue.push(vec![nx as usize, ny as usize])
                                    }
                                }
                            }
            
                        }
            
                        //println!("マウス座標: {:?}__{:?}", pos[0]-(1280.0/2.0),-(pos[1]-(720.0/2.0)));
                        println!("ワールド座標: {:?}__{:?}", map_x,map_y);
                        for (cell, mut state, mut text) in cells.iter_mut() {
                            if cell.cell_x == map_x && cell.cell_y == map_y {
                                if state.opened == false {
                                    state.opened = true;
                                    println!("wcell_x:{} cell_y:{}",cell.cell_x,cell.cell_y);
                                } else {
                                    *text = Text2d::new("⬛");
                                    state.opened = false;
                                    println!("cell_x:{} cell_y:{}",cell.cell_x, cell.cell_y);
                                }
                            }
                        }*/

        }
    }
}
