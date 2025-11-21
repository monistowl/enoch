use crate::engine::arrays::available_arrays;
use crate::engine::types::{Army, PieceKind, PlayerId, Team};
use crate::ui::app::{App, CurrentScreen};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

const BG_COLOR: Color = Color::Black;

pub fn render(frame: &mut Frame, app: &mut App) {
    // Capture frame for screenshots
    let size = frame.area();
    let mut capture = format!("Terminal: {}x{}\n", size.width, size.height);
    capture.push_str("═".repeat(size.width as usize).as_str());
    capture.push('\n');
    
    match app.current_screen {
        CurrentScreen::Help => {
            render_help(frame, app);
            capture.push_str("Help Screen\n");
        }
        _ => {
            render_main(frame, app);
            // Capture board state
            capture.push_str(&format!("Turn: {}\n", app.game.current_army().display_name()));
            capture.push_str(&format!("Array: {}\n", app.selected_array));
            if app.game.config.divination_mode {
                capture.push_str("Mode: Divination 🎲\n");
            }
            capture.push_str("\nBoard:\n");
            for row in app.board_rows() {
                capture.push_str(&row);
                capture.push('\n');
            }
        }
    }
    
    app.last_frame = Some(capture);
}

fn render_help(frame: &mut Frame, app: &App) {
    let size = frame.area();
    let help_lines = App::get_help_text();
    
    let visible_lines: Vec<Line> = help_lines
        .iter()
        .skip(app.help_scroll)
        .take(size.height.saturating_sub(4) as usize)
        .map(|s| Line::from(Span::styled(s.as_str(), Style::default().fg(Color::White).bg(BG_COLOR))))
        .collect();
    
    let help_text = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - Enochian Chess Rules & Commands")
                .style(Style::default().fg(Color::Cyan).bg(BG_COLOR)),
        )
        .style(Style::default().bg(BG_COLOR))
        .wrap(Wrap { trim: false });
    
    frame.render_widget(help_text, size);
}

fn render_main(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    
    // Calculate optimal board size (8x8 board + borders + labels)
    // Each square needs: width chars × height lines
    // We need to fit: 2 (rank labels) + 8*square_width + 1 (file labels)
    let available_height = size.height.saturating_sub(6); // Reserve for header + input
    let available_width = size.width.saturating_sub(4); // Reserve for borders
    
    // Calculate max square size that fits
    let max_square_height = available_height / 9; // 8 ranks + 1 label row
    let max_square_width = available_width / 10; // 2 label + 8 files
    let square_size = max_square_height.min(max_square_width / 2).max(1).min(3);
    
    // Calculate actual board dimensions
    let board_width = 2 + (square_size * 2 + 1) * 8 + 2; // labels + squares + borders
    let board_height = 1 + square_size * 8 + 1 + 2; // turn + squares + labels + borders
    
    // Determine if we can fit info panel beside board
    let info_width = 35;
    let can_fit_side_panel = size.width >= board_width + info_width + 2;
    
    let (header_height, input_height) = if size.height < 30 { (1, 1) } else { (3, 3) };
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Length(1), // Army selector
            Constraint::Min(10),
            Constraint::Length(input_height),
        ])
        .split(size);

    let header_text = if size.width < 100 {
        "Enochian Chess | ? for help | 1-4: Select Army | Tab: Cycle"
    } else {
        "Enochian Chess | 1-4 or Tab: Select Army | Enter square (e2) to select/move | ? for help"
    };
    
    let header = Paragraph::new(Span::styled(
        header_text,
        Style::default()
            .fg(Color::Yellow)
            .bg(BG_COLOR)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default()
        .borders(Borders::ALL)
        .title("Enochian Chess")
        .style(Style::default().bg(BG_COLOR)));
    frame.render_widget(header, layout[0]);

    // Army selector bar
    let army_selector = build_army_selector(app);
    frame.render_widget(army_selector, layout[1]);

    let mid_chunks = if can_fit_side_panel {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(board_width),
                Constraint::Min(info_width),
            ])
            .split(layout[2])
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(board_height), Constraint::Min(5)])
            .split(layout[2])
    };

    let board = Paragraph::new(text_from_board_scaled(app, Some(square_size)))
        .block(Block::default()
            .title("Enochian Board")
            .borders(Borders::ALL)
            .style(Style::default().bg(BG_COLOR)))
        .style(Style::default().bg(BG_COLOR))
        .wrap(Wrap { trim: true });
    frame.render_widget(board, mid_chunks[0]);

    if can_fit_side_panel {
        let info_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(7), Constraint::Length(6)])
            .split(mid_chunks[1]);

        let status = Paragraph::new(build_status_lines(app))
            .block(Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .style(Style::default().bg(BG_COLOR)))
            .style(Style::default().bg(BG_COLOR))
            .wrap(Wrap { trim: true });
        frame.render_widget(status, info_chunks[0]);

        let arrays = Paragraph::new(array_list_text(app))
            .block(Block::default()
                .title("Arrays")
                .borders(Borders::ALL)
                .style(Style::default().bg(BG_COLOR)))
            .style(Style::default().bg(BG_COLOR))
            .wrap(Wrap { trim: true });
        frame.render_widget(arrays, info_chunks[1]);
    } else {
        let status = Paragraph::new(build_status_lines(app))
            .block(Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .style(Style::default().bg(BG_COLOR)))
            .style(Style::default().bg(BG_COLOR))
            .wrap(Wrap { trim: true });
        frame.render_widget(status, mid_chunks[1]);
    }

    let input_line = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Green).bg(BG_COLOR)),
        Span::styled(app.input.clone(), Style::default().fg(Color::White).bg(BG_COLOR)),
    ])))
    .block(Block::default()
        .borders(Borders::ALL)
        .title("Command")
        .style(Style::default().bg(BG_COLOR)))
    .style(Style::default().bg(BG_COLOR));
    frame.render_widget(input_line, layout[3]);
}

