use crate::bounding_box::BoundingBox;

pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,

    pub camera_view: BoundingBox,
}

impl Camera {
    pub fn new(x: i32, y: i32, width: u16, height: u16) -> Self {
        Camera {
            x,
            y,
            width,
            height,
            camera_view: BoundingBox {
                left: x,
                right: x + width as i32,
                top: y,
                bottom: y + height as i32,
            },
        }
    }
    pub fn world_to_screen(&self, world_x: i32, world_y: i32) -> Option<(u16, u16)> {
        let screen_x = world_x - self.x;
        let screen_y = world_y - self.y;

        if screen_x >= 0
            && screen_x < self.width as i32
            && screen_y >= 0
            && screen_y < self.height as i32
        {
            Some((screen_x as u16, screen_y as u16))
        } else {
            None
        }
    }

    /// Returns whether a world rectangle intersects the camera.
    pub fn is_visible(&self, world_x: i32, world_y: i32, width: u16, height: u16) -> bool {
        let right = self.x + self.width as i32;
        let bottom = self.y + self.height as i32;

        !(world_x + width as i32 <= self.x
            || world_x >= right
            || world_y + height as i32 <= self.y
            || world_y >= bottom)
    }

    pub fn update_bbox(&mut self) {
        self.camera_view = BoundingBox {
            left: self.x,
            right: self.x + self.width as i32,
            top: self.y,
            bottom: self.y + self.height as i32,
        };
    }
}
