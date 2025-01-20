use crate::bounding_box::BoundingBox;
use crate::drawable::{Drawable};
use crate::frame::Frame;
use crate::health::Health;
use crate::player::Player;

pub struct Goblin {
    pub x: i32,
    pub y: i32,
    pub health: Health,
}

impl Goblin {
    pub fn new(x: i32, y: i32) -> Self {
        Goblin {
            x,
            y,
            health: Health::new(10),
        }
    }

    pub fn attack(&self, player: &mut Player) {
        player.health.take_damage(1);
    }
}

impl Drawable for Goblin {
    fn draw(&self, frame: &mut Frame) {
        frame.set_world_char(self.x, self.y, 'G');
    }

    fn bound_box(&self) -> BoundingBox {
        BoundingBox {
            left: self.x,
            right: self.x + 1,
            top: self.y,
            bottom: self.y + 1,
        }
    }
}