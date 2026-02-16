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
    let mapinfo = &mut MapInfo::new();

    mapinfo.map_size[0] = 3;
    mapinfo.map_size[1] = 3;

    let size_x = mapinfo.map_size[0];
    let size_y = mapinfo.map_size[1];

    for y in 0..size_y {
        for x in 0..size_x {
            commands.spawn((
                Text2d::new(" ⬛ "),
                TextFont {
                    font: asset_server.load("fonts/unifont-17.0.03.otf"),
                    font_size: 48.0,
                    ..default()
                },
                Transform::from_xyz(x as f32 * 50.0, y as f32 * 50.0, 0.0),
                Cell { cell_x: x as f32 * 50.0, cell_y: y as f32 * 50.0},
            ));
        }
    }
}

#[derive(Component)]
struct Cell {
    cell_x: f32,
    cell_y: f32,
}

struct MapInfo {
    map_size: Vec<i32>,
}

impl MapInfo {
    fn new() -> Self{
        Self {
            map_size: vec![0,0],
        }
    }
}

fn click_event(mouse: Res<ButtonInput<MouseButton>>, mut query: Query<&mut Text2d>, windows: Query<&Window>) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(pos) = window.unwrap().cursor_position() {
            println!("マウス座標: {:?}__{:?}", pos[0]-(1280.0/2.0),-(pos[1]-(720.0/2.0)));
        }
        if let Ok(mut text) = query.single_mut() {
            *text = Text2d::new(" ⬜ ");
        }
    }
}
