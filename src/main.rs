use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(
        MapInfo {
            map_width: 0,
            map_height: 0,
            bomb_offset: vec![],
            hint_number: vec![],
        })
        .insert_resource(CellSize { cell_scale: 0, })
        .add_systems(Startup, setup)
        .add_systems(Update, click_event)
        .run();
}

fn setup( mut mapinfo: ResMut<MapInfo>, mut cellsize: ResMut<CellSize>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    mapinfo.map_width = 4;
    mapinfo.map_height = 4;
    cellsize.cell_scale = 50;

    let size_x = mapinfo.map_width;
    let size_y = mapinfo.map_height;
    let cell_size = cellsize.cell_scale;


    for y in 0..size_y {
        for x in 0..size_x {
            commands.spawn((
                Text2d::new("⬛"),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 48.0,
                    ..default()
                },
                Transform::from_xyz((x * cell_size+(cell_size/2)-(size_x * cell_size/2)) as f32, (y * cell_size+(cell_size/2)-(size_y * cell_size/2)) as f32, 0.0),
                // floorは小数点以下を切り捨て,minusに丸めるメソッド
                Cell { cell_x: x, cell_y: y },
                OpenState { opened: false },
            ));
        }
    }
    commands.spawn(Text2d::new("×"));//中央可視用
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
    bomb_offset: Vec<Vec<usize>>,
    hint_number: Vec<Vec<usize>>,
}


