use crate::bounding_box::BoundingBox;
use crate::camera::Camera;
use crate::drawable::Drawable;
use crate::frame::{Cell, Frame};
use crate::health::Health;
use crate::pathfinding::{bounding_box_for_path, find_path, Point};
use crate::player::Player;
use crate::tile::{Coord, Tile};
use crossterm::style::Color;
use std::collections::HashMap;

pub struct Goblin {
    pub x: i32,
    pub y: i32,
    pub health: Health,
    move_cooldown: u32,
    current_path: Option<Vec<Point>>,
    pub debug_mode: bool,
}

impl Goblin {
    pub fn new(x: i32, y: i32) -> Self {
        Goblin {
            x,
            y,
            move_cooldown: 0,
            current_path: None,
            debug_mode: true,
            health: Health::new(10),
        }
    }

    pub fn attack(&self, player: &mut Player) {
        player.health.take_damage(1);
    }

    pub fn update(&mut self, static_map: &HashMap<Coord, Tile>, player: &Player) {
        if self.move_cooldown > 0 {
            self.move_cooldown -= 1;

            return;
        }

        if self.current_path.is_none() || self.move_cooldown == 0 {
            let start = Point {
                x: self.x,
                y: self.y,
            };
            let goal = Point {
                x: player.x,
                y: player.y,
            };

            let is_walkable = |p: Point| {
                if let Some(tile) = static_map.get(&(p.x, p.y)) {
                    return tile.is_walkable();
                }

                return true;
            };

            self.current_path = find_path(start, goal, is_walkable);
        }

        // Move along the path if we have one
        if let Some(path) = &self.current_path {
            if path.len() > 1 { // If we have more than just our current position
                 // let next = path[1];  // Get the next position
                 // self.x = next.x;
                 // self.y = next.y;
                 // self.current_path = Some(path[1..].to_vec());  // Update path
            } else {
                self.current_path = None; // Clear path if we've reached the end
            }
        }

        self.move_cooldown = 10; // Wait 10 frames before next move


    }

    pub fn draw_health(&self, frame: &mut Frame) {
        let max_hearts = self.health.get_max();
        let filled_hearts = self.health.get_current();

        let hearts = format!("{}{}", "♥".to_string().repeat(filled_hearts as usize),
                             "♡".repeat((max_hearts - filled_hearts) as usize));

        // Draw health bar, centered on goblin by calculating the middle of the health bar
        let health_bar_x = self.x - (max_hearts / 2);
        let health_bar_y = self.y - 1;

        for (i, ch) in hearts.chars().enumerate() {
            frame.set_world_cell(health_bar_x + i as i32, health_bar_y, Cell {
                ch,
                fg: Some(Color::Blue),
                bg: None,
                is_walkable: true,
            });
        }

        // for i in 0..self.health.get_max() {
        //     let ch = if i < self.health.get_current() {
        //         '♥'
        //     } else {
        //         '♡'
        //     };
        //
        //     frame.set_world_cell(self.x + i, self.y - 1, Cell {
        //         ch,
        //         fg: Some(Color::Red),
        //         bg: None,
        //         is_walkable: true,
        //     });
        // }
    }
}

impl Drawable for Goblin {
    fn draw(&self, frame: &mut Frame) {
        frame.set_world_char(self.x, self.y, 'G');

        self.draw_health(frame);

        if self.debug_mode {
            if let Some(path) = &self.current_path {
                let skip_first = path.len() > 1;

                // Skip first and last points
                for point in path.iter().skip(if skip_first { 1 } else { 0 }).take(path.len() - 2) {
                    frame.set_world_cell(
                        point.x,
                        point.y,
                        crate::frame::Cell {
                            ch: '·',
                            fg: Some(Color::Yellow),
                            bg: None,
                            is_walkable: true,
                        },
                    );
                }
            }
        }
    }

    fn static_map(
        &self,
        _collision_map: &mut std::collections::HashMap<(i32, i32), crate::tile::Tile>,
    ) {
        // Do nothing
    }

    fn bound_box(&self) -> BoundingBox {
        if self.debug_mode {
            return bounding_box_for_path(
                &Point {
                    x: self.x,
                    y: self.y,
                },
                self.current_path.as_ref().map(|v| &v[..]),
            );
        }

        BoundingBox {
            left: self.x,
            right: self.x + 1,
            top: self.y,
            bottom: self.y + 1,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