pub fn render_size_error(frame: &mut Frame, min_width: u16, min_height: u16, size: Rect) {
    let warning = Paragraph::new(Text::from(vec![Line::from(vec![Span::styled(
        format!(
            "Terminal too small: {}x{} (minimum {}x{})",
            size.width, size.height, min_width, min_height
        ),
        Style::default().fg(Color::Red).bg(BG_COLOR).add_modifier(Modifier::BOLD),
    )])]))
    .block(Block::default()
        .borders(Borders::ALL)
        .title("Size Error")
        .style(Style::default().bg(BG_COLOR)))
    .style(Style::default().bg(BG_COLOR));
    frame.render_widget(warning, size);
}

fn build_status_lines(app: &App) -> Text {
    let mut lines = Vec::new();
    let current_army = app.game.state.current_army(&app.game.config);
    
    let in_check = app.game.king_in_check(current_army);
    let check_indicator = if in_check { " ⚠ CHECK" } else { "" };
    
    lines.push(Line::from(vec![Span::styled(
        format!("Turn: {}{}", current_army.display_name(), check_indicator),
        Style::default()
            .fg(if in_check { Color::Red } else { Color::LightBlue })
            .bg(BG_COLOR)
            .add_modifier(if in_check { Modifier::BOLD } else { Modifier::empty() }),
    )]));

    lines.push(Line::from(Span::styled(
        format!("Array: {}", app.selected_array),
        Style::default().fg(Color::White).bg(BG_COLOR),
    )));

    let frozen: Vec<&str> = Army::ALL
        .iter()
        .filter(|&&army| app.game.army_is_frozen(army))
        .map(|army| army.display_name())
        .collect();
    if !frozen.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("❄ Frozen: {}", frozen.join(", ")),
            Style::default().fg(Color::Cyan).bg(BG_COLOR),
        )));
    }

    let stalemated: Vec<&str> = Army::ALL
        .iter()
        .filter(|&&army| app.game.state.is_stalemated(army))
        .map(|army| army.display_name())
        .collect();
    if !stalemated.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("⊗ Stalemated: {}", stalemated.join(", ")),
            Style::default().fg(Color::Gray).bg(BG_COLOR),
        )));
    }

    if let Some(team) = app.game.winning_team() {
        lines.push(Line::from(Span::styled(
            format!("🏆 {} TEAM WINS!", team.name().to_uppercase()),
            Style::default()
                .fg(Color::Green)
                .bg(BG_COLOR)
                .add_modifier(Modifier::BOLD),
        )));
    } else if app.game.draw_condition() {
        lines.push(Line::from(Span::styled(
            "⚖ DRAW",
            Style::default()
                .fg(Color::Yellow)
                .bg(BG_COLOR)
                .add_modifier(Modifier::BOLD),
        )));
    }

    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(
            format!("✓ {}", msg),
            Style::default().fg(Color::Green).bg(BG_COLOR),
        )));
    }

    if let Some(ref err) = app.error_message {
        lines.push(Line::from(Span::styled(
            format!("✗ {}", err),
            Style::default().fg(Color::Red).bg(BG_COLOR),
        )));
    }

    let history = app.history_lines();
    if !history.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("History: {}", history.join(", ")),
            Style::default().fg(Color::DarkGray).bg(BG_COLOR),
        )));
    }

    lines.extend(army_status_lines(app));

    lines.push(Line::from(Span::styled(
        command_help(),
        Style::default().fg(Color::Rgb(120, 120, 200)).bg(BG_COLOR),
    )));

    Text::from(lines)
}

