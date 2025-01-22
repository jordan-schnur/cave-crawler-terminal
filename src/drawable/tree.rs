use crate::bounding_box::BoundingBox;
use crate::drawable::Drawable;
use crate::frame::{Cell, Frame};
use crate::tile::{Coord, Tile};
use crossterm::style::Color;
use std::collections::HashMap;

pub struct Tree {
    pub x: i32,
    pub y: i32,
}

impl Drawable for Tree {
    fn draw(&self, frame: &mut Frame) {
        frame.set_world_cell(
            self.x,
            self.y,
            Cell {
                ch: 'T',
                fg: Some(Color::Green),
                bg: None,
                is_walkable: false,
            },
        );
    }

    fn static_map(&self, collision_map: &mut HashMap<Coord, Tile>) {
        collision_map.insert((self.x, self.y), Tile::new(false));
    }

    fn bound_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.x as i32,
            right: self.x as i32,
            top: self.y as i32,
            bottom: self.y as i32,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
