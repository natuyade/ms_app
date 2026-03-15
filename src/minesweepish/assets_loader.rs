use bevy::asset::{AssetServer, Assets};
use bevy::image::TextureAtlasLayout;
use bevy::math::UVec2;
use bevy::prelude::{Res, ResMut};
use crate::minesweepish::ms_main::{FontLoader, SoundsLoader, ImageLoader, AtlasLayout};

pub fn load_assets(
    mut font_loader: ResMut<FontLoader>,
    mut sounds_loader: ResMut<SoundsLoader>,
    mut images_loader: ResMut<ImageLoader>,
    mut atlas_handler: ResMut<AtlasLayout>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // font
    font_loader.uni_font = asset_server.load("fonts/unifont-17.0.03.otf");

    // sounds
    sounds_loader.bgm = asset_server.load("sounds/bgm.wav");
    sounds_loader.start = asset_server.load("sounds/start.wav");
    sounds_loader.setting = asset_server.load("sounds/setting_button.wav");
    sounds_loader.open_cell = asset_server.load("sounds/open_cell.wav");
    sounds_loader.failed = asset_server.load("sounds/failed.wav");
    sounds_loader.clear = asset_server.load("sounds/clear.wav");

    // images
    images_loader.bgm = asset_server.load("images/bgm_button.webp");
    images_loader.start = asset_server.load("images/start_button.webp");
    images_loader.setting = asset_server.load("images/setting_button.webp");

    // atlas settings
    atlas_handler.bgm = atlas_layout.add(
        TextureAtlasLayout::from_grid(
            UVec2::new(32,32),
            3,
            2,
            None,
            None,
        )
    );

    atlas_handler.start = atlas_layout.add(
        TextureAtlasLayout::from_grid(
            UVec2::new(120,32),
            3,
            1,
            None,
            None,
        )
    );

    atlas_handler.setting = atlas_layout.add(
        TextureAtlasLayout::from_grid(
            UVec2::new(24,32),
            3,
            4,
            None,
            None,
        )
    );
}