use std::collections::HashMap;
use crate::drawable::{BoundingBox, Drawable};
use crate::frame::Frame;
use crate::tile::{Coord, Tile};

pub struct Room {
    pub width: u16,
    pub height: u16,
    pub x: i32,
    pub y: i32,
}

impl Room {
    fn is_wall(&self, row: i32, col: i32) -> bool {
        row == self.y
            || row == (self.y + self.height as i32 - 1)
            || col == self.x
            || col == (self.x + self.width as i32 - 1)
    }
}

impl Drawable for Room {
    fn draw(&self, frame: &mut Frame) {
        // Outline a rectangle: walls on the boundary, '.' for floors inside.
        for row in self.y..(self.y + self.height as i32) {
            for col in self.x..(self.x + self.width as i32) {
                let is_wall = self.is_wall(row, col);

                let ch = if is_wall { '#' } else { ' ' };
                frame.set_world_char(col, row, ch);
            }
        }
    }

    fn static_map(&self, collision_map: &mut HashMap<Coord, Tile>) {
        for row in self.y..(self.y + self.height as i32) {
            for col in self.x..(self.x + self.width as i32) {
                let is_wall = self.is_wall(row, col);

                if is_wall {
                    collision_map.insert((col, row), Tile::new(false));
                }
            }
        }
    }

    fn bound_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.x as i32,
            right: (self.x + self.width as i32) as i32,
            top: self.y as i32,
            bottom: (self.y + self.height as i32) as i32,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
