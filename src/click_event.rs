use crate::{AppState, Cell, CellSize, MapInfo, OpenState};
use bevy::prelude::*;

pub fn click_event(
    cellsize: Res<CellSize>,
    mapinfo: Res<MapInfo>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut cells: Query<(&Cell, &mut OpenState, &mut Text2d, &mut TextColor)>,
    windows: Query<&Window>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right){
        let window = windows.single().unwrap();

        let window_width = window.width();
        let window_height = window.height();

        let size_x = &mapinfo.map_width;
        let size_y = &mapinfo.map_height;
        let hint_num = &mapinfo.hint_number;
        let cell_size = cellsize.cell_scale;

        let mut queue: Vec<Vec<i32>> = vec![];

        if mouse.just_pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                let world_x = (pos[0] - window_width / 2.0).floor() as i32;
                let world_y = (pos[1] - window_height / 2.0).floor() as i32;

                let map_x = ((world_x as f32 + (size_x * cell_size / 2) as f32) / cell_size as f32)
                    .floor() as i32;
                let map_y = ((world_y as f32 + (size_y * cell_size / 2) as f32) / cell_size as f32)
                    .floor() as i32;

                if map_x >= 0 && map_y >= 0 && map_x <= size_x - 1 && map_y <= size_y - 1 {
                    println!("マウス座標: {:?}__{:?}", world_x, world_y);
                    println!("ワールド座標: {:?}__{:?}", map_x, map_y);
                    queue.push(vec![map_x, map_y]);
                    while !queue.is_empty() {
                        let queue_pop = queue.pop().unwrap();
                        let pop_x = queue_pop[0];
                        let pop_y = queue_pop[1];
                        for (cell, mut state, mut text, mut color) in cells.iter_mut() {
                            if cell.cell_x == pop_x && cell.cell_y == pop_y {
                                if hint_num[pop_y as usize][pop_x as usize] != 0 {
                                    if state.opened == false && state.flag == false && state.question == false {
                                        if hint_num[pop_y as usize][pop_x as usize] == 9 {
                                            next_state.set(AppState::GameOver);
                                        }
                                        state.opened = true;
                                        let ( converted_text, text_color ) = num_convert(hint_num[pop_y as usize][pop_x as usize]);
                                        **text = converted_text;
                                        **color = text_color;
                                    }
                                    continue;
                                }
                                if state.opened == false && state.flag == false && state.question == false {
                                    state.opened = true;
                                    let ( converted_text, text_color ) = num_convert(hint_num[pop_y as usize][pop_x as usize]);
                                    **text = converted_text;
                                    **color = text_color;
                                    for dy in -1i32..=1 {
                                        for dx in -1i32..=1 {
                                            let nx = pop_x + dx;
                                            let ny = pop_y + dy;

                                            if dx == -1 && dy == -1 {
                                                continue;
                                            }
                                            if dx == 1 && dy == -1 {
                                                continue;
                                            }
                                            if dx == 0 && dy == 0 {
                                                continue;
                                            }
                                            if dx == -1 && dy == 1 {
                                                continue;
                                            }
                                            if dx == 1 && dy == 1 {
                                                continue;
                                            }

                                            if nx >= 0
                                                && ny >= 0
                                                && nx <= size_x - 1
                                                && ny <= size_y - 1
                                            {
                                                queue.push(vec![nx, ny])
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if mouse.just_pressed(MouseButton::Right) {
            if let Some(pos) = window.cursor_position() {
                let world_x = (pos[0] - window_width / 2.0).floor() as i32;
                let world_y = (pos[1] - window_height / 2.0).floor() as i32;

                let map_x = ((world_x as f32 + (size_x * cell_size / 2) as f32) / cell_size as f32)
                    .floor() as i32;
                let map_y = ((world_y as f32 + (size_y * cell_size / 2) as f32) / cell_size as f32)
                    .floor() as i32;

                if map_x >= 0 && map_y >= 0 && map_x <= size_x - 1 && map_y <= size_y - 1 {
                    println!("マウス座標: {:?}__{:?}", world_x, world_y);
                    println!("ワールド座標: {:?}__{:?}", map_x, map_y);

                    for (cell, mut state, mut text, mut color) in cells.iter_mut() {
                        if cell.cell_x == map_x && cell.cell_y == map_y {
                            match (state.opened, state.question, state.flag) {
                                (false, false, false) => {
                                    state.question = true;
                                    **text = "❔".to_string();
                                    **color = Color::srgb(0.5, 0.5, 0.5);
                                }// to question
                                (false, true, false) => {
                                    state.question = false;
                                    state.flag = true;
                                    **text = "❕".to_string();
                                    **color = Color::srgb(1.0, 0.0, 0.0);
                                }// to flag
                                (false, false, true) => {
                                    state.flag = false;
                                    **text = "⬛".to_string();
                                    **color = Color::srgb(1.0, 1.0, 1.0);
                                }// to nothing
                                (_, _, _) => {}
                            }
                        }
                    }
                }
            }
        }

    }
}

fn num_convert(number: usize) -> (String, Color) {
    match number {
        0 => ("0⃣".to_string(), Color::srgb(2.0, 2.0, 2.0)),
        1 => ("1⃣".to_string(), Color::srgb(0.7, 0.7, 1.0)),
        2 => ("2⃣".to_string(), Color::srgb(0.7, 1.0, 0.7)),
        3 => ("3⃣".to_string(), Color::srgb(1.0, 0.7, 0.7)),
        4 => ("4⃣".to_string(), Color::srgb(0.4, 0.4, 1.0)),
        5 => ("5⃣".to_string(), Color::srgb(0.4, 1.0, 0.4)),
        6 => ("6⃣".to_string(), Color::srgb(1.0, 0.4, 0.4)),
        7 => ("7⃣".to_string(), Color::srgb(0.0, 0.0, 1.0)),
        8 => ("8⃣".to_string(), Color::srgb(0.0, 1.0, 0.0)),
        9 => ("9⃣".to_string(), Color::srgb(1.0, 0.0, 0.0)),
        _ => ("".to_string(), Color::srgb(0.0, 0.0, 0.0)),
    }
}

pub fn check_remaining( mouse: Res<ButtonInput<MouseButton>> , cells: Query<&OpenState, With<Cell>>, mapinfo: Res<MapInfo>, mut next_state: ResMut<NextState<AppState>> ) {

    if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
        let safe_cells = &mapinfo.map_width * &mapinfo.map_height - &mapinfo.remaining_bombs;
        let mut opened_cells: Vec<bool> = vec![];
        let mut flag_cells: Vec<bool> = vec![];

        for state in cells.iter() {
            if state.opened == true {
                opened_cells.push(true)
            }
            if state.flag == true { flag_cells.push(true)
            }
            if safe_cells == opened_cells.len() as i32 && mapinfo.remaining_bombs == flag_cells.len() as i32 {
                next_state.set(AppState::GameClear)
            }
        }

    }
}
