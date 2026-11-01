use alloc::vec::Vec;
use ffz_utils::{
    animation::Animation, camera::Camera, movable::movement_to_step, point_math::PointMath, rectangle::Rectangle,
    vector::Vector, animation::*,
};
use firefly_rust::{Image, ImageBuf, Color, Point, Size, Style, draw_circle, draw_sub_image, math};

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
        self.bullets.retain(|bullet| bullet.should_keep(camera));
    }

    pub fn draw(&self, camera: &Camera, sheet: &ImageBuf) {
        for bullet in self.bullets.iter() {
            bullet.draw(camera, sheet);
        }
    }

    pub fn hits(&mut self, rect: Rectangle) -> bool {
        self.bullets
            .iter_mut()
            .find(|bullet| rect.overlaps(bullet.rect()))
            .map(|bullet| {
                bullet.kill();
                bullet
            })
            .is_some()
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
                size: Size::new(16,16),
                frame_position: Point::new(0,0),
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

        let frame = sheet.sub(
            self.animation.current_frame_position(),
            size,
        );

        let destination = Point::new(
            center.x - size.width / 2,
            center.y - size.height / 2,
        );

        draw_sub_image(&frame, destination);
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
