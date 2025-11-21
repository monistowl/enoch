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
#[command(version)]
#[command(about = "Enochian Chess - Four-player chess variant", long_about = None)]
#[command(after_help = "EXAMPLES:
    # Start interactive game
    enoch

    # Validate a move
    enoch --headless --validate \"blue: e2-e3\"

    # Make moves and save
    enoch --headless --move \"blue: e2-e3\" --state game.json

    # Interactive analysis
    enoch --headless --interactive --state game.json

    # Export to PGN
    enoch --headless --state game.json --export-pgn game.pgn

    # Run batch commands
    enoch --headless --batch commands.txt --state game.json

For more information, see README.md or visit https://github.com/monistowl/enoch")]
struct Args {
    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,
    
    /// Game state file
    #[arg(long, value_name = "FILE")]
    state: Option<String>,
    
    // === Move Operations ===
    
    /// Make a move (format: "army: from-to")
    #[arg(long, value_name = "MOVE")]
    move_cmd: Option<String>,
    
    /// Validate a move without applying it
    #[arg(long, value_name = "MOVE")]
    validate: Option<String>,
    
    /// Undo last N moves (default 1)
    #[arg(long, value_name = "N")]
    undo: Option<usize>,
    
    // === Analysis Tools ===
    
    /// Analyze a square (show piece info and legal moves)
    #[arg(long, value_name = "SQUARE")]
    analyze: Option<String>,
    
    /// Query rules (e.g., "queen capture queen", "promotion")
    #[arg(long, value_name = "QUERY")]
    query: Option<String>,
    
    /// Evaluate position (material, mobility, status)
    #[arg(long)]
    evaluate: bool,
    
    /// Show game statistics
    #[arg(long)]
    stats: bool,
    
    /// Show legal moves for army
    #[arg(long, value_name = "ARMY")]
    legal_moves: Option<String>,
    
    // === Position Setup ===
    
    /// Generate custom position (format: "Kb1,Qc2:blue Ke8:red")
    #[arg(long, value_name = "POSITION")]
    generate: Option<String>,
    
    /// List all available starting arrays
    #[arg(long)]
    list_arrays: bool,
    
    /// Start with specific array
    #[arg(long, value_name = "NAME")]
    array: Option<String>,
    
    // === Game I/O ===
    
    /// Export game in PGN-like format
    #[arg(long, value_name = "FILE")]
    export_pgn: Option<String>,
    
    /// Import game from PGN format
    #[arg(long, value_name = "FILE")]
    import_pgn: Option<String>,
    
    /// Convert format (json, ascii, compact)
    #[arg(long, value_name = "FORMAT")]
    convert: Option<String>,
    
    // === Modes ===
    
    /// Interactive REPL mode
    #[arg(long)]
    interactive: bool,
    
    /// Execute commands from file
    #[arg(long, value_name = "FILE")]
    batch: Option<String>,
    
    // === AI & Automation ===
    
    /// Enable AI for armies (comma-separated)
    #[arg(long, value_name = "ARMIES")]
    ai: Option<String>,
    
    /// Auto-play until game ends
    #[arg(long)]
    auto_play: bool,
    
    /// Performance test: count positions at depth N
    #[arg(long, value_name = "DEPTH")]
    perft: Option<u8>,
    
    // === Display ===
    
    /// Show board
    #[arg(long)]
    show: bool,
    
    /// Show move history
    #[arg(long)]
    history: bool,
    
    /// Show game status
    #[arg(long)]
    status: bool,
    
    /// Suppress non-essential output
    #[arg(long, short)]
    quiet: bool,
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
    use crate::engine::arrays::{default_array, find_array_by_name, available_arrays};
    use crate::engine::ai;
    use std::fs;
    
    // Handle list-arrays command first (doesn't need game state)
    if args.list_arrays {
        list_arrays();
        return;
    }
    
    // Handle generate command first (doesn't need existing game)
    if let Some(gen_str) = &args.generate {
        generate_position(gen_str, &args);
        return;
    }
    
