use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use ffz_utils::rectangle::Rectangle;
use firefly_rust::{Point, Size};

pub type Entity = usize;

pub struct CollisionGrid {
    grid: Vec<BTreeSet<Entity>>,
    rows: i32,
    columns: i32,
    grid_box_size: Size,
}

impl CollisionGrid {
    const COLUMNS: i32 = 12;
    const ROWS: i32 = 8;

    pub fn new(size: Size) -> Self {
        Self::with_rows_and_columns(size, Self::COLUMNS, Self::ROWS)
    }

    pub fn with_rows_and_columns(size: Size, columns: i32, rows: i32) -> Self {
        let mut grid = Vec::new();
        for _i in 0..(rows * columns) {
            grid.push(BTreeSet::new());
        }

        Self {
            grid,
            rows,
            columns,
            grid_box_size: Size {
                width: size.width / columns,
                height: size.height / rows,
            },
        }
    }

    pub fn add(&mut self, rect: Rectangle, entity: Entity) {
        self.boxes_for_rect(rect).iter().for_each(|index| {
            if let Some(grid_box) = self.grid.get_mut(*index) {
                grid_box.insert(entity);
            }
        });
    }

    pub fn remove(&mut self, entity: Entity) {
        for grid_box in self.grid.iter_mut() {
            grid_box.remove(&entity);
        }
    }

    pub fn replace(&mut self, rect: Rectangle, entity: Entity) {
        self.remove(entity);
        self.add(rect, entity);
    }

    pub fn entities_at(&self, rect: Rectangle) -> BTreeSet<Entity> {
        self.boxes_for_rect(rect)
            .iter()
            .flat_map(|index| self.grid.get(*index).clone())
            .flatten()
            .map(|entity| *entity)
            .collect()
    }

    fn boxes_for_rect(&self, rect: Rectangle) -> BTreeSet<usize> {
        let mut grid_boxes: BTreeSet<usize> = BTreeSet::new();
        grid_boxes.insert(self.box_at_position(rect.position));
        grid_boxes.insert(self.box_at_position(rect.top_right()));
        grid_boxes.insert(self.box_at_position(rect.bottom_left()));
        grid_boxes.insert(self.box_at_position(rect.bottom_right()));
        grid_boxes
    }

    fn box_at_position(&self, position: Point) -> usize {
        let x = position.x / self.grid_box_size.width;
        let y = position.y / self.grid_box_size.height;
        (x + y * self.columns) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_at_position() {
        let grid = CollisionGrid::with_rows_and_columns(Size::new(30, 20), 3, 2);
        assert_eq!(grid.box_at_position(Point::new(0, 0)), 0);
        assert_eq!(grid.box_at_position(Point::new(15, 0)), 1);
        assert_eq!(grid.box_at_position(Point::new(25, 0)), 2);
        assert_eq!(grid.box_at_position(Point::new(0, 15)), 3);
        assert_eq!(grid.box_at_position(Point::new(15, 15)), 4);
        assert_eq!(grid.box_at_position(Point::new(25, 15)), 5);
    }
}
