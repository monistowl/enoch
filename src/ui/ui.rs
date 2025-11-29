use crate::engine::arrays::available_arrays;
use crate::engine::game::{Game, Mode};
use crate::engine::types::{Army, PieceKind, PlayerId, Square, Team};
use crate::ui::app::{App, InputMode};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    
    // Layout: Header, Main Body, Input
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1), // Header
                Constraint::Min(15),   // Body
                Constraint::Length(3), // Input
            ]
            .as_ref(),
        )
        .split(size);

    let header_text = if app.input_mode == InputMode::Board {
        "Enochian Chess - [BOARD MODE] (Arrows to move, Enter to select, Esc to exit)"
    } else {
        "Enochian Chess - [COMMAND MODE] (Type command, Tab for Board)"
    };

    let header = Paragraph::new(Span::styled(
        header_text,
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::NONE));
    frame.render_widget(header, layout[0]);

    // Determine layout based on width
    // Min width 80. Board takes ~26 chars.
    // If width > 100, use 4 columns.
    // If width <= 100, maybe 3 columns or specific compact layout.
    
    let body_chunks = if size.width > 100 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20), // Air Captures
                Constraint::Percentage(40), // Board (Center)
                Constraint::Percentage(20), // Earth Captures
                Constraint::Percentage(20), // Info
            ].as_ref())
            .split(layout[1])
    } else {
        // Compact: Left (Info/Captures), Right (Board)
         Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), // Info/Captures
                Constraint::Percentage(60), // Board
            ].as_ref())
            .split(layout[1])
    };

    if size.width > 100 {
        // Wide Layout
        let air_captures = render_captures(app, Team::Air);
        frame.render_widget(air_captures, body_chunks[0]);

        render_centered_board(frame, app, body_chunks[1]);

        let earth_captures = render_captures(app, Team::Earth);
        frame.render_widget(earth_captures, body_chunks[2]);

        render_info_panel(frame, app, body_chunks[3]);
    } else {
        // Compact Layout
        // Left column: Status (Top), History (Middle), Captures (Bottom)
        let left_col = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Status
                Constraint::Min(5),    // History
                Constraint::Length(6), // Air Captures
                Constraint::Length(6), // Earth Captures
            ].as_ref())
            .split(body_chunks[0]);
            
        let status = Paragraph::new(build_status_lines(app))
            .block(Block::default().title("Status").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(status, left_col[0]);

        let history = Paragraph::new(render_history(app))
            .block(Block::default().title("History").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(history, left_col[1]);
        
        frame.render_widget(render_captures(app, Team::Air), left_col[2]);
        frame.render_widget(render_captures(app, Team::Earth), left_col[3]);

        render_centered_board(frame, app, body_chunks[1]);
    }

    // Input
    let input_border_color = if app.input_mode == InputMode::Board {
        Color::Blue
    } else {
        Color::White
    };
    
    let input_line = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Green)),
        Span::raw(app.input.clone()),
        if app.input_mode == InputMode::Normal {
            Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK))
        } else {
            Span::raw("")
        }
    ])))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(input_border_color))
            .title(if app.input_mode == InputMode::Board { " Board Active " } else { " Command Input " })
    );
    frame.render_widget(input_line, layout[2]);
}

fn render_centered_board(frame: &mut Frame, app: &App, area: Rect) {
    let board_widget = Paragraph::new(text_from_board(app))
        .block(
            Block::default()
                .title("Board")
                .borders(Borders::ALL)
                .border_style(if app.input_mode == InputMode::Board {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                }),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }); // Use wrap if needed, but center alignment helps
        
    // To truly center the 8x8 grid (plus coords), we might need to manually pad or let Paragraph handle it.
    // Paragraph alignment centers the text block within the area.
    frame.render_widget(board_widget, area);
}

fn render_info_panel(frame: &mut Frame, app: &App, area: Rect) {
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Status
            Constraint::Min(5),     // History
        ].as_ref())
        .split(area);

    let status = Paragraph::new(build_status_lines(app))
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status, info_chunks[0]);

    let history = Paragraph::new(render_history(app))
        .block(Block::default().title("History").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(history, info_chunks[1]);
}

fn render_captures(app: &App, team: Team) -> Paragraph {
    let title = format!("{} Captures", team.name());
    let mut captured_text = Vec::new();
    
    for army in team.armies() {
        let counts = app.game.board.piece_counts(army);
        // Standard counts
        let standard = [
            (PieceKind::King, 1),
            (PieceKind::Queen, 1),
            (PieceKind::Bishop, 1),
            (PieceKind::Knight, 1),
            (PieceKind::Rook, 1),
            (PieceKind::Pawn, 4),
        ];
        
        for (kind, initial) in standard {
            let current = counts[kind.index()];
            if current < initial {
                let lost = initial - current;
                for _ in 0..lost {
                    captured_text.push(Span::styled(
                        format!("{} ", piece_character(army, kind)),
                        style_for_army(army).add_modifier(Modifier::DIM),
                    ));
                }
            }
        }
    }
    
    let line = if captured_text.is_empty() {
        Line::from(Span::raw("-"))
    } else {
        Line::from(captured_text)
    };

    Paragraph::new(Text::from(vec![line]))
        .block(Block::default().title(title).borders(Borders::ALL))
        .wrap(Wrap { trim: true })
}

fn render_history(app: &App) -> Text {
    let mut lines = Vec::new();
    for (i, cmd) in app.command_history.iter().rev().take(10).enumerate() {
        lines.push(Line::from(format!("{}. {}", app.command_history.len() - i, cmd)));
    }
    Text::from(lines)
}