    // Load or create game
    let mut game = if let Some(state_file) = &args.state {
        if let Ok(json) = fs::read_to_string(state_file) {
            Game::from_json(&json).unwrap_or_else(|_| {
                let array = if let Some(array_name) = &args.array {
                    find_array_by_name(array_name).unwrap_or_else(|| {
                        eprintln!("❌ Unknown array: {}", array_name);
                        eprintln!("Use --list-arrays to see available options");
                        process::exit(1);
                    })
                } else {
                    default_array()
                };
                Game::from_array_spec(array)
            })
        } else {
            let array = if let Some(array_name) = &args.array {
                find_array_by_name(array_name).unwrap_or_else(|| {
                    eprintln!("❌ Unknown array: {}", array_name);
                    eprintln!("Use --list-arrays to see available options");
                    process::exit(1);
                })
            } else {
                default_array()
            };
            Game::from_array_spec(array)
        }
    } else {
        let array = if let Some(array_name) = &args.array {
            find_array_by_name(array_name).unwrap_or_else(|| {
                eprintln!("❌ Unknown array: {}", array_name);
                eprintln!("Use --list-arrays to see available options");
                process::exit(1);
            })
        } else {
            default_array()
        };
        Game::from_array_spec(array)
    };
    
    // Import PGN if provided
    if let Some(pgn_file) = &args.import_pgn {
        game = import_pgn(pgn_file);
        // Save to state file if provided
        if let Some(save_file) = &args.state {
            if let Ok(json) = game.to_json() {
                fs::write(save_file, json).ok();
                println!("Imported and saved to {}", save_file);
            }
        }
    }
    
    // Parse AI armies
    let ai_armies: Vec<Army> = if let Some(ai_str) = &args.ai {
        ai_str.split(',')
            .filter_map(|s| Army::from_str(s.trim()))
            .collect()
    } else {
        Vec::new()
    };
    
    // Interactive mode
    if args.interactive {
        run_interactive(&mut game, &ai_armies, &args);
        return;
    }
    
    // Batch mode
    if let Some(batch_file) = &args.batch {
        run_batch(&mut game, batch_file, &args);
        return;
    }
    
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
    
    // Query rules if provided
    if let Some(query_str) = &args.query {
        query_rules(query_str);
        return;
    }
    
    // Perft if provided
    if let Some(depth) = args.perft {
        run_perft(&mut game, depth);
        return;
    }
    
