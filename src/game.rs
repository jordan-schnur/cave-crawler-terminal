use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};
use crate::drawable::Drawable;
use crate::frame::Frame;

use crate::drawable::room::Room;
use crate::drawable::tree::Tree;
use crate::drawable::fps::Fps;

pub struct Game {
    drawables: Vec<Box<dyn Drawable>>,

    pub player_x: u16,
    pub player_y: u16,
    pub request_exit: bool,
    fps: Fps,
}

impl Game {
    pub fn new() -> Self {
        // Create a room and a tree
        let room = Room {
            x: 2,
            y: 2,
            width: 10,
            height: 6,
        };
        let tree = Tree { x: 5, y: 5 };
        let fps = Fps {
            last_frame: std::time::Instant::now(),
            frames: 0,
            fps: 0,
        };

        Self {
            drawables: vec![
                Box::new(room),
                Box::new(tree),
                // Add more objects here
            ],
            player_x: 0,
            player_y: 0,
            request_exit: false,
            fps
        }

    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.fps.update();
        // Start by clearing the frame
        frame.clear();

        // Draw all the objects
        for drawable in &self.drawables {
            drawable.draw(frame);
        }

        // Now draw the player
        frame.set_char(self.player_x, self.player_y, '@');

        self.fps.draw(frame);
    }

    pub fn update(&mut self) {

    }

    // fn handle_input(&mut self) -> std::io::Result<bool> {
    //     if poll(Duration::from_millis(16))? {
    //         if let Event::Key(key_event) = read()? {
    //             match key_event.code {
    //                 KeyCode::Esc => self.request_exit = true,
    //                 KeyCode::Left => {
    //                     self.try_move(self.player_x.saturating_sub(1), self.player_y);
    //                 }
    //                 KeyCode::Right => {
    //                     self.try_move(self.player_x + 1, self.player_y);
    //                 }
    //                 KeyCode::Up => {
    //                     self.try_move(self.player_x, self.player_y.saturating_sub(1));
    //                 }
    //                 KeyCode::Down => {
    //                     self.try_move(self.player_x, self.player_y + 1);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     Ok(false)
    // }
    //
    // fn try_move(&mut self, new_x: u16, new_y: u16) {
    //     // Make sure we're within map bounds
    //     if new_y as usize >= self.map.len() {
    //         return;
    //     }
    //     if new_x as usize >= self.map[new_y as usize].len() {
    //         return;
    //     }
    //
    //     // Check if destination is a wall
    //     if self.map[new_y as usize].chars().nth(new_x as usize).unwrap() == '#' {
    //         // It's a wall, so ignore
    //         return;
    //     }
    //
    //     // Otherwise, we can move
    //     self.player_x = new_x;
    //     self.player_y = new_y;
    // }
}
