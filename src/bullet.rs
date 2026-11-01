use alloc::vec::Vec;
use ffz_utils::{
    camera::Camera, movable::movement_to_step, point_math::PointMath, rectangle::Rectangle,
    utilities::random_range, vector::Vector,
};
use firefly_rust::{Color, Point, Size, Style, draw_circle, math};

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

    pub fn update(&mut self, camera: &Camera) {
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
        self.hit_monsters();
        self.bullets.retain(|bullet| bullet.should_keep(camera));
    }

    pub fn draw(&self, camera: &Camera) {
        for bullet in self.bullets.iter() {
            bullet.draw(camera);
        }
    }

    pub fn hit_monsters(&mut self) {
        let state = get_state();
        for bullet in self.bullets.iter_mut() {
            if let Some(monster) = state.monsters.hit_by(bullet.rect()) {
                bullet.kill();
                monster.kill();
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
        }
    }

    pub fn update(&mut self) {
        self.move_bullet();
        self.lifetime = self.lifetime.saturating_sub(1);
    }

    pub fn draw(&self, camera: &Camera) {
        let draw_position = camera.world_to_screen(self.position);
        draw_circle(
            Point {
                x: draw_position.x - 1,
                y: draw_position.y - 1,
            },
            2,
            Style {
                fill_color: Color::Red,
                stroke_color: Color::Red,
                stroke_width: 1,
            },
        );
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle {
            position: Point::new(self.position.x - 1, self.position.y - 1),
            size: Size::new(2, 2),
        }
    }

    pub fn should_keep(&self, camera: &Camera) -> bool {
        self.lifetime > 0 && camera.is_in_screen(self.position)
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
