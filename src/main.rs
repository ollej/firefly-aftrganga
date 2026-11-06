#![no_std]
#![no_main]
extern crate alloc;
use ffz_utils::animation::*;
use ffz_utils::image_text::{CHARACTERS, ImageText};
use ffz_utils::particles::{ParticleSystem, Visualization};
use ffz_utils::rectangle::Rectangle;
use ffz_utils::text_box::{HorizontalAlignment, TextBox};
use ffz_utils::tile_map::TileMap;
use ffz_utils::utilities::random_range;
use ffz_utils::{audio::AudioPlayer, camera::Camera};
use firefly_rust::*;

use crate::game_state::GameState;
use crate::palette::set_colors;
use crate::player::Player;
use crate::state::{STATE, State, get_state};
use crate::tile_array::{BLOCKING_TILES, TILE_ARRAY};

mod actor_sprite;
mod bullet;
mod game_state;
mod monster;
mod palette;
mod player;
mod state;
mod tile_array;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    set_colors();
    let spritesheet = firefly_rust::load_file_buf("spritesheet")
        .unwrap()
        .into_image();
    let blood = firefly_rust::load_file_buf("blood").unwrap().into_image();
    let title_font = firefly_rust::load_file_buf("title_font")
        .unwrap()
        .into_image();
    let is_solid = |tile_index| BLOCKING_TILES.contains(&tile_index);
    let modify_tile = |tile_index| match tile_index {
        0 => {
            if random_range(0, 100) > 75 {
                random_range(1, 3) as i32
            } else {
                0
            }
        }
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
    let font = load_file_buf("font").unwrap().into_font();
    let text_box = TextBox {
        font: Some(font),
        font_size: Size::new(5, 8),
        horizontal_alignment: HorizontalAlignment::Center,
        rect: Rectangle {
            position: Point::new(0, 126),
            size: Size::new(240, 20),
        },
        color: Color::from(8),
        ..TextBox::default()
    };
    let start_position = Point::new(map_pixel_size.width / 2, map_pixel_size.height / 2);
    let players = get_peers()
        .iter()
        .map(|peer| Player::new(peer, start_position))
        .collect();
    let character_spritesheet = firefly_rust::load_file_buf("character_sprites")
        .unwrap()
        .into_image();
    let enemy_spritesheet = firefly_rust::load_file_buf("enemy_sprites")
        .unwrap()
        .into_image();
    let animation = Animation::new(3, Size::new(16, 16), Point::MIN, 10);
    let visualization = Visualization::Animation(animation, blood);
    let particles = ParticleSystem::new(50, Size::new(480, 480), visualization);
    let bullet_sprites = firefly_rust::load_file_buf("bullet_sprites")
        .unwrap()
        .into_image();
    #[allow(static_mut_refs)]
    unsafe {
        STATE.set(State::new(
            camera,
            tile_map,
            audio_player,
            players,
            character_spritesheet,
            enemy_spritesheet,
            ImageText::new(CHARACTERS.to_vec(), title_font),
            particles,
            bullet_sprites,
            text_box,
        ))
    }
    .ok()
    .unwrap();
    let state = get_state();
    state.audio_player.play_music("theme");
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();

    match state.game_state {
        GameState::Title => update_title(),
        GameState::Playing => update_playing(),
        GameState::GameOver => update_gameover(),
    }
}

fn update_title() {
    let state = get_state();

    let buttons = read_buttons(Peer::COMBINED);
    if buttons.w {
        state.game_state = GameState::Playing;
    }
}

fn update_playing() {
    let state = get_state();

    if state.players.iter().any(|player| !player.alive) {
        state.game_state = GameState::GameOver;
        return;
    }

    for player in state.players.iter_mut() {
        player.update(&state.tile_map);
        if get_me() == player.peer {
            state.camera.follow_player(player.position, 0.2);
        }
    }
    state.bullets.update(&state.camera);
    state.monsters.update(&state.camera, &state.tile_map);
    state.particles.update();
}

fn update_gameover() {
    let state = get_state();

    let buttons = read_buttons(Peer::COMBINED);
    if buttons.w {
        state.reset();
        state.game_state = GameState::Title;
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();

    match state.game_state {
        GameState::Title => render_title(),
        GameState::Playing => render_playing(),
        GameState::GameOver => render_gameover(),
    }
}

fn render_title() {
    let state = get_state();
    clear_screen(Color::from(13));
    state.image_text.draw("AFTR", Point::new(52, 44), 1);
    state.image_text.draw("GANGA", Point::new(35, 84), 1);
    state.text_box.draw(&["Press button <W> to start"]);
}

fn render_playing() {
    let state = get_state();

    clear_screen(Color::from(1));
    state.tile_map.draw(&state.camera);
    for player in state.players.iter() {
        player.draw(&state.camera, &state.character_sprites);
    }
    state.monsters.draw(&state.camera, &state.enemy_sprites);
    state.bullets.draw(&state.camera, &state.bullet_sprites);
    state.particles.draw(&state.camera);
}

fn render_gameover() {
    let state = get_state();
    clear_screen(Color::from(13));
    state.image_text.draw("GAME", Point::new(42, 44), 1);
    state.image_text.draw("OVER", Point::new(48, 84), 1);
}
