mod frame;
mod drawable;
mod game;

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, Write, Result};
use std::time::Duration;
use crossterm::cursor::{Hide, Show};
use crossterm::style::{ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::frame::Frame;
use crate::game::Game;

fn main() -> Result<()> {
    let mut stdout = stdout();

    // Enable raw mode and switch to the alternate screen
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::Purge))?;

    let mut game = Game::new();

    loop {
        // execute!(stdout, Clear(ClearType::Purge))?;

        if poll(Duration::from_millis(8))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        game.request_exit = true;
                    }
                    KeyCode::Left => {
                        // Move player left, if possible
                        if game.player_x > 0 {
                            game.player_x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        game.player_x += 1;
                    }
                    KeyCode::Up => {
                        if game.player_y > 0 {
                            game.player_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        game.player_y += 1;
                    }
                    _ => {}
                }
            }
        }

        let (width, height) = size()?;
        let mut frame = Frame::new(width, height);


        // Manually overwrite every cell with a space
        // for row in 0..height {
        //     for col in 0..width {
        //         execute!(stdout, MoveTo(col, row), Print(" "))?;
        //     }
        // }

        // game.handle_input().expect("TODO: panic message");

        game.update();

        game.draw(&mut frame);

        render_frame(&mut stdout, &frame)?;

        stdout.flush()?;

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