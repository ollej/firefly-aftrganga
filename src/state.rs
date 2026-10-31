use core::cell::OnceCell;

use ffz_utils::{audio::AudioPlayer, camera::Camera, tile_map::TileMap};
use firefly_rust::Buttons;

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub struct State {
    pub buttons: Buttons,
    pub camera: Camera,
    pub tile_map: TileMap,
    pub audio_player: AudioPlayer,
}

impl State {
    pub fn new(camera: Camera, tile_map: TileMap, audio_player: AudioPlayer) -> Self {
        Self {
            buttons: Buttons::default(),
            camera,
            tile_map,
            audio_player,
        }
    }
}