    // Convert format if provided
    if let Some(format) = &args.convert {
        convert_format(&game, format);
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
    
    // Undo moves if requested
    if let Some(count) = args.undo {
        match game.undo(count) {
            Ok(undone) => {
                if !args.quiet {
                    println!("Undid {} move(s)", undone);
                }
                // Save state after undo
                if let Some(save_file) = &args.state {
                    if let Ok(json) = game.to_json() {
                        std::fs::write(save_file, json).ok();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
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
    
    if args.history {
        show_history(&game);
    }
    
    if args.evaluate {
        evaluate_position(&mut game);
    }
    
    if args.stats {
        show_stats(&game);
    }
    
    if let Some(output_file) = &args.export_pgn {
        export_pgn(&game, output_file);
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
    
    if !args.quiet {
        println!("✓ {} moved from {} to {}", army.display_name(), coords[0], coords[1]);
    }
    
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
            
            if !args.quiet {
                println!("🤖 {} AI: {}{} -> {}{}", 
                    current.display_name(), from_file, from_rank, to_file, to_rank);
            }
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

fn run_batch(game: &mut Game, batch_file: &str, args: &Args) {
    use std::fs;
    
    let contents = match fs::read_to_string(batch_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading batch file: {}", e);
            process::exit(1);
        }
    };
    
    for (line_num, line) in contents.lines().enumerate() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        println!("{}> {}", line_num + 1, line);
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        let cmd = parts[0];
        
        match cmd {
            "show" | "board" => {
                for row in game.board.ascii_rows() {
                    println!("{}", row);
                }
            }
            "status" => show_status(game),
            "history" => show_history(game),
            "evaluate" | "eval" => evaluate_position(game),
            "move" => {
                if parts.len() < 2 {
                    eprintln!("Error: move requires argument");
                    continue;
                }
                let move_str = parts[1..].join(" ");
                let move_parts: Vec<&str> = move_str.split(':').collect();
                if move_parts.len() == 2 {
                    if let Some(army) = Army::from_str(move_parts[0].trim()) {
                        let coords = move_parts[1].trim().replace('x', "-");
                        let coord_parts: Vec<&str> = coords.split('-').collect();
                        if coord_parts.len() == 2 {
                            if let (Ok(from), Ok(to)) = (
                                parse_square_headless(coord_parts[0].trim()),
                                parse_square_headless(coord_parts[1].trim())
                            ) {
                                match game.apply_move(army, from, to, None) {
                                    Ok(msg) => println!("  ✓ {}", msg),
                                    Err(e) => eprintln!("  ❌ {}", e),
                                }
                            }
                        }
                    }
                }
            }
            "legal" => {
                if parts.len() < 2 {
                    eprintln!("Error: legal requires army argument");
                } else if let Some(army) = Army::from_str(parts[1]) {
                    show_legal_moves(game, army);
                }
            }
            _ => eprintln!("Unknown command: {}", cmd),
        }
    }
    
    // Save state if specified
    if let Some(save_file) = &args.state {
        if let Ok(json) = game.to_json() {
            fs::write(save_file, json).ok();
            println!("\nGame saved to {}", save_file);
        }
    }
}

fn run_interactive(game: &mut Game, ai_armies: &[Army], args: &Args) {
    use std::io::{self, Write};
    
    println!("Enochian Chess Interactive Mode");
    println!("Type 'help' for commands, 'quit' to exit\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0];
        
        match cmd {
            "quit" | "exit" | "q" => break,
            "help" | "h" => {
                println!("Commands:");
                println!("  show              - Display board");
                println!("  status            - Show game status");
                println!("  history           - Show move history");
                println!("  evaluate          - Evaluate position");
                println!("  analyze <square>  - Analyze a square");
                println!("  validate <move>   - Validate a move");
                println!("  move <move>       - Make a move (e.g., 'move blue: e2-e3')");
                println!("  undo [N]          - Undo last N moves (default 1)");
                println!("  legal <army>      - Show legal moves for army");
                println!("  quit              - Exit interactive mode");
            }
            "show" | "board" => {
                for row in game.board.ascii_rows() {
                    println!("{}", row);
                }
            }
            "status" => show_status(game),
            "history" => show_history(game),
            "evaluate" | "eval" => evaluate_position(game),
            "analyze" => {
                if parts.len() < 2 {
                    println!("Usage: analyze <square>");
                } else {
                    let square_str = parts[1];
                    if let Ok(square) = parse_square_headless(square_str) {
                        // Inline analyze logic
                        if let Some((piece_army, piece_kind)) = game.board.piece_at(square) {
                            println!("Square {}: {} {}", square_str, piece_army.display_name(), piece_kind.name());
                            let all_moves = game.legal_moves(piece_army).to_vec();
                            let moves: Vec<_> = all_moves.iter().filter(|m| m.from == square).collect();
                            if moves.is_empty() {
                                println!("No legal moves from this square");
                            } else {
                                println!("Legal moves:");
                                for mv in moves {
                                    let to_file = (b'a' + (mv.to % 8)) as char;
                                    let to_rank = (b'1' + (mv.to / 8)) as char;
                                    println!("  {}{}", to_file, to_rank);
                                }
                            }
                        } else {
                            println!("Empty square");
                        }
                    } else {
                        println!("Invalid square");
                    }
                }
            }
            "validate" => {
                if parts.len() < 2 {
                    println!("Usage: validate <move>");
                } else {
                    let move_str = parts[1..].join(" ");
                    // Parse and validate
                    let move_parts: Vec<&str> = move_str.split(':').collect();
                    if move_parts.len() == 2 {
                        if let Some(army) = Army::from_str(move_parts[0].trim()) {
                            let coords = move_parts[1].trim().replace('x', "-");
                            let coord_parts: Vec<&str> = coords.split('-').collect();
                            if coord_parts.len() == 2 {
                                if let (Ok(from), Ok(to)) = (
                                    parse_square_headless(coord_parts[0].trim()),
                                    parse_square_headless(coord_parts[1].trim())
                                ) {
                                    if game.is_legal_move(army, from, to) {
                                        println!("✓ Valid move");
                                    } else {
                                        println!("❌ Invalid move");
                                    }
                                } else {
                                    println!("Invalid square notation");
                                }
                            } else {
                                println!("Invalid move format");
                            }
                        } else {
                            println!("Unknown army");
                        }
                    } else {
                        println!("Format: army: from-to");
                    }
                }
            }
            "move" | "m" => {
                if parts.len() < 2 {
                    println!("Usage: move <army: from-to>");
                } else {
                    let move_str = parts[1..].join(" ");
                    let move_parts: Vec<&str> = move_str.split(':').collect();
                    if move_parts.len() == 2 {
                        if let Some(army) = Army::from_str(move_parts[0].trim()) {
                            let coords = move_parts[1].trim().replace('x', "-");
                            let coord_parts: Vec<&str> = coords.split('-').collect();
                            if coord_parts.len() == 2 {
                                if let (Ok(from), Ok(to)) = (
                                    parse_square_headless(coord_parts[0].trim()),
                                    parse_square_headless(coord_parts[1].trim())
                                ) {
                                    match game.apply_move(army, from, to, None) {
                                        Ok(msg) => println!("✓ {}", msg),
                                        Err(e) => println!("❌ {}", e),
                                    }
                                } else {
                                    println!("Invalid square notation");
                                }
                            } else {
                                println!("Invalid move format");
                            }
                        } else {
                            println!("Unknown army");
                        }
                    } else {
                        println!("Format: army: from-to");
                    }
                }
            }
            "legal" => {
                if parts.len() < 2 {
                    println!("Usage: legal <army>");
                } else if let Some(army) = Army::from_str(parts[1]) {
                    show_legal_moves(game, army);
                } else {
                    println!("Unknown army");
                }
            }
            "undo" | "u" => {
                let count = if parts.len() > 1 {
                    parts[1].parse().unwrap_or(1)
                } else {
                    1
                };
                match game.undo(count) {
                    Ok(undone) => println!("Undid {} move(s)", undone),
                    Err(e) => println!("Error: {}", e),
                }
            }
            _ => println!("Unknown command. Type 'help' for commands."),
        }
    }
    
    // Save state if specified
    if let Some(save_file) = &args.state {
        if let Ok(json) = game.to_json() {
            std::fs::write(save_file, json).ok();
            println!("Game saved to {}", save_file);
        }
    }
}

fn import_pgn(pgn_file: &str) -> Game {
    use std::fs;
    use crate::engine::arrays::default_array;
    
    let contents = match fs::read_to_string(pgn_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading PGN file: {}", e);
            process::exit(1);
        }
    };
    
    let mut game = Game::from_array_spec(default_array());
    let mut move_count = 0;
    
    for line in contents.lines() {
        let line = line.trim();
        
        // Skip headers and empty lines
        if line.is_empty() || line.starts_with('[') {
            continue;
        }
        
        // Parse moves (format: B:e2-e3 R:e7-e6)
        for token in line.split_whitespace() {
            // Skip move numbers (e.g., "1.")
            if token.ends_with('.') {
                continue;
            }
            
            // Parse move (format: B:e2-e3)
            let parts: Vec<&str> = token.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let army = match parts[0] {
                "B" => Army::Blue,
                "R" => Army::Red,
                "K" => Army::Black,
                "Y" => Army::Yellow,
                _ => continue,
            };
            
            let move_str = parts[1];
            let coords: Vec<&str> = move_str.split('-').collect();
            if coords.len() != 2 {
                continue;
            }
            
            if let (Ok(from), Ok(to)) = (
                parse_square_headless(coords[0]),
                parse_square_headless(coords[1])
            ) {
                if let Err(e) = game.apply_move(army, from, to, None) {
                    eprintln!("Warning: Failed to apply move {}: {}", token, e);
                } else {
                    move_count += 1;
                }
            }
        }
    }
    
    println!("Imported {} moves from {}", move_count, pgn_file);
    game
}

fn export_pgn(game: &Game, output_file: &str) {
    use std::fs;
    
    let mut pgn = String::new();
    
    // Header
    pgn.push_str("[Event \"Enochian Chess Game\"]\n");
    pgn.push_str(&format!("[Date \"{}\"]\n", chrono::Local::now().format("%Y.%m.%d")));
    pgn.push_str("[Variant \"Enochian\"]\n");
    pgn.push_str("[Players \"4\"]\n");
    
    if let Some(team) = game.winning_team() {
        pgn.push_str(&format!("[Result \"{} team wins\"]\n", team.name()));
    } else {
        pgn.push_str("[Result \"*\"]\n");
    }
    
    pgn.push_str("\n");
    
    // Moves
    for (i, (army, from, to, promotion)) in game.move_history.iter().enumerate() {
        if i % 4 == 0 {
            pgn.push_str(&format!("{}. ", i / 4 + 1));
        }
        
        let from_file = (b'a' + (from % 8)) as char;
        let from_rank = (b'1' + (from / 8)) as char;
        let to_file = (b'a' + (to % 8)) as char;
        let to_rank = (b'1' + (to / 8)) as char;
        
        let promo_str = if let Some(kind) = promotion {
            format!("={}", match kind {
                crate::engine::types::PieceKind::Queen => "Q",
                crate::engine::types::PieceKind::Rook => "R",
                crate::engine::types::PieceKind::Bishop => "B",
                crate::engine::types::PieceKind::Knight => "N",
                _ => "",
            })
        } else {
            String::new()
        };
        
        pgn.push_str(&format!("{}:{}{}-{}{}{} ", 
            match army {
                crate::engine::types::Army::Blue => "B",
                crate::engine::types::Army::Red => "R",
                crate::engine::types::Army::Black => "K",
                crate::engine::types::Army::Yellow => "Y",
            },
            from_file, from_rank, to_file, to_rank, promo_str
        ));
        
        if (i + 1) % 4 == 0 {
            pgn.push('\n');
        }
    }
    
    if !game.move_history.is_empty() && game.move_history.len() % 4 != 0 {
        pgn.push('\n');
    }
    
    if let Err(e) = fs::write(output_file, pgn) {
        eprintln!("Error writing PGN: {}", e);
        process::exit(1);
    }
    
    println!("Exported to {}", output_file);
}

fn show_stats(game: &Game) {
    use crate::engine::types::{Army, PieceKind};
    
    println!("Game Statistics\n");
    
    // Move count
    println!("Moves played: {}", game.move_history.len());
    
    // Captures (inferred from missing pieces)
    println!("\nCaptures:");
    let initial_counts: [(PieceKind, usize); 6] = [
        (PieceKind::King, 1),
        (PieceKind::Queen, 1),
        (PieceKind::Rook, 2),
        (PieceKind::Bishop, 2),
        (PieceKind::Knight, 2),
        (PieceKind::Pawn, 8),
    ];
    
    for &army in Army::ALL.iter() {
        let counts = game.board.piece_counts(army);
        let mut captured = Vec::new();
        let mut total_captured = 0;
        
        for &(kind, initial) in &initial_counts {
            let current = counts[kind.index()] as usize;
            let lost = initial.saturating_sub(current);
            if lost > 0 {
                captured.push(format!("{}×{}", lost, kind.name()));
                total_captured += lost;
            }
        }
        
        if total_captured > 0 {
            println!("  {} lost: {} ({})", army.display_name(), total_captured, captured.join(", "));
        } else {
            println!("  {} lost: 0", army.display_name());
        }
    }
    
    // Army status
    println!("\nArmy Status:");
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
    
    // Winner
    if let Some(team) = game.winning_team() {
        println!("\n🏆 Winner: {} team", team.name());
    }
}

fn evaluate_position(game: &mut Game) {
    use crate::engine::types::{Army, PieceKind};
    
    println!("Position Evaluation\n");
    
    // Material count
    println!("Material:");
    let piece_values = [
        (PieceKind::King, 0),
        (PieceKind::Queen, 9),
        (PieceKind::Rook, 5),
        (PieceKind::Bishop, 3),
        (PieceKind::Knight, 3),
        (PieceKind::Pawn, 1),
    ];
    
    for &army in Army::ALL.iter() {
        let mut total = 0;
        let mut pieces = Vec::new();
        let counts = game.board.piece_counts(army);
        
        for &(kind, value) in &piece_values {
            let count = counts[kind.index()] as usize;
            if count > 0 {
                total += count * value;
                pieces.push(format!("{}×{}", count, kind.name()));
            }
        }
        
        println!("  {}: {} ({})", army.display_name(), total, pieces.join(", "));
    }
    
    // Mobility (legal moves)
    println!("\nMobility:");
    for &army in Army::ALL.iter() {
        if game.army_is_frozen(army) {
            println!("  {}: Frozen", army.display_name());
        } else {
            let moves = game.legal_moves(army).len();
            println!("  {}: {} legal moves", army.display_name(), moves);
        }
    }
    
    // Status
    println!("\nStatus:");
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
    
    // Winner
    if let Some(team) = game.winning_team() {
        println!("\n🏆 Winner: {} team", team.name());
    }
}

fn show_history(game: &Game) {
    if game.move_history.is_empty() {
        println!("No moves played yet");
        return;
    }
    
    println!("Move history ({} moves):\n", game.move_history.len());
    for (i, (army, from, to, promotion)) in game.move_history.iter().enumerate() {
        let from_file = (b'a' + (from % 8)) as char;
        let from_rank = (b'1' + (from / 8)) as char;
        let to_file = (b'a' + (to % 8)) as char;
        let to_rank = (b'1' + (to / 8)) as char;
        
        let promo_str = if let Some(kind) = promotion {
            format!("={}", match kind {
                crate::engine::types::PieceKind::Queen => "Q",
                crate::engine::types::PieceKind::Rook => "R",
                crate::engine::types::PieceKind::Bishop => "B",
                crate::engine::types::PieceKind::Knight => "N",
                _ => "",
            })
        } else {
            String::new()
        };
        
        println!("{}. {}: {}{}-{}{}{}", 
            i + 1, 
            army.display_name(), 
            from_file, from_rank, 
            to_file, to_rank,
            promo_str
        );
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

fn list_arrays() {
    use crate::engine::arrays::available_arrays;
    
    println!("Available starting arrays:\n");
    for (i, array) in available_arrays().iter().enumerate() {
        println!("{}. {}", i + 1, array.name);
        println!("   {}", array.description);
        println!();
    }
}

fn convert_format(game: &Game, format: &str) {
    match format.to_lowercase().as_str() {
        "json" => {
            if let Ok(json) = game.to_json() {
                println!("{}", json);
            } else {
                eprintln!("❌ Failed to convert to JSON");
                process::exit(1);
            }
        }
        "ascii" => {
            for row in game.board.ascii_rows() {
                println!("{}", row);
            }
        }
        "compact" => {
            // Compact notation: piece positions per army
            for &army in crate::engine::types::Army::ALL.iter() {
                let mut pieces = Vec::new();
                for square in 0..64 {
                    if let Some((piece_army, kind)) = game.board.piece_at(square) {
                        if piece_army == army {
                            let file = (b'a' + (square % 8)) as char;
                            let rank = (b'1' + (square / 8)) as char;
                            let piece_char = match kind {
                                crate::engine::types::PieceKind::King => 'K',
                                crate::engine::types::PieceKind::Queen => 'Q',
                                crate::engine::types::PieceKind::Bishop => 'B',
                                crate::engine::types::PieceKind::Knight => 'N',
                                crate::engine::types::PieceKind::Rook => 'R',
                                crate::engine::types::PieceKind::Pawn => 'P',
                            };
                            pieces.push(format!("{}{}{}", piece_char, file, rank));
                        }
                    }
                }
                if !pieces.is_empty() {
                    println!("{}:{}", army.display_name().to_lowercase(), pieces.join(","));
                }
            }
        }
        _ => {
            eprintln!("❌ Unknown format: {}", format);
            eprintln!("Available formats: json, ascii, compact");
            process::exit(1);
        }
    }
}

fn run_perft(game: &mut Game, depth: u8) {
    use std::time::Instant;
    
    println!("Running perft({})", depth);
    let start = Instant::now();
    let nodes = perft(game, depth);
    let elapsed = start.elapsed();
    
    println!("Nodes: {}", nodes);
    println!("Time: {:.3}s", elapsed.as_secs_f64());
    println!("NPS: {:.0}", nodes as f64 / elapsed.as_secs_f64());
}

fn perft(game: &mut Game, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let army = game.current_army();
    let moves = game.legal_moves(army).to_vec();
    
    if depth == 1 {
        return moves.len() as u64;
    }
    
    let mut nodes = 0u64;
    for mv in moves {
        let saved = game.clone();
        if game.apply_move(army, mv.from, mv.to, None).is_ok() {
            nodes += perft(game, depth - 1);
        }
        *game = saved;
    }
    
    nodes
}

fn generate_position(gen_str: &str, args: &Args) {
    use crate::engine::board::Board;
    use crate::engine::game::Game;
    use crate::engine::types::{Army, PieceKind, Piece};
    use std::fs;
    
    let mut placements = Vec::new();
    
    // Parse format: "Kb1,Qc2:blue Ke8:red"
    for army_spec in gen_str.split_whitespace() {
        let parts: Vec<&str> = army_spec.split(':').collect();
        if parts.len() != 2 {
            eprintln!("❌ Invalid format. Use: 'Kb1,Qc2:blue Ke8:red'");
            process::exit(1);
        }
        
        let army = match Army::from_str(parts[1].trim()) {
            Some(a) => a,
            None => {
                eprintln!("❌ Unknown army: {}", parts[1]);
                process::exit(1);
            }
        };
        
        for piece_spec in parts[0].split(',') {
            let piece_spec = piece_spec.trim();
            if piece_spec.len() < 2 {
                eprintln!("❌ Invalid piece spec: {}", piece_spec);
                process::exit(1);
            }
            
            let kind = match piece_spec.chars().next().unwrap() {
                'K' => PieceKind::King,
                'Q' => PieceKind::Queen,
                'B' => PieceKind::Bishop,
                'N' => PieceKind::Knight,
                'R' => PieceKind::Rook,
                'P' => PieceKind::Pawn,
                c => {
                    eprintln!("❌ Unknown piece: {}", c);
                    process::exit(1);
                }
            };
            
            let square_str = &piece_spec[1..];
            let square = match parse_square_headless(square_str) {
                Ok(sq) => sq,
                Err(e) => {
                    eprintln!("❌ Invalid square {}: {}", square_str, e);
                    process::exit(1);
                }
            };
            
            placements.push((army, Piece { army, kind, pawn_type: None }, 1u64 << square));
        }
    }
    
    if placements.is_empty() {
        eprintln!("❌ No pieces specified");
        process::exit(1);
    }
    
    let board = Board::new(&placements);
    let game = Game::new(board);
    
    println!("✓ Generated position with {} pieces", placements.len());
    
    if args.show {
        println!();
        for row in game.board.ascii_rows() {
            println!("{}", row);
        }
    }
    
    if let Some(save_file) = &args.state {
        if let Ok(json) = game.to_json() {
            fs::write(save_file, json).ok();
            println!("✓ Saved to {}", save_file);
        }
    }
}

fn query_rules(query: &str) {
    let q = query.to_lowercase();
    
    if q.contains("queen") && q.contains("capture") && q.contains("queen") {
        println!("Can queens capture queens?");
        println!("❌ No - Queens cannot capture other queens");
    } else if q.contains("bishop") && q.contains("capture") && q.contains("bishop") {
        println!("Can bishops capture bishops?");
        println!("❌ No - Bishops cannot capture other bishops");
    } else if q.contains("queen") && q.contains("bishop") {
        println!("Can queens and bishops capture each other?");
        println!("✓ Yes - Queens can capture bishops, and bishops can capture queens");
    } else if q.contains("check") {
        println!("Check rules:");
        println!("• No checkmate - kings are captured like other pieces");
        println!("• If in check with legal king moves, you MUST move the king");
        println!("• If in check with no legal king moves, you may move any piece");
    } else if q.contains("promotion") || q.contains("promote") {
        println!("Promotion rules:");
        println!("• Blue pawns promote on rank 8 (north edge)");
        println!("• Red pawns promote on rank 1 (south edge)");
        println!("• Black pawns promote on file h (east edge)");
        println!("• Yellow pawns promote on file a (west edge)");
        println!("• Privileged pawn: With only K+Q+P, K+B+P, or K+P remaining,");
        println!("  the pawn can promote to any piece type");
    } else if q.contains("frozen") || q.contains("freeze") {
        println!("Frozen army rules:");
        println!("• When a king is captured, that army becomes frozen");
        println!("• Frozen pieces cannot move or attack");
        println!("• Frozen pieces act as blocking terrain");
        println!("• An army can be revived by controlling its throne square");
    } else if q.contains("throne") {
        println!("Throne square rules:");
        println!("• Each army has a throne (king's starting position)");
        println!("• Moving your king onto an ally's throne = gain control");
        println!("• Controlling a throne revives that frozen army");
    } else if q.contains("team") || q.contains("victory") || q.contains("win") {
        println!("Victory conditions:");
        println!("• Teams: Air (Blue + Black) vs Earth (Red + Yellow)");
        println!("• Win by capturing both enemy kings");
        println!("• Frozen armies can be revived via throne control");
    } else if q.contains("queen") && q.contains("move") {
        println!("Queen movement:");
        println!("• Leaps exactly 2 squares (orthogonal or diagonal)");
        println!("• Ignores intervening pieces (like a knight)");
        println!("• Cannot move 1 square or 3+ squares");
    } else if q.contains("pawn") && (q.contains("move") || q.contains("capture")) {
        println!("Pawn movement:");
        println!("• Moves 1 square forward");
        println!("• Captures 1 square diagonally");
        println!("• No double-step initial move");
        println!("• No en passant");
    } else if q.contains("stalemate") {
        println!("Stalemate rules:");
        println!("• If an army has no legal moves, that turn is skipped");
        println!("• Play continues with the next army");
    } else {
        println!("Unknown query. Try:");
        println!("  --query 'queen capture queen'");
        println!("  --query 'bishop capture bishop'");
        println!("  --query 'check'");
        println!("  --query 'promotion'");
        println!("  --query 'frozen'");
        println!("  --query 'throne'");
        println!("  --query 'victory'");
        println!("  --query 'queen move'");
        println!("  --query 'pawn move'");
        println!("  --query 'stalemate'");
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
