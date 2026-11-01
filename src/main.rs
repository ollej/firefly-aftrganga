#![no_std]
#![no_main]
extern crate alloc;
use ffz_utils::tile_map::TileMap;
use ffz_utils::utilities::random_range;
use ffz_utils::{audio::AudioPlayer, camera::Camera};
use firefly_rust::*;

use crate::player::Player;
use crate::state::{STATE, State, get_state};
use crate::tile_array::{BLOCKING_TILES, TILE_ARRAY};

mod bullet;
mod monster;
mod player;
mod state;
mod tile_array;
mod actor_sprite;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let spritesheet = firefly_rust::load_file_buf("spritesheet")
        .unwrap()
        .into_image();
    let is_solid = |tile_index| BLOCKING_TILES.contains(&tile_index);
    let modify_tile = |tile_index| match tile_index {
        0 => random_range(56, 58) as i32,
        _ => tile_index,
    };
    let tile_map = TileMap::new_from_2d_array(
        TILE_ARRAY,
        Size::new(16, 16),
        is_solid,
        modify_tile,
        spritesheet,
    );
    let camera = Camera::new(Size::new(480, 480));
    let audio_player = AudioPlayer::new(0.5, 1.0);
    let map_pixel_size = tile_map.map_size_in_pixels;
    let start_position = Point::new(map_pixel_size.width / 2, map_pixel_size.height / 2);
    let players = get_peers()
        .iter()
        .map(|peer| Player::new(peer, start_position))
        .collect();
    //audio_player.play_music("music_loop");
    let character_spritesheet = firefly_rust::load_file_buf("character_sprites")
        .unwrap()
        .into_image();
    #[allow(static_mut_refs)]
    unsafe { STATE.set(State::new(camera, tile_map, audio_player, players, character_spritesheet)) }
        .ok()
        .unwrap();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
    for player in state.players.iter_mut() {
        player.update(&state.tile_map);
        if get_me() == player.peer {
            state.camera.follow_player(player.position, 0.2);
        }
    }
    state.bullets.update(&state.camera);
    state.monsters.update(&state.tile_map);
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    clear_screen(Color::White);
    state.tile_map.draw(&state.camera);
    for player in state.players.iter() {
        player.draw(&state.camera, &state.character_sprites);
    }
    state.monsters.draw(&state.camera);
    state.bullets.draw(&state.camera);
}
