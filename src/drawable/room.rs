
use crate::drawable::Drawable;
use crate::frame::Frame;

pub struct Room {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
}

impl Drawable for Room {
    fn draw(&self, frame: &mut Frame) {
        // Outline a rectangle: walls on the boundary, '.' for floors inside.
        for row in self.y..(self.y + self.height) {
            for col in self.x..(self.x + self.width) {
                let is_wall = row == self.y
                    || row == (self.y + self.height - 1)
                    || col == self.x
                    || col == (self.x + self.width - 1);

                let ch = if is_wall { '#' } else { '.' };
                frame.set_char(col, row, ch);
            }
        }
    }
}