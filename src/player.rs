use crate::camera::Camera;
use crate::frame::Frame;
use crate::health::Health;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub health: Health
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            health: Health::new(100),
        }
    }

    pub fn attempt_move(&mut self, dx: i32, dy: i32, frame: &Frame, camera: &Camera) -> bool {
        let new_x =  self.x + dx;
        let new_y = self.y + dy;

        if let Some((scr_x, scr_y)) = camera.world_to_screen(new_x, new_y) {
            if frame.is_walkable(scr_x, scr_y) {
                self.x = new_x;
                self.y = new_y;

                return true;
            }
        }


        return false;
    }
}