impl MapInfo {
    fn new(base_size: Vec<usize>, base_percent: usize) -> Self {
        fn map_builder(size: &[usize], offset: &[Vec<usize>]) -> Vec<Vec<usize>> {
            let mut map: Vec<Vec<usize>> = vec![vec![0; size[0]]; size[1]];

            for i in 0..offset.len() {
                let offset_y = offset[i][1];
                let offset_x = offset[i][0];

                if size[0] <= 2 && size[1] <= 2{
                    if offset_y == 0&&offset_x == 0 && size[0] == 1 && size[1] == 2 {
                        map[offset_y+1][offset_x] += 1;
                    }
                    if offset_y == 1&&offset_x == 0 && size[0] == 1 && size[1] == 2 {
                        map[offset_y-1][offset_x] += 1;
                    }
                    if offset_y == 0&&offset_x == 0 && size[0] == 2 && size[1] == 1 {
                        map[offset_y][offset_x+1] += 1;
                    }
                    if offset_y == 0&&offset_x == 1 && size[0] == 2 && size[1] == 1 {
                        map[offset_y][offset_x-1] += 1;
                    }
                } else {
                    if offset_y == 0&&offset_x == 0 && size[0] >= 2 && size[1] >= 2{
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_y == 0 && offset_x != 0 && offset_x != size[0]-1 {
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_y == 0 && offset_x == size[0]-1 && size[0] >= 2 && size[1] >= 2{
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                    }
                    if offset_x == 0 && offset_y != 0 && offset_y != size[1]-1{
                        map[offset_y-1][offset_x] += 1;
                        map[offset_y-1][offset_x+1] += 1;
                        map[offset_y][offset_x+1] += 1;
                        map[offset_y+1][offset_x] += 1;
                        map[offset_y+1][offset_x+1] += 1;
                    }
                    if offset_x == size[0]-1 && offset_y != 0 && offset_y != size[1]-1{
                        map[offset_y-1][offset_x-1] += 1;
                        map[offset_y-1][offset_x] += 1;
                        map[offset_y][offset_x-1] += 1;
                        map[offset_y+1][offset_x-1] += 1;
                        map[offset_y+1][offset_x] += 1;
                    }
                    if offset_y == size[1]-1 && offset_x == 0 && size[0] >= 2 && size[1] >= 2{
                        map[offset_y-1][offset_x]+=1;
                        map[offset_y-1][offset_x+1]+=1;
                        map[offset_y][offset_x+1]+=1;
                    }
                    if offset_y == size[1]-1 && offset_x != 0 && offset_x != size[0]-1{
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y-1][offset_x+1] +=1;
                        map[offset_y][offset_x-1] +=1;
                        map[offset_y][offset_x+1] +=1;
                    }
                    if offset_y == size[1]-1 && offset_x == size[0]-1&& size[0] >= 2 && size[1] >= 2{
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y][offset_x-1] +=1;
                    }
                    if offset_x != 0&& offset_y != 0 && offset_x != size[0]-1 && offset_y != size[1] -1 {
                        map[offset_y-1][offset_x-1] +=1;
                        map[offset_y-1][offset_x] +=1;
                        map[offset_y-1][offset_x+1] +=1;
                        map[offset_y][offset_x-1] +=1;
                        map[offset_y][offset_x+1] +=1;
                        map[offset_y+1][offset_x-1] +=1;
                        map[offset_y+1][offset_x] +=1;
                        map[offset_y+1][offset_x+1] +=1;
                    }

                }

            }

            for i in 0..offset.len() {
                let offset_x = offset[i][0];
                let offset_y = offset[i][1];
                map[offset_y][offset_x] = 9;
            }
            map
        }
        fn set_offset_random(size: &[usize], percent: usize) -> Vec<Vec<usize>> {
            let mut offset: Vec<Vec<usize>> = vec![];
            let mut num_of_bomb = (size[0] * size[1]) * percent / 100;
            let rand_x = fastrand::usize(0..size[0]);
            let rand_y = fastrand::usize(0..size[1]);
            offset.push(vec![rand_x, rand_y]);

            if num_of_bomb == 0 {
                num_of_bomb = 1;
            }
            while offset.len() != num_of_bomb {
                println!("{}:{}",offset.len(),num_of_bomb);
                let mut offset_bool:Vec<bool> = vec![];
                let gate_x = fastrand::usize(0..size[0]);
                let gate_y = fastrand::usize(0..size[1]);
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

        let set_bomb_offset = set_offset_random(&base_size, base_percent);
        let set_hint_num = map_builder(&base_size, &set_bomb_offset);
        Self {
            map_width: base_size[0] as i32,
            map_height: base_size[1] as i32,
            bomb_offset: set_bomb_offset,
            hint_number: set_hint_num,
        }
    }
}

fn click_event( cellsize: Res<CellSize>, mapinfo: Res<MapInfo>, mouse: Res<ButtonInput<MouseButton>>, mut cells: Query<(&Cell, &mut OpenState, &mut Text2d)>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {


        let window = windows.single().unwrap();

        let window_width = window.width();
        let window_height = window.height();

        let size_x = mapinfo.map_width;
        let size_y = mapinfo.map_height;

        let cell_size = cellsize.cell_scale;

        if let Some(pos) = window.cursor_position() {

            let world_x : i32;
            let world_y: i32;
                world_x = (pos[0] - window_width/2.0).floor() as i32;
                world_y = -(pos[1] - window_height/2.0).floor() as i32;

            let map_x = ((world_x as f32 + (size_x * cell_size / 2) as f32) / cell_size as f32).floor() as i32;
            let map_y = ((world_y as f32 + (size_y * cell_size / 2) as f32) / cell_size as f32).floor() as i32;

            //println!("マウス座標: {:?}__{:?}", pos[0]-(1280.0/2.0),-(pos[1]-(720.0/2.0)));
            println!("ワールド座標: {:?}__{:?}", map_x,map_y);
            for (cell, mut state, mut text) in cells.iter_mut() {
                if cell.cell_x == map_x && cell.cell_y == map_y {
                    if state.opened == false {
                        if cell.cell_y != 4 {
                            *text = Text2d::new("⬜");
                        }
                        if cell.cell_y == 1 {
                            *text = Text2d::new("1️⃣");
                        }
                        state.opened = true;
                        println!("wcell_x:{} cell_y:{}",cell.cell_x,cell.cell_y);
                    } else {
                        *text = Text2d::new("⬛");
                        state.opened = false;
                        println!("cell_x:{} cell_y:{}",cell.cell_x, cell.cell_y);
                    }
                }
            }
            //let hint_num = bomb.clone();
            let mut queue: Vec<Vec<usize>> = vec![];

            //let hint_x = &hint_num[0];
            //let hint_y = &hint_num;

            //queue.push(vec![input_x, input_y]);
            while !queue.is_empty() {
                println!("queue: {:?}", queue);
                let queue_pop = queue.pop().unwrap();
                let pop_x = queue_pop[0];
                let pop_y = queue_pop[1];
                let map_x = pop_x as i32 + size_x/2;
                let map_y = pop_y as i32 + size_y/2;

                //map[map_y][map_x] = num_convert(hint_num[pop_x][pop_y]);

                //if hint_num[map_y][map_x] != 0 {
                    continue
                }
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        //let nx = pop_x + dx;
                        //let ny = pop_y + dy;

                        if dx == 0 && dy == 0 {
                            continue
                        }

                        //if nx >= 0 && ny >= 0 && nx <= width -1 && ny <= height -1 {
                        //    map[map_y][map_x] = num_convert(hint_num[pop_y][pop_x]);
                        //    queue.push(vec![nx as usize, ny as usize])
                        }
                    }
                }

            }
        //}
    //}
}

fn num_convert(number: usize) -> String {
    match number {
        0 => "0⃣".to_string(),
        1 => "1⃣".to_string(),
        2 => "2⃣".to_string(),
        3 => "3⃣".to_string(),
        4 => "4⃣".to_string(),
        5 => "5⃣".to_string(),
        6 => "6⃣".to_string(),
        7 => "7⃣".to_string(),
        8 => "8⃣".to_string(),
        9 => "9⃣".to_string(),
        _ => "".to_string()
    }
}