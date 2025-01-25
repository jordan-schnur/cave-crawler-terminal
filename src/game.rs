use crate::camera::Camera;
use crate::drawable::Drawable;
use crate::frame::Frame;
use crossterm::event::{poll, read, Event, KeyCode};
use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;
use crate::activity_log::ActivityLog;
use crate::drawable::fps::Fps;
use crate::drawable::room::Room;
use crate::drawable::tree::Tree;
use crate::enemy::goblin::Goblin;
use crate::player::Player;
use crate::tile::{Coord, Tile};

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMN OPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const RANDOM_SENTENCES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "The five boxing wizards jump quickly.",
    "Pack my box with five dozen liquor jugs.",
    "How razorback-jumping frogs can level six piqued gymnasts!",
    "Cozy lummox gives smart squid who asks for job pen.",
    "The jay, pig, fox, zebra, and my wolves quack!",
    "Sympathizing would fix Quaker objectives.",
    "A wizard's job is to vex chumps quickly in fog.",
    "Watch \"Jeopardy!\", Alex Trebek's fun TV quiz game.",
    "By Jove, my quick study of lexicography won a prize!",
    "Waltz, bad nymph for quick jigs vex!",
    "Crazy Fredrick bought many very exquisite opal jewels.",

];

pub struct Game {
    drawables: Vec<Box<dyn Drawable>>,
    pub player: Player,
    pub request_exit: bool,
    fps: Fps,
    pub camera: Camera,
    static_map: HashMap<Coord, Tile>,
    activity_log: ActivityLog,
    window_resized: bool,
}

impl Game {
    pub fn new(view_width: u16, view_height: u16) -> Self {
        let camera = Camera::new(0, 0, view_width, view_height);

        let goblin = Goblin::new(20, 10);

        let room = Room {
            x: 2,
            y: 2,
            width: 50,
            height: 55,
        };
        let tree = Tree { x: 15, y: 15 };
        let tree2 = Tree { x: 16, y: 15 };
        let tree3 = Tree { x: 17, y: 15 };
        let tree4 = Tree { x: 18, y: 15 };
        let tree5 = Tree { x: 18, y: 16 };
        let tree6 = Tree { x: 18, y: 17 };
        let tree7 = Tree { x: 18, y: 18 };
        let fps = Fps {
            last_frame: std::time::Instant::now(),
            frames: 0,
            fps: 0,
        };

        let drawables: Vec<Box<dyn Drawable>> = vec![
            Box::new(room),
            Box::new(tree),
            Box::new(tree2),
            Box::new(tree3),
            Box::new(tree4),
            Box::new(tree5),
            Box::new(tree6),
            Box::new(tree7),
            Box::new(goblin),
        ];

        let mut static_map: HashMap<Coord, Tile> = HashMap::new();

        for drawables in &drawables {
            drawables.static_map(&mut static_map);
        }

        Self {
            drawables,
            static_map,
            request_exit: false,
            fps,
            camera,
            player: Player::new(10, 10),
            activity_log: ActivityLog::new(0, 0, view_width, view_height / 3),
            window_resized: true,
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
        frame.clear();

        for drawable in &self.drawables {
            if self.camera.camera_view.intersects(&drawable.bound_box()) {
                drawable.draw(frame);
            }
        }

        if self.window_resized {
            let ui_start = frame.height - (frame.height / 3);

            self.activity_log.update_dimensions(0, ui_start + 1, 80, frame.height - ui_start - 1);
            self.window_resized = false;
        }

        self.activity_log.draw(frame);

        if let Some((scr_x, scr_y)) = self.camera.world_to_screen(self.player.x, self.player.y) {
            frame.set_char(scr_x, scr_y, '@');
        }
    }

    pub fn update(&mut self, camera_width: u16, camera_height: u16) {
        let mut player_dx = 0;
        let mut player_dy = 0;
        let mut damage_goblin = false;
        let mut write_to_log = false;

        // Check to see if the window has been resized
        if self.camera.height != camera_height || self.camera.width != camera_width {
            self.window_resized = true;
        }

        if poll(Duration::from_millis(8)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => {
                        self.request_exit = true;
                    }
                    KeyCode::Left => {
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
                    },
                    KeyCode::Char('d') => {
                        damage_goblin = true;
                    },
                    KeyCode::Char('t') => {
                       write_to_log = true;
                    }
                    _ => {}
                }
            }
        }

        if player_dx != 0 || player_dy != 0 {
            if self
                .player
                .attempt_move(player_dx, player_dy, &self.static_map)
            {
                self.player.health.take_damage(1);
            }
        }

        for drawable in &mut self.drawables {
            if let Some(goblin) = drawable.as_any_mut().downcast_mut::<Goblin>() {
                goblin.update(&self.static_map, &self.player);
                if damage_goblin {
                    goblin.health.take_damage(1);
                }
            }
        }

        if write_to_log {
            // Select a random sentence from the list
            let sentence = RANDOM_SENTENCES[rand::thread_rng().gen_range(0..RANDOM_SENTENCES.len())];

            self.activity_log.add_entry(sentence);
        }

        self.update_camera(camera_width, camera_height);
    }

    pub fn draw_ui(&self, frame: &mut Frame) {
        let ui_start = frame.height - (frame.height / 3);
        let middle = frame.width / 2;

        for col in 0..frame.width {
            frame.set_char(col, ui_start, '—');
        }

        frame.draw_text(
            middle,
            ui_start + 1,
            &format!("Health: {}", self.player.health),
            None,
            None,
        );

        frame.draw_text(25, ui_start + 3, "Weapon: Rusty Sword", None, None);

        frame.draw_text(
            frame.width - 10,
            ui_start + 1,
            &format!("FPS: {}", self.fps.fps),
            None,
            None,
        );
    }
}
