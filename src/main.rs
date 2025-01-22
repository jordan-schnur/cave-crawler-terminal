mod bounding_box;
mod camera;
mod drawable;
mod frame;
mod game;
mod player;
mod health;
mod moveable;
mod enemy;
mod pathfinding;
mod tile;
mod collideable;

use crate::frame::Frame;
use crate::game::Game;
use crossterm::cursor::{Hide, Show};
use crossterm::style::{ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate, SetTitle};
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, Result, Write};
use std::time::Duration;

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Enable raw mode and switch to the alternate screen
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::Purge), SetTitle("Cave Diver Terminal"))?;

    let (width, height) = size()?;

    let mut game = Game::new(width, height);

    loop {
        execute!(&mut stdout, BeginSynchronizedUpdate)?;
        let (width, height) = size()?;

        game.update(width, height);

        let mut frame = Frame::new(game.camera.x, game.camera.y, width, height);

        game.draw(&mut frame);
        game.draw_ui(&mut frame);

        render_frame(&mut stdout, &frame)?;

        stdout.flush()?;

        execute!(&mut stdout, EndSynchronizedUpdate)?;

        if game.request_exit {
            break;
        }
    }

    // Clean up
    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn render_frame(stdout: &mut std::io::Stdout, frame: &Frame) -> Result<()> {
    use crossterm::{
        cursor::MoveTo,
        style::Print,
        terminal::{Clear, ClearType},
    };

    // Clear the screen so we can draw fresh
    execute!(stdout, Clear(ClearType::Purge))?;

    // Loop through our buffer and print each character
    let mut x = 0;
    let mut y = 0;

    for (i, cell) in frame.buffer.iter().enumerate() {
        x = (i as u16) % frame.width;
        y = (i as u16) / frame.width;

        execute!(stdout, MoveTo(x, y))?;

        // If there's a foreground color, set it
        if let Some(fg_color) = cell.fg {
            execute!(stdout, SetForegroundColor(fg_color))?;
        }
        // If there's a background color, set it
        if let Some(bg_color) = cell.bg {
            execute!(stdout, SetBackgroundColor(bg_color))?;
        }

        execute!(stdout, Print(cell.ch))?;

        // Reset color for next cell
        // (if you don't reset, cells below might inherit the color)
        if cell.fg.is_some() || cell.bg.is_some() {
            execute!(stdout, ResetColor)?;
        }
    }

    Ok(())
}
