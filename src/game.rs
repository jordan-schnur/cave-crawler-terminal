use crate::camera::Camera;
use crate::drawable::Drawable;
use crate::frame::Frame;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

use crate::drawable::fps::Fps;
use crate::drawable::room::Room;
use crate::drawable::tree::Tree;
use crate::player::player;

pub struct Game {
    drawables: Vec<Box<dyn Drawable>>,

    pub player: player,
    pub request_exit: bool,
    fps: Fps,
    pub camera: Camera,
}

impl Game {
    pub fn new(view_width: u16, view_height: u16) -> Self {
        let camera = Camera::new(0, 0, view_width, view_height);

        // Create a room and a tree
        let room = Room {
            x: 2,
            y: 2,
            width: 50,
            height: 55,
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
            request_exit: false,
            fps,
            camera,
            player: player::new(10, 10),
        }
    }

    fn update_camera(&mut self, camera_width: u16, camera_height: u16) {
        let ui_height = (camera_height as f32 / 3.0).round() as u16;
        let game_height = camera_height - ui_height;

        self.camera.width = camera_width;
        self.camera.height = game_height;

        let half_w = camera_width as i32 / 2;
        let half_h = game_height as i32 / 2;

        self.camera.x = self.player.x - half_w;
        self.camera.y = self.player.y - half_h;

        self.camera.update_bbox();
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.fps.update();
        // Start by clearing the frame
        frame.clear();

        for drawable in &self.drawables {
            if self.camera.camera_view.intersects(&drawable.bound_box()) {
                drawable.draw(frame);
            }
        }

        // Draw the player (directly to frame or as another Drawable)
        if let Some((scr_x, scr_y)) = self.camera.world_to_screen(self.player.x, self.player.y) {
            frame.set_char(scr_x, scr_y, 'ɠ');
        }
    }

    pub fn update(&mut self, camera_width: u16, camera_height: u16) {
        let mut player_dx = 0;
        let mut player_dy = 0;
        if poll(Duration::from_millis(8)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => {
                        self.request_exit = true;
                    }
                    KeyCode::Left => {
                        // Move player left, if possible
                        player_dx = -1;
                    }
                    KeyCode::Right => {
                        player_dx = 1;
                    }
                    KeyCode::Up => {
                        player_dy = -1;
                    }
                    KeyCode::Down => {
                        player_dy = 1;
                    }
                    _ => {}
                }
            }
        }
        let mut temp_frame = Frame::new(self.camera.x, self.camera.y, camera_width, camera_height);
        temp_frame.clear();

        for drawable in &self.drawables {
            if self.camera.camera_view.intersects(&drawable.bound_box()) {
                drawable.draw(&mut temp_frame);
            }
        }

        self.player.attempt_move(player_dx, player_dy, &temp_frame, &self.camera);

        self.update_camera(camera_width, camera_height);
    }

    pub fn draw_ui(&self, frame: &mut Frame) {
        // Where the "UI" starts.
        let ui_start = frame.height - (frame.height / 3);
        // Or if you used the same calculation as update_camera, do:
        // let ui_height = (frame.height as f32 / 3.0).round() as u16;
        // let ui_start = frame.height - ui_height;

        for col in 0..frame.width {
            frame.set_char(col, ui_start, '—');
        }

        // Example: Write some text in the panel
        frame.draw_text(
            2,            // x
            ui_start + 1, // y
            "Action Log: Nothing yet...",
            None,
            None,
        );

        // If you want to display an equipped weapon, do:
        frame.draw_text(25, ui_start + 3, "Weapon: Rusty Sword", None, None);

        // Or show the FPS you’re already tracking:
        frame.draw_text(
            frame.width - 10,
            ui_start + 1,
            &format!("FPS: {}", self.fps.fps),
            None,
            None,
        );
    }
}
