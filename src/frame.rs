use std::cmp;
use crossterm::style::Color;

pub struct Frame {
    pub width: u16,
    pub height: u16,

    pub buffer: Vec<Cell>,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        Frame {
            width,
            height,
            buffer: vec![Cell {
                ch: ' ',
                fg: None,
                bg: None,
            }; (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u16, y: u16, ch: char, fg: Option<Color>, bg: Option<Color>) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.buffer[index] = Cell { ch, fg, bg };
        }
    }

    pub fn set_char(&mut self, x: u16, y: u16, ch: char) {
        self.set(x, y, ch, None, None);
    }

    pub fn get_char(&self, x: u16, y: u16) -> Option<Cell> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        for cell in self.buffer.iter_mut() {
            cell.ch = ' ';
            cell.fg = None;
            cell.bg = None;
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, fg: Option<Color>, bg: Option<Color>) {
        let mut col = x;
        for ch in text.chars() {
            self.set(col, y, ch, fg, bg);
            col += 1;
            if col >= self.width {
                break;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}