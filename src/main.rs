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

pub const MIN_WIDTH: u16 = 132;
pub const MIN_HEIGHT: u16 = 46;

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
                match key.code {
                    KeyCode::Char(']') => {
                        app.cycle_array_direction(1);
                        continue;
                    }
                    KeyCode::Char('[') => {
                        app.cycle_array_direction(-1);
                        continue;
                    }
                    KeyCode::Char('?') | KeyCode::F(1) => {
                        if matches!(app.current_screen, CurrentScreen::Main) {
                            app.current_screen = CurrentScreen::Help;
                            app.help_scroll = 0;
                        }
                        continue;
                    }
                    _ => {}
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
                        KeyCode::Char(to_insert) => app.add_char(to_insert),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Enter => app.submit_command(),
                        _ => {}
                    },
                    CurrentScreen::Help => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.scroll_help(-1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.scroll_help(1);
                        }
                        KeyCode::PageUp => {
                            app.scroll_help(-10);
                        }
                        KeyCode::PageDown => {
                            app.scroll_help(10);
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => return Ok(true),
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
