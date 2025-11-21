use crate::engine::arrays::available_arrays;
use crate::engine::types::{Army, PieceKind, PlayerId, Team};
use crate::ui::app::{App, CurrentScreen};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Help => render_help(frame, app),
        _ => render_main(frame, app),
    }
}

fn render_help(frame: &mut Frame, app: &App) {
    let size = frame.area();
    let help_lines = App::get_help_text();
    
    let visible_lines: Vec<Line> = help_lines
        .iter()
        .skip(app.help_scroll)
        .take(size.height.saturating_sub(4) as usize)
        .map(|s| Line::from(s.as_str()))
        .collect();
    
    let help_text = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - Enochian Chess Rules & Commands")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });
    
    frame.render_widget(help_text, size);
}

fn render_main(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    
    // Responsive layout based on terminal size
    let (header_height, input_height) = if size.height < 30 {
        (1, 1) // Minimal for small terminals
    } else {
        (3, 3) // Full size for larger terminals
    };
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(header_height),
                Constraint::Min(10),
                Constraint::Length(input_height),
            ]
            .as_ref(),
        )
        .split(size);

    // Compact header for small terminals
    let header_text = if size.width < 100 {
        "Enochian Chess | ? for help"
    } else {
        "Enochian Chess | Move: army: e2-e4 | Commands: /arrays /status /array /exchange /save /load | ? for help"
    };
    
    let header = Paragraph::new(Span::styled(
        header_text,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("Enochian Chess"));
    frame.render_widget(header, layout[0]);

    // Responsive board/info split
    let board_pct = if size.width < 100 { 100 } else { 65 };
    let info_pct = 100 - board_pct;
    
    let mid_chunks = if size.width < 100 {
        // Stack vertically for narrow terminals
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(layout[1])
    } else {
        // Side by side for wide terminals
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(board_pct), Constraint::Percentage(info_pct)].as_ref())
            .split(layout[1])
    };

    let board_height = mid_chunks[0].height;
    let board = Paragraph::new(text_from_board_scaled(app, Some(board_height)))
        .block(
            Block::default()
                .title("Enochian Board")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(board, mid_chunks[0]);

    // Only show info panel if there's space
    if size.width >= 100 {
        let info_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(7), Constraint::Length(6)].as_ref())
            .split(mid_chunks[1]);

        let status = Paragraph::new(build_status_lines(app))
            .block(Block::default().title("Status").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(status, info_chunks[0]);

        let array_text = array_list_text(app);
        let arrays = Paragraph::new(array_text)
            .block(Block::default().title("Arrays").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(arrays, info_chunks[1]);
    } else {
        // Compact status in bottom panel for narrow terminals
        let status = Paragraph::new(build_status_lines(app))
            .block(Block::default().title("Status").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(status, mid_chunks[1]);
    }

    let input_line = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Green)),
        Span::raw(app.input.clone()),
    ])))
    .block(Block::default().borders(Borders::ALL).title("Command"));
    frame.render_widget(input_line, layout[2]);
}

pub fn render_size_error(frame: &mut Frame, min_width: u16, min_height: u16, size: Rect) {
    let warning = Paragraph::new(Text::from(vec![Line::from(vec![Span::styled(
        format!(
            "Terminal too small: {}x{} (minimum {}x{})",
            size.width, size.height, min_width, min_height
        ),
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )])]))
    .block(Block::default().borders(Borders::ALL).title("Size Error"));
    frame.render_widget(warning, size);
}

