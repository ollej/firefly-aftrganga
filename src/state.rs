use core::cell::OnceCell;

use alloc::vec::Vec;
use ffz_utils::{animation::*,audio::AudioPlayer, camera::Camera, image_text::ImageText, tile_map::TileMap};
use firefly_rust::ImageBuf;

use crate::{bullet::Bullets, game_state::GameState, monster::Monsters, player::Player};

pub static mut STATE: OnceCell<State> = OnceCell::new();

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub struct State {
    pub camera: Camera,
    pub tile_map: TileMap,
    pub audio_player: AudioPlayer,
    pub players: Vec<Player>,
    pub bullets: Bullets,
    pub monsters: Monsters,
    pub character_sprites: ImageBuf,
    pub enemy_sprites: ImageBuf,
    pub bullet_sprites: ImageBuf,
    pub game_state: GameState,
    pub image_text: ImageText,
}

impl State {
    pub fn new(
        camera: Camera,
        tile_map: TileMap,
        audio_player: AudioPlayer,
        players: Vec<Player>,
        character_sprites: ImageBuf,
        enemy_sprites: ImageBuf,
        image_text: ImageText,
        bullet_sprites: ImageBuf,
    ) -> Self {
        Self {
            camera,
            tile_map,
            audio_player,
            players,
            character_sprites,
            enemy_sprites,
            bullet_sprites,
            bullets: Bullets::new(),
            monsters: Monsters::new(30),
            game_state: GameState::Playing,
            image_text,
        }
    }
}
