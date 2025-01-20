use crate::drawable::{BoundingBox, Drawable};
use crate::frame::Frame;

pub struct Fps {
    pub last_frame: std::time::Instant,
    pub frames: u32,
    pub fps: u32,
}

impl Fps {
    pub fn update(&mut self) {
        // Calculate FPS.
        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame;
        if elapsed.as_secs() > 0 {
            self.fps = self.frames / elapsed.as_secs() as u32;
            self.last_frame = now;
            self.frames = 0;
        }
        self.frames += 1;
    }
}

impl Drawable for Fps {
    fn draw(&self, frame: &mut Frame) {
        // Draw the FPS counter at (x, y).
        frame.draw_text(
            frame.width - 10,
            0,
            &format!("FPS: {}", self.fps),
            None,
            None,
        );
    }

    fn bound_box(&self) -> BoundingBox {
        BoundingBox {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }
}
