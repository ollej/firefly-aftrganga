#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use ffz_utils::rectangle::Rectangle;
use ffz_utils::tile::Tile;
use ffz_utils::tile_map::TileMap;
use ffz_utils::*;
use ffz_utils::{audio::AudioPlayer, camera::Camera};
use firefly_rust::*;

use crate::state::{STATE, State, get_state};

mod state;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let spritesheet = firefly_rust::load_file_buf("spritesheet")
        .unwrap()
        .into_image();
    let tile_size = Size::new(16, 16);
    let tiles = vec![
        Tile::new(Rectangle::new(Point::new(0, 0), tile_size), 1, false),
        Tile::new(Rectangle::new(Point::new(16, 0), tile_size), 1, false),
        Tile::new(Rectangle::new(Point::new(0, 16), tile_size), 1, false),
        Tile::new(Rectangle::new(Point::new(16, 16), tile_size), 1, false),
    ];
    let tile_map = TileMap::new(tiles, Size::new(32, 32), tile_size, spritesheet);
    let camera = Camera::new(Size::new(500, 500));
    let mut audio_player = AudioPlayer::new(0.5, 1.0);
    audio_player.play_music("music_loop");

    #[allow(static_mut_refs)]
    unsafe { STATE.set(State::new(camera, tile_map, audio_player)) }
        .ok()
        .unwrap();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let _state = get_state();
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    let map = &state.tile_map;
    let camera = &state.camera;
    clear_screen(Color::White);
    map.draw(camera);
}