fn array_list_text(app: &App) -> Text {
    let mut lines = Vec::new();
    for spec in available_arrays() {
        let name = spec.name;
        let style = if name == app.selected_array {
            Style::default()
                .fg(Color::LightGreen)
                .bg(BG_COLOR)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White).bg(BG_COLOR)
        };
        let order = spec
            .turn_order
            .iter()
            .map(|army| army.display_name())
            .collect::<Vec<_>>()
            .join(" → ");
        lines.push(Line::from(Span::styled(
            format!("{} [{}]", name, order),
            style,
        )));
    }
    Text::from(lines)
}

fn army_status_lines(app: &App) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        "─── Armies ───",
        Style::default().fg(Color::DarkGray).bg(BG_COLOR),
    )));
    
    for &army in Army::ALL.iter() {
        let mut status_parts = Vec::new();
        
        if app.game.army_is_frozen(army) {
            status_parts.push("❄ Frozen");
        } else if app.game.state.is_stalemated(army) {
            status_parts.push("⊗ Stalemate");
        } else if app.game.king_in_check(army) {
            status_parts.push("⚠ Check");
        } else {
            status_parts.push("✓ Active");
        }
        
        let controller = controller_label(app.game.board.controller_for(army));
        status_parts.push(controller);
        
        let style = match army {
            Army::Blue => Style::default().fg(Color::Blue).bg(BG_COLOR),
            Army::Black => Style::default().fg(Color::White).bg(BG_COLOR),
            Army::Red => Style::default().fg(Color::Red).bg(BG_COLOR),
            Army::Yellow => Style::default().fg(Color::Yellow).bg(BG_COLOR),
        };
        
        let current = app.game.current_army();
        let style = if army == current {
            style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            style
        };
        
        lines.push(Line::from(Span::styled(
            format!(
                "{:8} ({:4}) {}",
                army.display_name(),
                army.team().name(),
                status_parts.join(" • ")
            ),
            style,
        )));
    }
    lines
}

fn text_from_board_scaled(app: &App, square_size: Option<u16>) -> Text {
    let mut lines = Vec::new();
    let current_army = app.game.current_army();
    
    let square_height = square_size.unwrap_or(1);
    let square_width = (square_height * 2 + 1) as usize; // Convert to usize for formatting
    
    // Add turn indicator at top
    lines.push(Line::from(Span::styled(
        format!("▶ {} to move", current_army.display_name()),
        Style::default()
            .fg(army_color(current_army))
            .bg(BG_COLOR)
            .add_modifier(Modifier::BOLD),
    )));
    
    // Render board with scaled squares
    for rank in (0..8).rev() {
        for row in 0..square_height {
            let mut spans = Vec::new();
            
            // Rank label on first row of square
            if row == square_height / 2 {
                spans.push(Span::styled(
                    format!("{} ", rank + 1),
                    Style::default().fg(Color::White).bg(BG_COLOR),
                ));
            } else {
                spans.push(Span::styled("  ", Style::default().bg(BG_COLOR)));
            }
            
            for file in 0..8 {
                let square = rank * 8 + file;
                let (chr, style) = board_square_info(app, square, current_army);
                
                // Center piece character in the middle row
                let content = if row == square_height / 2 {
                    format!("{:^width$}", chr, width = square_width)
                } else {
                    " ".repeat(square_width)
                };
                
                spans.push(Span::styled(content, style));
            }
            lines.push(Line::from(spans));
        }
    }
    
    // File labels
    let mut file_spans = vec![Span::styled("  ", Style::default().bg(BG_COLOR))];
    for f in b'a'..=b'h' {
        let label = format!("{:^width$}", (f as char).to_ascii_uppercase(), width = square_width);
        file_spans.push(Span::styled(label, Style::default().fg(Color::Gray).bg(BG_COLOR)));
    }
    lines.push(Line::from(file_spans));
    
    Text::from(lines)
}

