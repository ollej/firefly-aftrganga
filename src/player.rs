use ffz_utils::rectangle::Rectangle;
use ffz_utils::{
    camera::Camera, movable::movement_to_step, point_math::PointMath, tile_map::TileMap,
    vector::Vector,
};
use firefly_rust::{Buttons, ImageBuf, Peer, Point, Size, math, read_buttons, read_pad};

use crate::actor_sprite::{ActorSprite, PLAYER_SPRITE};
use crate::state::get_state;

pub struct Player {
    buttons: Buttons,
    pub peer: Peer,
    pub points: i32,
    pub position: Point,
    sprite: ActorSprite,
    start_position: Point,
    speed: f32,
    remainder: Vector,
    shoot_timer: u32,
    velocity: Vector,
    pub alive: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            peer: Peer::default(),
            points: 0,
            speed: 2.5,
            position: Point::MIN,
            sprite: ActorSprite::new(),
            start_position: Point::MIN,
            remainder: Vector::ZERO,
            shoot_timer: 0,
            velocity: Vector::ZERO,
            alive: true,
        }
    }
}

impl Player {
    const SHOOT_TIMER: u32 = 2;

    pub fn new(peer: Peer, position: Point) -> Self {
        Self {
            peer,
            position,
            start_position: position,
            ..Default::default()
        }
    }

    pub fn update(&mut self, tile_map: &TileMap) {
        let previous_position = self.position;

        if let Some(pad) = read_pad(self.peer) {
            if pad.radius() > 400.0 {
                self.velocity = Into::<Vector>::into(pad) * self.speed;
                self.move_player(self.velocity, tile_map);
            }
        }

        let actual_delta = Vector::new(
            (self.position.x - previous_position.x) as f32,
            (self.position.y - previous_position.y) as f32,
        );

        self.sprite.update(actual_delta, PLAYER_SPRITE);

        let buttons = read_buttons(self.peer);
        let just_pressed = buttons.just_pressed(&self.buttons);
        if just_pressed.e {
            self.shoot();
        }

        self.buttons = buttons;

        self.shoot_timer = self.shoot_timer.saturating_sub(1);
    }

    fn shoot(&mut self) {
        let state = get_state();

        if self.shoot_timer == 0 {
            state.audio_player.play_sfx("shoot");
            state.bullets.add(self.position, self.velocity);
            self.shoot_timer = Self::SHOOT_TIMER;
        } else {
            //state.audio_player.play_sfx("failed_shot");
        }
    }

    pub fn move_player(&mut self, velocity: Vector, tile_map: &TileMap) {
        self.move_horizontally(velocity, tile_map);
        self.move_vertically(velocity, tile_map);
    }

    pub fn move_horizontally(&mut self, velocity: Vector, tile_map: &TileMap) {
        self.remainder.x += velocity.x;
        let amount = math::floor(self.remainder.x + 0.5);
        self.remainder.x -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = self.position.addx(step);
            let nudge_pos = test_pos.addy(step);
            if !tile_map.is_blocked(test_pos) {
                self.position = test_pos;
            } else if !tile_map.is_blocked(nudge_pos) {
                // There was a collision, let's nudge up
                self.position = nudge_pos;
            } else {
                self.stop_movement();
                break;
            }
        }
    }

    pub fn move_vertically(&mut self, velocity: Vector, tile_map: &TileMap) {
        self.remainder.y += velocity.y;
        let amount = math::floor(self.remainder.y + 0.5);
        self.remainder.y -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = self.position.addy(step);
            let nudge_pos = test_pos.addx(step);
            if !tile_map.is_blocked(test_pos) {
                self.position = test_pos;
            } else if !tile_map.is_blocked(nudge_pos) {
                // There was a collision, let's nudge up
                self.position = nudge_pos
            } else {
                self.stop_movement();
                break;
            }
        }
    }

    pub fn draw(&self, camera: &Camera, sheet: &ImageBuf) {
        self.draw_player(camera, sheet);
    }

    pub fn draw_player(&self, camera: &Camera, sheet: &ImageBuf) {
        self.sprite
            .draw(self.position, camera, sheet, PLAYER_SPRITE);
    }

    pub fn reset(&mut self) {
        self.alive = true;
        self.points = 0;
        self.position = self.start_position;
        self.velocity = Vector::ZERO;
        self.sprite = ActorSprite::new();
        self.shoot_timer = 0;
        self.buttons = Buttons::default();
    }

    pub fn stop_movement(&mut self) {
        self.remainder = Vector::ZERO;
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle {
            position: Point::new(self.position.x - 8, self.position.y - 8),
            size: Size::new(16, 16),
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn kill_if_hit_by_monster(&mut self) {
        let state = get_state();
        if state.monsters.has_monsters_in_rect(self.rect()) {
            self.kill();
        }
    }
}
