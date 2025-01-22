use crate::camera::Camera;
use crate::frame::Frame;
use crate::health::Health;
use crate::tile::{Coord, Tile};
use std::collections::HashMap;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub health: Health,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            health: Health::new(100),
        }
    }

    pub fn attempt_move(&mut self, dx: i32, dy: i32, static_map: &HashMap<Coord, Tile>) -> bool {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        // Static map only stores non-walkable tiles, therefore if the tile is not in the map, it is walkable
        if let Some(tile) = static_map.get(&(new_x, new_y)) {
            if !tile.is_walkable() {
                return false;
            }
        }

        self.x = new_x;
        self.y = new_y;

        return true;
    }
}
