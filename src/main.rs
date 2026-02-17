use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, click_event)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let mapinfo = &mut MapInfo { map_size: vec![0,0], bom_offset: vec![] ,hint_number: vec![]};

    mapinfo.map_size[0] = 10;
    mapinfo.map_size[1] = 10;

    let size_x = mapinfo.map_size[0];
    let size_y = mapinfo.map_size[1];

    let cell = &mut CellSize { cell_size: 0 };

    cell.cell_size = 50;

    let cell_size = cell.cell_size;

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
                Cell { cell_x: (x as f32 -(size_x as f32/2.0)).floor() as i32, cell_y: (y as f32-(size_y as f32/2.0)).floor() as i32},
                OpenState { opened: false },
            ));
        }
    }
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
    cell_size: i32,
}

#[derive(Resource)]
struct MapInfo {
    map_size: Vec<i32>,
    bom_offset: Vec<Vec<i32>>,
    hint_number: Vec<Vec<i32>>,
}

fn click_event( mouse: Res<ButtonInput<MouseButton>>, mut cells: Query<(&Cell, &mut OpenState, &mut Text2d)>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single().unwrap();

        let window_width = window.width();
        let window_height = window.height();

        if let Some(pos) = window.cursor_position() {
            let map_x = ((pos[0] -(window_width/2.0))/50.0).floor() as i32;
            let map_y = (-(pos[1] -(window_height/2.0))/50.0).floor() as i32;


            //println!("マウス座標: {:?}__{:?}", pos[0]-(1280.0/2.0),-(pos[1]-(720.0/2.0)));
            println!("ワールド座標: {:?}__{:?}", map_x,map_y);
            for (cell, mut state, mut text) in cells.iter_mut() {
                if cell.cell_x == map_x && cell.cell_y == map_y {
                    if state.opened == false{
                        *text = Text2d::new("⬜");
                        state.opened = true;
                        println!("cell_x:{} cell_y:{}",cell.cell_x, cell.cell_y);
                    } else {
                        *text = Text2d::new("⬛");
                        state.opened = false;
                    }
                }
            }
        }
    }
}
