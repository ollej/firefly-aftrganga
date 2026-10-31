use alloc::format;
use ffz_utils::{
    camera::Camera, movable::movement_to_step, point_math::PointMath, tile_map::TileMap,
    vector::Vector,
};
use firefly_rust::{Buttons, Color, Peer, Point, Style, draw_circle, math, read_buttons, read_pad};

pub struct Player {
    buttons: Buttons,
    pub peer: Peer,
    pub points: i32,
    pub position: Point,
    start_position: Point,
    speed: f32,
    remainder: Vector,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            buttons: Buttons::default(),
            peer: Peer::default(),
            points: 0,
            speed: 2.5,
            position: Point::MIN,
            start_position: Point::MIN,
            remainder: Vector::ZERO,
        }
    }
}

impl Player {
    pub fn new(peer: Peer, position: Point) -> Self {
        Self {
            peer,
            position,
            start_position: position,
            ..Default::default()
        }
    }

    pub fn update(&mut self, tile_map: &TileMap) {
        let buttons = read_buttons(self.peer);
        if let Some(pad) = read_pad(self.peer) {
            let dpad = pad.as_dpad8();
            let mut direction = Vector::ZERO;
            if dpad.left {
                direction.x = -1.0;
            }
            if dpad.right {
                direction.x = 1.0;
            }
            if dpad.up {
                direction.y = -1.0;
            }
            if dpad.down {
                direction.y = 1.0;
            }
            let velocity = direction.normalize() * self.speed;
            self.move_player(velocity, tile_map);
        }
        self.buttons = buttons;
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

    pub fn draw(&self, camera: &Camera) {
        let draw_position = camera.world_to_screen(self.position);
        draw_circle(
            Point {
                x: draw_position.x - 4,
                y: draw_position.y - 4,
            },
            8,
            Style {
                fill_color: Color::Purple,
                stroke_color: Color::Black,
                stroke_width: 1,
            },
        );
    }

    pub fn reset(&mut self) {
        self.points = 0;
        self.position = self.start_position;
    }

    pub fn stop_movement(&mut self) {
        self.remainder = Vector::ZERO;
    }
}