fn army_color(army: Army) -> Color {
    match army {
        Army::Blue => Color::Rgb(100, 150, 255),    // Brighter blue
        Army::Black => Color::Rgb(220, 220, 220),   // Light gray (not pure white)
        Army::Red => Color::Rgb(255, 100, 100),     // Brighter red
        Army::Yellow => Color::Rgb(255, 220, 100),  // Brighter yellow
    }
}

fn board_square_info(app: &App, square: u8, current_army: Army) -> (char, Style) {
    let base_color = if (square / 8 + square % 8) % 2 == 0 {
        Color::Rgb(80, 80, 80)
    } else {
        Color::Rgb(40, 40, 40)
    };
    
    let is_selected = app.selected_square == Some(square);
    let is_legal_move = if let Some(from_sq) = app.selected_square {
        if let Some(army) = app.selected_army {
            app.game.is_legal_move(army, from_sq, square)
        } else {
            false
        }
    } else {
        false
    };
    
    let throne_bg = Color::Rgb(120, 70, 30);
    let selected_bg = Color::Rgb(100, 100, 50);
    let legal_move_bg = Color::Rgb(50, 80, 50);
    
    let throne = app.game.board.throne_owner(square);
    let bg = if is_selected {
        selected_bg
    } else if is_legal_move {
        legal_move_bg
    } else if throne.is_some() {
        throne_bg
    } else {
        base_color
    };
    
    if let Some((army, kind)) = app.game.board.piece_at(square) {
        let fg = army_color(army);
        let mut style = Style::default().fg(fg).bg(bg);
        if army == current_army || is_selected {
            style = style.add_modifier(Modifier::BOLD);
        }
        (
            piece_character(army, kind),
            style,
        )
    } else if throne.is_some() {
        ('◆', Style::default().fg(Color::Rgb(220, 160, 80)).bg(bg))
    } else {
        ('.', Style::default().fg(Color::Rgb(120, 120, 120)).bg(bg))
    }
}

fn piece_character(army: Army, kind: PieceKind) -> char {
    let letter = match kind {
        PieceKind::King => 'K',
        PieceKind::Queen => 'Q',
        PieceKind::Rook => 'R',
        PieceKind::Bishop => 'B',
        PieceKind::Knight => 'N',
        PieceKind::Pawn => 'P',
    };
    if matches!(army, Army::Black | Army::Yellow) {
        letter.to_ascii_lowercase()
    } else {
        letter
    }
}

fn controller_label(id: PlayerId) -> &'static str {
    match id.0 {
        0 => "P1",
        1 => "P2",
        _ => "P?",
    }
}

fn command_help() -> String {
    "Commands: blue: e2-e4 | /arrays | /status | /array <name|next|prev> | /exchange <army> | /save <file> | /load <file> | [ ] to cycle".to_string()
}

fn build_army_selector(app: &App) -> Paragraph {
    let armies = [Army::Blue, Army::Red, Army::Black, Army::Yellow];
    let mut spans = vec![Span::styled("Army: ", Style::default().fg(Color::White).bg(BG_COLOR))];
    
    for (i, &army) in armies.iter().enumerate() {
        let is_selected = app.selected_army == Some(army);
        let is_current = app.game.current_army() == army;
        
        let mut style = Style::default()
            .fg(army_color(army))
            .bg(if is_selected { Color::Rgb(60, 60, 60) } else { BG_COLOR });
        
        if is_selected {
            style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
        } else if is_current {
            style = style.add_modifier(Modifier::BOLD);
        }
        
        let label = format!("[{}] {} ", i + 1, army.display_name());
        spans.push(Span::styled(label, style));
    }
    
    if let Some(sq) = app.selected_square {
        let file = (b'a' + (sq % 8)) as char;
        let rank = (b'1' + (sq / 8)) as char;
        spans.push(Span::styled(
            format!(" | Selected: {}{}", file, rank),
            Style::default().fg(Color::Yellow).bg(BG_COLOR).add_modifier(Modifier::BOLD),
        ));
    }
    
    Paragraph::new(Line::from(spans))
        .style(Style::default().bg(BG_COLOR))
}
