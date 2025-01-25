use std::collections::VecDeque;
use crossterm::style::Color;
use textwrap::wrap;
use crate::frame::Frame;

struct Message {
    lines: Vec<String>,
}

impl Message {
    fn new(text: &str, max_width: usize) -> Self {
        let wrapped_lines = ActivityLog::wrap_text(text, max_width);
        Message { lines: wrapped_lines }
    }
}

/// ActivityLog manages and displays a list of activity texts.
pub struct ActivityLog {
    entries: VecDeque<Message>,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl ActivityLog {
    /// Creates a new ActivityLog with specified position and dimensions.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        ActivityLog {
            entries: VecDeque::new(),
            x,
            y,
            width,
            height,
        }
    }

    /// Adds a new entry to the activity log.
    pub fn add_entry(&mut self, text: &str) {
        let message = Message::new(text, self.width as usize);

        self.entries.push_back(message);

        self.trim_entries();
    }

    /// Draws the activity log onto the frame at the current position and dimensions.
    pub fn draw(&self, frame: &mut Frame) {
        let mut all_lines = Vec::new();

        // Collect all lines from all messages
        for message in &self.entries {
            for line in &message.lines {
                all_lines.push(line.clone());
            }
        }

        // Take the last `height` lines to display
        let start = if all_lines.len() > self.height as usize {
            all_lines.len() - self.height as usize
        } else {
            0
        };

        let lines_to_draw = &all_lines[start..];

        // Draw each line sequentially
        for (i, line) in lines_to_draw.iter().enumerate() {
            let current_y = self.y + i as u16;
            frame.draw_text(self.x, current_y, line, Some(Color::White), None);
        }
    }


    /// Wraps text into lines not exceeding the specified width.
    fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
        wrap(text, max_width)
            .iter()
            .map(|line| line.to_string())
            .collect()
    }

    /// Updates the position and dimensions of the ActivityLog.
    /// This should be called when the window is resized.
    pub fn update_dimensions(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;

        let mut rewrapped_entries = VecDeque::new();

        for message in &self.entries {
            let rewrapped_message = Message::new(&message.lines.join(" "), width as usize);
            rewrapped_entries.push_back(rewrapped_message);
        }

        self.entries = rewrapped_entries;

        self.trim_entries();
    }

    fn trim_entries(&mut self) {
        let mut total_lines = self.entries.iter().map(|m| m.lines.len()).sum::<usize>();

        while total_lines as u16 > self.height && !self.entries.is_empty() {
            if let Some(oldest) = self.entries.pop_front() {
                total_lines -= oldest.lines.len();
            }
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::frame::Frame;
    use super::*;

    #[test]
    fn test_activity_log() {
        let mut log = ActivityLog::new(10, 5, 20, 5);
        log.add_entry("Player attacks the goblin for 15 damage.");
        log.add_entry("Goblin retaliates with a claw attack.");
        log.add_entry("Player uses a health potion.");
        log.add_entry("Goblin is defeated!");

        let mut frame = Frame::new(0, 0, 80, 24);

        log.draw(&mut frame);
        // Expected Output:
        // Drawing at (10, 5): Player attacks the
        // Drawing at (10, 6): goblin for 15
        // Drawing at (10, 7): damage.
        // Drawing at (10, 8): Goblin retaliates
        // Drawing at (10, 9): with a claw
        // Drawing at (10, 10): attack.
        // Since height is 5, only the last 5 lines should be drawn.
    }
}