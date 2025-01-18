use crossterm::style::Color;
use crate::drawable::Drawable;
use crate::frame::Frame;

pub struct Tree {
    pub x: u16,
    pub y: u16,
}

impl Drawable for Tree {
    fn draw(&self, frame: &mut Frame) {
        // A single 'T' at (x, y).
        frame.set(self.x, self.y, 'T', Some(Color::Green), None);
    }
}