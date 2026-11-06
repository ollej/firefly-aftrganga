use alloc::vec::Vec;
use ffz_utils::{
    animation::Animation, camera::Camera, movable::movement_to_step, point_math::PointMath,
    rectangle::Rectangle, tile_map::TileMap, utilities::random_range, vector::Vector,
};
use firefly_rust::{Color, Image, ImageBuf, Point, Size, draw_sub_image, math};

use crate::state::get_state;

pub struct Bullets {
    pub bullets: Vec<Bullet>,
}

impl Bullets {
    pub fn new() -> Self {
        Self {
            bullets: Vec::new(),
        }
    }

    pub fn add(&mut self, position: Point, direction: Vector) {
        self.bullets.push(Bullet::new(position, direction));
    }

    pub fn update(&mut self, tile_map: &TileMap) {
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
        self.hit_monsters();
        self.bullets.retain(|bullet| bullet.should_keep(tile_map));
    }

    pub fn draw(&self, camera: &Camera, sheet: &ImageBuf) {
        for bullet in self
            .bullets
            .iter()
            .filter(|bullet| camera.is_rect_in_screen(bullet.rect()))
        {
            bullet.draw(camera, sheet);
        }
    }

    pub fn hit_monsters(&mut self) {
        let state = get_state();
        for bullet in self.bullets.iter_mut() {
            if state.monsters.kill_monsters_in_rect(bullet.rect()) {
                bullet.kill();
                state.audio_player.play_sfx("hit");
                state.particles.spawn_radial_burst(
                    bullet.position,
                    random_range(1, 5) as i32,
                    || random_range(0, 2) as i32,
                    20,
                    Color::Red,
                );
            }
        }
    }
}

pub struct Bullet {
    position: Point,
    velocity: Vector,
    remainder: Vector,
    lifetime: u32,
    animation: Animation,
}

impl Bullet {
    const LIFETIME: u32 = 60;
    const SPEED: f32 = 5.0;

    pub fn new(position: Point, velocity: Vector) -> Self {
        Self {
            position,
            velocity: velocity * Self::SPEED,
            remainder: Vector::ZERO,
            lifetime: Self::LIFETIME,
            animation: Animation {
                frames: 4,
                size: Size::new(16, 16),
                frame_position: Point::new(0, 0),
                time_per_frame: 3,
                looping: true,
                ..Animation::default()
            },
        }
    }

    pub fn update(&mut self) {
        self.move_bullet();
        self.animation.update();
        self.lifetime = self.lifetime.saturating_sub(1);
    }

    pub fn draw(&self, camera: &Camera, sheet: &ImageBuf) {
        let center = camera.world_to_screen(self.position);
        let size = self.animation.frame_size();

        let frame = sheet.sub(self.animation.current_frame_position(), size);

        let destination = Point::new(center.x - size.width / 2, center.y - size.height / 2);

        draw_sub_image(&frame, destination);
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle {
            position: Point::new(self.position.x - 1, self.position.y - 1),
            size: Size::new(2, 2),
        }
    }

    pub fn should_keep(&self, tile_map: &TileMap) -> bool {
        self.lifetime > 0 && tile_map.rect().contains(self.position)
    }

    pub fn move_bullet(&mut self) {
        self.move_horizontally();
        self.move_vertically();
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

    fn kill(&mut self) {
        self.lifetime = 0;
    }
}
