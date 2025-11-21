#![allow(unused)]

mod engine;
mod ui;

use crate::engine::game::Game;
use crate::engine::arrays::{default_array, find_array_by_name};
use crate::engine::ai;
use crate::engine::types::Army;
use crate::ui::app::{App, CurrentScreen};
use crate::ui::ui::{render, render_size_error};
use clap::Parser;
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

#[derive(Parser)]
#[command(name = "enoch")]
#[command(about = "Enochian Chess - Four-player chess variant", long_about = None)]
struct Args {
    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,
    
    /// Game state file
    #[arg(long)]
    state: Option<String>,
    
    /// Make a move
    #[arg(long, name = "move")]
    move_cmd: Option<String>,
    
    /// Validate a move without applying it
    #[arg(long)]
    validate: Option<String>,
    
    /// Analyze a square (show piece info and legal moves)
    #[arg(long)]
    analyze: Option<String>,
    
    /// Show board
    #[arg(long)]
    show: bool,
    
    /// Enable AI for armies (comma-separated)
    #[arg(long)]
    ai: Option<String>,
    
    /// Auto-play until game ends
    #[arg(long)]
    auto_play: bool,
    
    /// Show legal moves for army
    #[arg(long)]
    legal_moves: Option<String>,
    
    /// Show game status
    #[arg(long)]
    status: bool,
}

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
                        && (key.code == KeyCode::Char('c')
                            && key.modifiers.contains(event::KeyModifiers::CONTROL)
                            || key.code == KeyCode::Esc)
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
    let args = Args::parse();
    
    if args.headless {
        run_headless(args);
        Ok(())
    } else {
        let use_halfblocks = env::args().any(|arg| arg == "--halfblocks");
        run_tui(use_halfblocks)
    }
}

fn run_tui(use_halfblocks: bool) -> Result<(), io::Error> {
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
                // Handle Ctrl-C for immediate exit
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    return Ok(true);
                }
                
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
                        KeyCode::Esc => {
                            if app.selected_square.is_some() {
                                app.selected_square = None;
                                app.status_message = Some("Selection cleared".to_string());
                            } else {
                                app.current_screen = CurrentScreen::Exiting;
                            }
                        }
                        KeyCode::Char('u') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            app.undo();
                        }
                        KeyCode::Char('r') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            app.redo();
                        }
                        KeyCode::Char('1') => app.select_army(Army::Blue),
                        KeyCode::Char('2') => app.select_army(Army::Red),
                        KeyCode::Char('3') => app.select_army(Army::Black),
                        KeyCode::Char('4') => app.select_army(Army::Yellow),
                        KeyCode::Tab => app.cycle_selected_army(1),
                        KeyCode::BackTab => app.cycle_selected_army(-1),
                        KeyCode::Char(to_insert) => {
                            app.add_char(to_insert);
                        }
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Enter => {
                            let input = app.input.trim().to_string();
                            if !app.try_select_square(&input) {
                                app.submit_command();
                            }
                            app.input.clear();
                        }
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

fn run_headless(args: Args) {
    use crate::engine::game::Game;
    use crate::engine::arrays::{default_array, find_array_by_name};
    use crate::engine::ai;
    use std::fs;
    
    // Load or create game
    let mut game = if let Some(state_file) = &args.state {
        if let Ok(json) = fs::read_to_string(state_file) {
            Game::from_json(&json).unwrap_or_else(|_| Game::from_array_spec(default_array()))
        } else {
            Game::from_array_spec(default_array())
        }
    } else {
        Game::from_array_spec(default_array())
    };
    
    // Parse AI armies
    let ai_armies: Vec<Army> = if let Some(ai_str) = &args.ai {
        ai_str.split(',')
            .filter_map(|s| Army::from_str(s.trim()))
            .collect()
    } else {
        Vec::new()
    };
    
    // Validate move if provided
    if let Some(validate_cmd) = &args.validate {
        validate_move(&mut game, validate_cmd);
        return;
    }
    
    // Analyze square if provided
    if let Some(square_str) = &args.analyze {
        analyze_square(&mut game, square_str);
        return;
    }
    
    // Execute move if provided
    if let Some(move_cmd) = &args.move_cmd {
        if let Err(e) = execute_headless_move(&mut game, move_cmd, &args) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        
        // AI moves after player move
        make_ai_moves(&mut game, &ai_armies, &args);
    }
    
    // Auto-play mode
    if args.auto_play {
        auto_play(&mut game, &ai_armies, &args);
    }
    
    // Query commands
    if let Some(army_name) = &args.legal_moves {
        if let Some(army) = Army::from_str(army_name) {
            show_legal_moves(&mut game, army);
        }
    }
    
    if args.status {
        show_status(&game);
    }
    
    // Show board
    if args.show {
        show_board(&game);
    }
    
    // Save state
    if let Some(save_file) = &args.state {
        if let Ok(json) = game.to_json() {
            fs::write(save_file, json).ok();
        }
    }
}

