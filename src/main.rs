#![allow(unused)]

mod engine;
mod ui;

use crate::ui::app::{App, CurrentScreen};
use crate::ui::ui::{render, render_size_error};
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, terminal, ExecutableCommand};
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, Terminal};
use std::io::{stdout, Error, ErrorKind, Stdout};
use std::{env, io, process};

pub const MIN_WIDTH: u16 = 80;
pub const MIN_HEIGHT: u16 = 24;

fn check_size(terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
    let size = terminal.size()?;
    if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
        terminal.clear();
        let area = Rect::new(0, 0, size.width, size.height);
        terminal.draw(|frame| render_size_error(frame, MIN_WIDTH, MIN_HEIGHT, area))?;

        loop {
            match event::read()? {
                Event::Resize(new_width, new_height) => {
                    if new_width >= MIN_WIDTH && new_height >= MIN_HEIGHT {
                        return Ok(());
                    }
                }
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('c')
                        && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        || key.code == KeyCode::Esc
                    {
                        ratatui::restore();
                        process::exit(0);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let use_halfblocks = args.contains(&"--halfblocks".to_string());
    let mut terminal = ratatui::init();
    let mut app = App::new(use_halfblocks);
    run(&mut terminal, &mut app)?;
    ratatui::restore();
    Ok(())
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<bool> {
    loop {
        check_size(terminal)?;
        terminal.hide_cursor()?;
        terminal.draw(|frame| render(frame, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // Global keys
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    KeyCode::Char(']') => {
                        app.cycle_array_direction(1);
                        continue;
                    }
                    KeyCode::Char('[') => {
                        app.cycle_array_direction(-1);
                        continue;
                    }
                    _ => {}
                }

                match app.current_screen {
                    CurrentScreen::Main => {
                        match app.input_mode {
                            crate::ui::app::InputMode::Normal => match key.code {
                                KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
                                KeyCode::Tab | KeyCode::Down | KeyCode::Up | KeyCode::Left | KeyCode::Right => {
                                    app.input_mode = crate::ui::app::InputMode::Board;
                                }
                                KeyCode::Char(to_insert) => app.add_char(to_insert),
                                KeyCode::Backspace => app.delete_char(),
                                KeyCode::Enter => app.submit_command(),
                                _ => {}
                            },
                            crate::ui::app::InputMode::Board => match key.code {
                                KeyCode::Esc => app.handle_board_esc(),
                                KeyCode::Left => app.move_cursor(-1, 0),
                                KeyCode::Right => app.move_cursor(1, 0),
                                KeyCode::Up => app.move_cursor(0, 1),
                                KeyCode::Down => app.move_cursor(0, -1),
                                KeyCode::Enter | KeyCode::Char(' ') => app.handle_board_enter(),
                                KeyCode::Tab => app.input_mode = crate::ui::app::InputMode::Normal,
                                KeyCode::Char('/') => {
                                    app.input_mode = crate::ui::app::InputMode::Normal;
                                    app.add_char('/');
                                }
                                _ => {}
                            },
                        }
                    }
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => return Ok(true),
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