pub fn render_size_error(frame: &mut Frame, min_width: u16, min_height: u16, size: Rect) {
    let warning = Paragraph::new(Text::from(vec![Line::from(vec![Span::styled(
        format!(
            "Terminal too small: {}x{} (minimum {}x{})",
            size.width, size.height, min_width, min_height
        ),
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )])]))
    .block(Block::default().borders(Borders::ALL).title("Size Error"))
    .alignment(Alignment::Center);
    frame.render_widget(warning, size);
}

fn style_for_army(army: Army) -> Style {
    match army {
        Army::Blue => Style::default().fg(Color::Blue),
        Army::Black => Style::default().fg(Color::White), // Black on terminal usually White/Gray
        Army::Red => Style::default().fg(Color::Red),
        Army::Yellow => Style::default().fg(Color::Yellow),
    }
}

fn build_status_lines(app: &App) -> Text {
    let mut lines = Vec::new();
    let current_army = app.game.state.current_army(&app.game.config);
    
    // BIG Turn Indicator
    lines.push(Line::from(vec![
        Span::raw("TURN: "),
        Span::styled(
            current_army.display_name().to_uppercase(),
            style_for_army(current_army).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ),
    ]));

    if app.game.config.mode == Mode::Divination {
        if let Some(die) = app.game.state.divination_die {
            lines.push(Line::from(vec![
                Span::raw("DIE: "),
                Span::styled(
                    format!("{} ({})", die, die_piece_name(die)),
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }

    let frozen: Vec<&str> = Army::ALL
        .iter()
        .filter(|&&army| app.game.army_is_frozen(army))
        .map(|army| army.display_name())
        .collect();
    if !frozen.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("FROZEN: ", Style::default().fg(Color::Cyan)),
            Span::raw(frozen.join(", ")),
        ]));
    }
    
    // Check Alert
    if app.game.king_in_check(current_army) {
         lines.push(Line::from(Span::styled(
            "!!! CHECK !!!",
            Style::default().fg(Color::Red).add_modifier(Modifier::SLOW_BLINK | Modifier::BOLD),
        )));
    }

    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(
            format!("> {}", msg),
            Style::default().fg(Color::Green),
        )));
    }

    if let Some(ref err) = app.error_message {
        lines.push(Line::from(Span::styled(
            format!("! {}", err),
            Style::default().fg(Color::Red),
        )));
    }

    Text::from(lines)
}

fn die_piece_name(die: u8) -> &'static str {
    match die {
        1 => "King/Pawn",
        2 => "Knight",
        3 => "Bishop",
        4 => "Queen",
        5 => "Rook",
        6 => "Pawn",
        _ => "?",
    }
}

fn text_from_board(app: &App) -> Text {
    let mut lines = Vec::new();
    let current_army = app.game.current_army();
    for rank in (0..8).rev() {
        let mut spans = Vec::new();
        spans.push(Span::styled(
            format!("{} ", rank + 1),
            Style::default().fg(Color::White),
        ));
        for file in 0..8 {
            let square = rank * 8 + file;
            let (chr, style) = board_square_info(app, square, current_army);
            spans.push(Span::styled(format!(" {} ", chr), style));
        }
        lines.push(Line::from(spans));
    }
    let files_line = (b'a'..=b'h')
        .map(|f| format!(" {} ", (f as char).to_ascii_uppercase()))
        .collect::<Vec<_>>()
        .join("");
    lines.push(Line::from(Span::styled(
        format!("  {}", files_line),
        Style::default().fg(Color::Gray),
    )));
    Text::from(lines)
}

fn board_square_info(app: &App, square: u8, current_army: Army) -> (char, Style) {
    let is_cursor = app.input_mode == InputMode::Board && app.cursor_pos == square;
    let is_selected = app.selected_square == Some(square);
    let is_hint = (app.valid_moves & (1u64 << square)) != 0;

    let base_color = if (square / 8 + square % 8) % 2 == 0 {
        Color::Rgb(30, 30, 30)
    } else {
        Color::Rgb(20, 20, 20)
    };
    let throne_bg = Color::Rgb(80, 45, 15);
    let throne = app.game.board.throne_owner(square);
    
    let mut bg = if throne.is_some() {
        throne_bg
    } else {
        base_color
    };

    // Overlay highlighting
    if is_selected {
        bg = Color::Rgb(50, 100, 50); // Greenish selection
    } else if is_cursor {
        bg = Color::Rgb(100, 100, 50); // Yellowish cursor
    } else if is_hint {
         // Subtle hint background
         bg = if throne.is_some() {
             Color::Rgb(100, 60, 30)
         } else {
             Color::Rgb(40, 40, 60)
         };
    }

    if let Some((army, kind)) = app.game.board.piece_at(square) {
        let fg = match army {
            Army::Blue => Color::Blue,
            Army::Black => Color::White,
            Army::Red => Color::Red,
            Army::Yellow => Color::Yellow,
        };
        let mut style = Style::default().fg(fg).bg(bg);
        
        if army == current_army {
            style = style.add_modifier(Modifier::BOLD);
        }
        
        // If captured? No, board only shows active pieces.
        
        if is_hint {
             // Attack hint
             style = style.bg(Color::Rgb(100, 40, 40));
        }

        if is_cursor {
            style = style.add_modifier(Modifier::REVERSED);
        }

        (
            piece_character(army, kind),
            style,
        )
    } else {
        let char = if is_hint { 'x' } else { '.' };
        let mut style = Style::default().fg(Color::Gray).bg(bg);
        if is_hint {
            style = style.fg(Color::Green);
        }
        if is_cursor {
             style = style.add_modifier(Modifier::REVERSED);
        }
        (char, style)
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
    "/new <array>  /save <path>  /load <path>  /mode <div|normal>  /ai  army: e2-e4".to_string()
}