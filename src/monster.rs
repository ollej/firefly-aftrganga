use alloc::vec::Vec;
use ffz_utils::{
    camera::Camera, movable::movement_to_step, point_math::PointMath, rectangle::Rectangle,
    tile_map::TileMap, vector::Vector,
};
use firefly_rust::{Color, Image, ImageBuf, Point, Size, Style, draw_circle, math};

use crate::actor_sprite::{ActorSprite, ENEMY_SPRITE};
use crate::state::get_state;

pub struct Monsters {
    monsters: Vec<Monster>,
    spawn_timer: u32,
    max_spawn_timer: u32,
}

impl Default for Monsters {
    fn default() -> Self {
        Self {
            monsters: Vec::new(),
            spawn_timer: 0,
            max_spawn_timer: 0,
        }
    }
}

impl Monsters {
    pub const MAX_MONSTERS: usize = 20;

    pub fn new(spawn_timer: u32) -> Self {
        Monsters {
            spawn_timer,
            max_spawn_timer: spawn_timer,
            ..Default::default()
        }
    }

    pub fn add(&mut self, position: Point, direction: Vector) {
        if self.monsters.len() < Self::MAX_MONSTERS {
            self.monsters.push(Monster::new(position, direction));
        }
    }

    pub fn update(&mut self, tile_map: &TileMap) {
        for monster in self.monsters.iter_mut() {
            monster.update();
        }
        self.monsters.retain(|monster| !monster.is_dead());
        self.spawn_timer = self.spawn_timer.saturating_sub(1);
        if self.spawn_timer == 0 {
            self.spawn_timer = self.max_spawn_timer;
            self.spawn_monster(tile_map);
        }
    }

    pub fn draw(&self, camera: &Camera, sheet: &ImageBuf) {
        for monster in self.monsters.iter() {
            monster.draw(camera, sheet);
        }
    }

    pub fn spawn_monster(&mut self, tile_map: &TileMap) {
        let position = tile_map.random_unblocked_point();
        let velocity = Vector::ZERO;

        self.add(position, velocity);
    }
}

struct Monster {
    position: Point,
    velocity: Vector,
    remainder: Vector,
    movement_timer: u32,
    sprite: ActorSprite,
}

impl Default for Monster {
    fn default() -> Self {
        Self {
            position: Point::MIN,
            velocity: Vector::ZERO,
            remainder: Vector::ZERO,
            movement_timer: 0,
            sprite: ActorSprite::new(),
        }
    }
}

impl Monster {
    const MOVEMENT_TIMER: u32 = 10;

    pub fn new(position: Point, velocity: Vector) -> Self {
        Self {
            position,
            velocity,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let previous_position = self.position;

        self.movement_timer = self.movement_timer.saturating_sub(1);
        if self.movement_timer == 0 {
            self.velocity = self.vector_to_closest_player().unwrap_or(Vector::ZERO);
            self.movement_timer = Self::MOVEMENT_TIMER;
        }
        self.move_horizontally();
        self.move_vertically();

        let actual_delta = Vector::new(
            (self.position.x - previous_position.x) as f32,
            (self.position.y - previous_position.y) as f32,
        );
        self.sprite.update(actual_delta, ENEMY_SPRITE);
        self.kill_players();
    }

    pub fn move_horizontally(&mut self) {
        self.remainder.x += self.velocity.x;
        let amount = math::floor(self.remainder.x + 0.5);
        self.remainder.x -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            self.position = self.position.addx(step);
        }
    }

    pub fn move_vertically(&mut self) {
        self.remainder.y += self.velocity.y;
        let amount = math::floor(self.remainder.y + 0.5);
        self.remainder.y -= amount;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            self.position = self.position.addy(step);
        }
    }

    pub fn draw(&self, camera: &Camera, sheet: &ImageBuf) {
        self.sprite.draw(self.position, camera, sheet, ENEMY_SPRITE);
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle {
            position: Point::new(self.position.x - 8, self.position.y - 8),
            size: Size::new(16, 16),
        }
    }

    pub fn is_dead(&self) -> bool {
        let state = get_state();
        state.bullets.hits(self.rect())
    }

    fn vector_to_closest_player(&self) -> Option<Vector> {
        let state = get_state();
        state
            .players
            .iter()
            .map(|player| Vector::from_point(player.position - self.position))
            .min_by(|a, b| a.magnitude().total_cmp(&b.magnitude()))
            .map(|dir| dir.normalize())
    }

    fn kill_players(&mut self) {
        let state = get_state();
        for player in state.players.iter_mut() {
            if self.hits(player.rect()) {
                player.kill();
            }
        }
    }

    fn hits(&self, rect: Rectangle) -> bool {
        self.rect().overlaps(rect)
    }
}
