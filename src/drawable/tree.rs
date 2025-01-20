use crate::drawable::{BoundingBox, Drawable};
use crate::frame::{Cell, Frame};
use crossterm::style::Color;

pub struct Tree {
    pub x: i32,
    pub y: i32,
}

impl Drawable for Tree {
    fn draw(&self, frame: &mut Frame) {
        // A single 'T' at (x, y).
        frame.set_world_cell(
            self.x,
            self.y,
            Cell {
                ch: 'T',
                fg: Some(Color::Green),
                bg: None,
            },
        );
    }

    fn bound_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.x as i32,
            right: self.x as i32,
            top: self.y as i32,
            bottom: self.y as i32,
        }
    }
}
