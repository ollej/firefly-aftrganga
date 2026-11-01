use ffz_utils::{camera::Camera, vector::Vector};
use firefly_rust::{
    Color, Image, ImageBuf, Point, Size, Style, draw_ellipse, draw_sub_image, math,
};

const DIRECTION_ROWS: i32 = 16;
// 0,15. 0 is north, 7 is south
const WALK_CYCLE_TICKS: u32 = 24;

#[derive(Clone, Copy)]
pub struct SpriteSpec {
    pub cell: Size,
}

pub const PLAYER_SPRITE: SpriteSpec = SpriteSpec {
    cell: Size { width: 16, height: 16 },
};

pub const ENEMY_SPRITE: SpriteSpec = SpriteSpec {
    cell: Size { width: 16, height: 16 },
};

pub struct ActorSprite {
    facing: i32,
    moving: bool,
    walk_tick: u32,
}

impl ActorSprite {
    pub fn new() -> Self {
        Self {
            facing: 0,
            moving: false,
            walk_tick: 0, 
        }
    }

    pub fn is_attacking(&self) -> bool {
        return false // TODO
    }

    pub fn trigger_attack(&mut self) -> bool {
        if self.is_attacking() {
            return false;
        }
        return false; // PLACEHOLDER
    }

    pub fn update(&mut self, actual_delta: Vector, spec: SpriteSpec){
        
        self.moving = actual_delta.x != 0.0 || actual_delta.y != 0.0;
    }

    pub fn draw(
        &self,
        position: Point,
        camera: &Camera,
        sheet: &ImageBuf,
        spec: SpriteSpec,
    ) {
        let center = camera.world_to_screen(position);

        // draw drop shadow
        draw_ellipse(
            Point::new(center.x - 4, center.y + 3),
            Size::new(8,3),
            Style {
                fill_color: Color::DarkGray,
                stroke_color: Color::DarkGray,
                stroke_width: 1,
            },
        );

        let bob = if self.moving
            && !self.is_attacking()
            && self.walk_tick < WALK_CYCLE_TICKS / 2
        {
            -1
        } else {
            0
        };

        let sprite_column = 0; // Implement logic for attacks later
        let row = self.facing;
        let source = Point::new(sprite_column * spec.cell.width, row * spec.cell.height);
        let frame = sheet.sub(source, spec.cell);
        let destination = Point::new(
            center.x - spec.cell.width / 2,
            center.y - spec.cell.height / 2 + bob,
        );
        draw_sub_image(&frame, destination);
    }
}

fn facing_row(direction: Vector) -> i32 {
    if direction.x == 0.0 &&  direction.y == 0.0 {
        return 0;
    }

    let angle = math::atan2(-direction.x, -direction.y);
    let row = math::floor(
        angle * 8.0 / 3.14 + 0.5,) as i32;

    row.rem_euclid(16)
}
