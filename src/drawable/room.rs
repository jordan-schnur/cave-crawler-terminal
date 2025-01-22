use crate::drawable::{BoundingBox, Drawable};
use crate::frame::Frame;
use crate::tile::{Coord, Tile};
use std::collections::HashMap;

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

    fn is_door(&self, row: i32, col: i32) -> bool {
        let middle_x = self.x + (self.width as f32 / 2.0).floor() as i32;
        let middle_y = self.y + (self.height as f32 / 2.0).floor() as i32;

        if row == self.x && col == middle_x {
            return true;
        } else if row == middle_y && col == self.y {
            return true;
        } else if row == (self.y + self.height as i32 - 1) && col == middle_x {
            return true;
        } else if row == middle_y && col == (self.x + self.width as i32 - 1) {
            return true;
        }

        return false;
    }
}

impl Drawable for Room {
    fn draw(&self, frame: &mut Frame) {
        for row in self.y..(self.y + self.height as i32) {
            for col in self.x..(self.x + self.width as i32) {
                let is_wall = self.is_wall(row, col);
                let is_door = self.is_door(row, col);

                if is_door {
                    frame.set_world_char(col, row, '+');
                    continue;
                } else if is_wall {
                    frame.set_world_char(col, row, '#');
                    continue;
                }
            }
        }
    }

    fn static_map(&self, collision_map: &mut HashMap<Coord, Tile>) {
        for row in self.y..(self.y + self.height as i32) {
            for col in self.x..(self.x + self.width as i32) {
                let is_wall = self.is_wall(row, col);
                let is_door = self.is_door(row, col);

                // TODO: We probably don't need to loop through all of these again.
                if is_door {
                    collision_map.insert((col, row), Tile::new(true));
                    continue;
                } else if is_wall {
                    collision_map.insert((col, row), Tile::new(false));
                    continue;
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