fn build_status_lines(app: &App) -> Text {
    let mut lines = Vec::new();
    let current_army = app.game.state.current_army(&app.game.config);
    
    // Check if current army is in check
    let in_check = app.game.king_in_check(current_army);
    let check_indicator = if in_check { " ⚠ CHECK" } else { "" };
    
    lines.push(Line::from(vec![Span::styled(
        format!("Turn: {}{}", current_army.display_name(), check_indicator),
        Style::default()
            .fg(if in_check { Color::Red } else { Color::LightBlue })
            .add_modifier(if in_check { Modifier::BOLD } else { Modifier::empty() }),
    )]));

    lines.push(Line::from(Span::raw(format!(
        "Array: {}",
        app.selected_array
    ))));

    let frozen: Vec<&str> = Army::ALL
        .iter()
        .filter(|&&army| app.game.army_is_frozen(army))
        .map(|army| army.display_name())
        .collect();
    if !frozen.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("❄ Frozen: {}", frozen.join(", ")),
            Style::default().fg(Color::Cyan),
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
            Style::default().fg(Color::Gray),
        )));
    }

    // Check for game outcome
    if let Some(team) = app.game.winning_team() {
        lines.push(Line::from(Span::styled(
            format!("🏆 {} TEAM WINS!", team.name().to_uppercase()),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
    } else if app.game.draw_condition() {
        lines.push(Line::from(Span::styled(
            "⚖ DRAW",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }

    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(
            format!("✓ {}", msg),
            Style::default().fg(Color::Green),
        )));
    }

    if let Some(ref err) = app.error_message {
        lines.push(Line::from(Span::styled(
            format!("✗ {}", err),
            Style::default().fg(Color::Red),
        )));
    }

    let history = app.history_lines();
    if !history.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("History: {}", history.join(", ")),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.extend(army_status_lines(app));

    lines.push(Line::from(Span::styled(
        command_help(),
        Style::default().fg(Color::Rgb(120, 120, 200)),
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
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
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
        Style::default().fg(Color::DarkGray),
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
            Army::Blue => Style::default().fg(Color::Blue),
            Army::Black => Style::default().fg(Color::White),
            Army::Red => Style::default().fg(Color::Red),
            Army::Yellow => Style::default().fg(Color::Yellow),
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

fn text_from_board_scaled(app: &App, available_height: Option<u16>) -> Text {
    let mut lines = Vec::new();
    let current_army = app.game.current_army();
    
    // Determine square size based on available space
    // Minimum: 1x1 (3 chars wide: " X ")
    // Medium: 2x2 (5 chars wide: "  X  ")
    // Large: 3x3 (7 chars wide: "   X   ")
    let square_height = if let Some(h) = available_height {
        if h >= 35 { 3 } else if h >= 25 { 2 } else { 1 }
    } else {
        1
    };
    
    let square_width = square_height * 2 + 1; // Maintain aspect ratio
    
    // Add turn indicator at top
    lines.push(Line::from(Span::styled(
        format!("▶ {} to move", current_army.display_name()),
        Style::default()
            .fg(army_color(current_army))
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
                    Style::default().fg(Color::White),
                ));
            } else {
                spans.push(Span::raw("  "));
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
    let mut file_spans = vec![Span::raw("  ")];
    for f in b'a'..=b'h' {
        let label = format!("{:^width$}", (f as char).to_ascii_uppercase(), width = square_width);
        file_spans.push(Span::styled(label, Style::default().fg(Color::Gray)));
    }
    lines.push(Line::from(file_spans));
    
    Text::from(lines)
}

fn army_color(army: Army) -> Color {
    match army {
        Army::Blue => Color::Blue,
        Army::Black => Color::White,
        Army::Red => Color::Red,
        Army::Yellow => Color::Yellow,
    }
}

fn board_square_info(app: &App, square: u8, current_army: Army) -> (char, Style) {
    let base_color = if (square / 8 + square % 8) % 2 == 0 {
        Color::Rgb(30, 30, 30)
    } else {
        Color::Rgb(20, 20, 20)
    };
    let throne_bg = Color::Rgb(80, 45, 15);
    let throne = app.game.board.throne_owner(square);
    let bg = if throne.is_some() {
        throne_bg
    } else {
        base_color
    };
    if let Some((army, kind)) = app.game.board.piece_at(square) {
        let fg = army_color(army);
        let mut style = Style::default().fg(fg).bg(bg);
        if army == current_army {
            style = style.add_modifier(Modifier::BOLD);
        }
        (
            piece_character(army, kind),
            style,
        )
    } else if throne.is_some() {
        // Show throne marker on empty throne squares
        ('◆', Style::default().fg(Color::Rgb(150, 100, 50)).bg(bg))
    } else {
        ('.', Style::default().fg(Color::Gray).bg(bg))
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
