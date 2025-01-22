use std::collections::HashMap;

pub type Coord = (i32, i32);

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    is_walkable: bool,
}

impl Tile {
    pub fn new(is_walkable: bool) -> Self {
        Tile { is_walkable }
    }
}