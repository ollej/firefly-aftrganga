use ffz_utils::{
    camera::Camera, movable::movement_to_step, point_math::PointMath, tile_map::TileMap,
    vector::Vector,
};
use firefly_rust::{
    Buttons, Color, LineStyle, Peer, Point, Style, draw_circle, draw_line, math, read_buttons,
    read_pad,
};

use crate::{bullet::SHOOT_DIRECTIONS, state::get_state};

enum Shooting {
    Left,
    Right,
}

pub struct Player {
    buttons: Buttons,
    pub peer: Peer,
    pub points: i32,
    pub position: Point,
    start_position: Point,
    speed: f32,
    remainder: Vector,
    shoot_timer: u32,
    shoot_direction: usize,
    shoot_direction_timer: usize,
    shooting: Shooting,
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
            shoot_timer: 0,
            shoot_direction: 0,
            shoot_direction_timer: SHOOT_DIRECTIONS.len(),
            shooting: Shooting::Right,
        }
    }
}

impl Player {
    const SHOOT_TIMER: u32 = 2;
    const SHOOT_DIRECTION_TIMER: usize = 3;
    const TARGET_DISTANCE: f32 = 30.0;

    pub fn new(peer: Peer, position: Point) -> Self {
        Self {
            peer,
            position,
            start_position: position,
            ..Default::default()
        }
    }

    pub fn update(&mut self, tile_map: &TileMap) {
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

        let buttons = read_buttons(self.peer);
        let just_pressed = buttons.just_pressed(&self.buttons);
        if just_pressed.e {
            self.shoot();
        }
        if just_pressed.n {
            self.change_shoot_direction();
        }

        self.buttons = buttons;

        self.shoot_timer = self.shoot_timer.saturating_sub(1);
        self.update_shoot_direction();
    }

    fn shoot(&mut self) {
        let state = get_state();

        if self.shoot_timer == 0 {
            state.audio_player.play_sfx("shoot");
            state
                .bullets
                .add_bullet(self.position, *self.shoot_direction());
            self.shoot_timer = Self::SHOOT_TIMER;
        } else {
            state.audio_player.play_sfx("failed_shot");
        }
    }

    fn shoot_direction(&self) -> &Vector {
        SHOOT_DIRECTIONS
            .get(self.shoot_direction)
            .unwrap_or(&Vector::AXIS_ANGLE)
    }

    fn update_shoot_direction(&mut self) {
        if self.shoot_direction_timer == 0 {
            self.shoot_direction_timer = Self::SHOOT_DIRECTION_TIMER;
            match self.shooting {
                Shooting::Left => {
                    self.shoot_direction += 1;
                    if self.shoot_direction >= SHOOT_DIRECTIONS.len() {
                        self.shoot_direction = 0;
                    }
                }
                Shooting::Right => {
                    if self.shoot_direction == 0 {
                        self.shoot_direction = SHOOT_DIRECTIONS.len() - 1;
                    } else {
                        self.shoot_direction = self.shoot_direction.saturating_sub(1);
                    }
                }
            }
        } else {
            self.shoot_direction_timer = self.shoot_direction_timer.saturating_sub(1);
        }
    }

    pub fn change_shoot_direction(&mut self) {
        self.shooting = match self.shooting {
            Shooting::Left => Shooting::Right,
            Shooting::Right => Shooting::Left,
        };
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
        self.draw_aim(camera);
        self.draw_player(camera);
    }

    pub fn draw_player(&self, camera: &Camera) {
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

    pub fn draw_aim(&self, camera: &Camera) {
        let start_position = camera.world_to_screen(self.position);
        let target_position = self.shoot_direction() * Self::TARGET_DISTANCE + self.position;
        let end_position = camera.world_to_screen(target_position.into());
        draw_line(
            start_position,
            end_position,
            LineStyle::new(Color::Black, 1),
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
