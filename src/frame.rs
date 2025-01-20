use std::cmp;
use crossterm::style::Color;
use crate::camera::Camera;

pub struct Frame {
    pub width: u16,
    pub height: u16,

    pub buffer: Vec<Cell>,
    cam_x: i32,
    cam_y: i32,
}

impl Frame {
    pub fn new(cam_x: i32, cam_y: i32, width: u16, height: u16) -> Self {
        Frame {
            width,
            height,
            buffer: vec![Cell {
                ch: ' ',
                fg: None,
                bg: None,
            }; (width * height) as usize],
            cam_x,
            cam_y
        }
    }

    pub fn set_world_char(&mut self, world_x: i32, world_y: i32, ch: char) {
        self.set_world_cell(world_x, world_y, Cell {
            ch,
            fg: None,
            bg: None,
        })
    }

    pub fn set_world_cell(&mut self, world_x: i32, world_y: i32, cell: Cell) {
        let screen_x = world_x - self.cam_x;
        let screen_y = world_y - self.cam_y;

        // Figure out how tall the game region is
        let game_height = self.height - (self.height / 3);

        // Now, ensure we only draw if it's within the top game region
        if screen_x < 0 || screen_x as u16 >= self.width {
            return;
        }
        if screen_y < 0 || screen_y as u16 >= game_height {
            return;
        }

        let idx = (screen_y as u16 * self.width + screen_x as u16) as usize;
        self.buffer[idx] = cell;
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