#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec;
use ffz_utils::rectangle::Rectangle;
use ffz_utils::tile::Tile;
use ffz_utils::tile_map::TileMap;
use ffz_utils::{audio::AudioPlayer, camera::Camera};
use firefly_rust::*;

use crate::state::{STATE, State, get_state};
use crate::tile_array::TILE_ARRAY;

mod state;
mod tile_array;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let spritesheet = firefly_rust::load_file_buf("spritesheet")
        .unwrap()
        .into_image();
    let is_solid = |tile_index| {
        tile_index != 0
            && tile_index != 16
            && tile_index != 17
            && tile_index != 24
            && tile_index != 25
    };

    let tile_map = TileMap::new_from_2d_array(TILE_ARRAY, Size::new(16, 16), is_solid, spritesheet);
    let camera = Camera::new(Size::new(480, 480));
    let audio_player = AudioPlayer::new(0.5, 1.0);
    //audio_player.play_music("music_loop");

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