fn execute_headless_move(game: &mut Game, move_cmd: &str, args: &Args) -> Result<(), String> {
    // Parse move command (format: "blue: e2-e4")
    let parts: Vec<&str> = move_cmd.split(':').collect();
    if parts.len() != 2 {
        return Err("Move must follow format 'army: e2-e4'".to_string());
    }
    
    let army = Army::from_str(parts[0].trim())
        .ok_or_else(|| "Unknown army".to_string())?;
    
    let move_part = parts[1].trim().replace('x', "-");
    let coords: Vec<&str> = move_part.split('-').collect();
    if coords.len() != 2 {
        return Err("Move must contain source and destination".to_string());
    }
    
    let from = parse_square_headless(coords[0].trim())?;
    let to = parse_square_headless(coords[1].trim())?;
    
    game.apply_move(army, from, to, None)?;
    
    println!("✓ {} moved from {} to {}", army.display_name(), coords[0], coords[1]);
    
    Ok(())
}

fn parse_square_headless(s: &str) -> Result<u8, String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() != 2 {
        return Err("Invalid square".to_string());
    }
    let file = chars[0].to_ascii_lowercase() as u8 - b'a';
    let rank = chars[1] as u8 - b'1';
    if file > 7 || rank > 7 {
        return Err("Square out of bounds".to_string());
    }
    Ok(rank * 8 + file)
}

fn make_ai_moves(game: &mut Game, ai_armies: &[Army], args: &Args) {
    loop {
        let current = game.current_army();
        if !ai_armies.contains(&current) {
            break;
        }
        
        if let Some(mv) = ai::capture_preferring_move(game, current) {
            let from_file = (b'a' + (mv.from % 8)) as char;
            let from_rank = (b'1' + (mv.from / 8)) as char;
            let to_file = (b'a' + (mv.to % 8)) as char;
            let to_rank = (b'1' + (mv.to / 8)) as char;
            
            game.apply_move(current, mv.from, mv.to, None).ok();
            
            println!("🤖 {} AI: {}{} -> {}{}", 
                current.display_name(), from_file, from_rank, to_file, to_rank);
        } else {
            break;
        }
        
        if game.winning_team().is_some() {
            break;
        }
    }
}

fn auto_play(game: &mut Game, ai_armies: &[Army], args: &Args) {
    let mut move_count = 0;
    
    while game.winning_team().is_none() && move_count < 500 {
        let current = game.current_army();
        
        if let Some(mv) = ai::capture_preferring_move(game, current) {
            let from_file = (b'a' + (mv.from % 8)) as char;
            let from_rank = (b'1' + (mv.from / 8)) as char;
            let to_file = (b'a' + (mv.to % 8)) as char;
            let to_rank = (b'1' + (mv.to / 8)) as char;
            
            game.apply_move(current, mv.from, mv.to, None).ok();
            move_count += 1;
            
            println!("{}. {}: {}{} -> {}{}", 
                move_count, current.display_name(), 
                from_file, from_rank, to_file, to_rank);
        } else {
            break;
        }
    }
    
    if let Some(team) = game.winning_team() {
        println!("\n🏆 {} TEAM WINS after {} moves!", team.name().to_uppercase(), move_count);
    } else {
        println!("\nGame ended after {} moves", move_count);
    }
}

fn show_legal_moves(game: &mut Game, army: Army) {
    let moves = game.legal_moves(army).to_vec();
    println!("Legal moves for {}:", army.display_name());
    for mv in moves {
        let from_file = (b'a' + (mv.from % 8)) as char;
        let from_rank = (b'1' + (mv.from / 8)) as char;
        let to_file = (b'a' + (mv.to % 8)) as char;
        let to_rank = (b'1' + (mv.to / 8)) as char;
        println!("  {}{} -> {}{}", from_file, from_rank, to_file, to_rank);
    }
}

fn show_status(game: &Game) {
    println!("Current turn: {}", game.current_army().display_name());
    
    for &army in Army::ALL.iter() {
        let status = if game.army_is_frozen(army) {
            "Frozen"
        } else if game.king_in_check(army) {
            "In Check"
        } else {
            "Active"
        };
        println!("  {}: {}", army.display_name(), status);
    }
    
    if let Some(team) = game.winning_team() {
        println!("\n🏆 Winner: {} team", team.name());
    }
}

fn show_board(game: &Game) {
    for row in game.board.ascii_rows() {
        println!("{}", row);
    }
}

fn analyze_square(game: &mut Game, square_str: &str) {
    let square = match parse_square_headless(square_str.trim()) {
        Ok(sq) => sq,
        Err(e) => {
            println!("❌ Invalid square: {}", e);
            process::exit(1);
        }
    };
    
    let file = (b'a' + (square % 8)) as char;
    let rank = (b'1' + (square / 8)) as char;
    
    println!("Analyzing {}{}", file, rank);
    println!();
    
    if let Some((army, kind)) = game.board.piece_at(square) {
        println!("Piece: {} {}", army.display_name(), kind.name());
        
        // Show if frozen
        if game.army_is_frozen(army) {
            println!("Status: Frozen");
        } else if game.king_in_check(army) && kind == crate::engine::types::PieceKind::King {
            println!("Status: In Check");
        } else {
            println!("Status: Active");
        }
        
        // Show legal moves from this square
        let all_moves = game.legal_moves(army).to_vec();
        let moves: Vec<_> = all_moves.iter()
            .filter(|m| m.from == square)
            .collect();
        
        if moves.is_empty() {
            println!("\nNo legal moves from this square");
        } else {
            println!("\nLegal moves ({}):", moves.len());
            for mv in moves {
                let to_file = (b'a' + (mv.to % 8)) as char;
                let to_rank = (b'1' + (mv.to / 8)) as char;
                
                if let Some((target_army, target_kind)) = game.board.piece_at(mv.to) {
                    println!("  {}{} (captures {} {})", to_file, to_rank, target_army.display_name(), target_kind.name());
                } else {
                    println!("  {}{}", to_file, to_rank);
                }
            }
        }
    } else {
        println!("Empty square");
    }
}

fn validate_move(game: &mut Game, move_cmd: &str) {
    let parts: Vec<&str> = move_cmd.split(':').collect();
    if parts.len() != 2 {
        println!("❌ Invalid format. Use: army: e2-e4");
        process::exit(1);
    }
    
    let army = match Army::from_str(parts[0].trim()) {
        Some(a) => a,
        None => {
            println!("❌ Unknown army: {}", parts[0].trim());
            process::exit(1);
        }
    };
    
    let move_part = parts[1].trim().replace('x', "-");
    let coords: Vec<&str> = move_part.split('-').collect();
    if coords.len() != 2 {
        println!("❌ Invalid move format. Use: e2-e4");
        process::exit(1);
    }
    
    let from = match parse_square_headless(coords[0].trim()) {
        Ok(sq) => sq,
        Err(e) => {
            println!("❌ Invalid source square: {}", e);
            process::exit(1);
        }
    };
    
    let to = match parse_square_headless(coords[1].trim()) {
        Ok(sq) => sq,
        Err(e) => {
            println!("❌ Invalid destination square: {}", e);
            process::exit(1);
        }
    };
    
    // Check if it's the army's turn
    if game.current_army() != army {
        println!("❌ Not {}'s turn (current: {})", 
            army.display_name(), game.current_army().display_name());
        process::exit(1);
    }
    
    // Check if army is frozen
    if game.army_is_frozen(army) {
        println!("❌ {} is frozen", army.display_name());
        process::exit(1);
    }
    
    // Check if move is legal
    if game.is_legal_move(army, from, to) {
        println!("✓ Valid move: {} {} → {}", 
            army.display_name(), coords[0], coords[1]);
        
        // Show what piece is moving
        if let Some((piece_army, piece_kind)) = game.board.piece_at(from) {
            println!("  Piece: {}", piece_kind.name());
            
            // Check if it's a capture
            if let Some((target_army, target_kind)) = game.board.piece_at(to) {
                println!("  Captures: {} {}", target_army.display_name(), target_kind.name());
            }
        }
    } else {
        println!("❌ Illegal move: {} {} → {}", 
            army.display_name(), coords[0], coords[1]);
        
        // Provide helpful context
        if let Some((piece_army, piece_kind)) = game.board.piece_at(from) {
            if piece_army != army {
                println!("  Reason: That piece belongs to {}", piece_army.display_name());
            } else {
                println!("  Reason: {} cannot move there", piece_kind.name());
            }
        } else {
            println!("  Reason: No piece at {}", coords[0]);
        }
        
        process::exit(1);
    }
}
