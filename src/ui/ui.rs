use crate::engine::arrays::available_arrays;
use crate::engine::types::{Army, PieceKind, PlayerId, Team};
use crate::ui::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    let header = Paragraph::new(Span::styled(
        "Enochian Chess (type /arrays, /status, army: e2-e4)",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("Help"));
    frame.render_widget(header, layout[0]);

    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
        .split(layout[1]);

    let board = Paragraph::new(text_from_board(app))
        .block(
            Block::default()
                .title("Enochian Board")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });
    frame.render_widget(board, mid_chunks[0]);

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
    lines.push(Line::from(vec![Span::styled(
        format!("Turn: {}", current_army.display_name()),
        Style::default().fg(Color::LightBlue),
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
        lines.push(Line::from(Span::raw(format!(
            "Frozen armies: {}",
            frozen.join(", ")
        ))));
    }

    let stalemated: Vec<&str> = Army::ALL
        .iter()
        .filter(|&&army| app.game.state.is_stalemated(army))
        .map(|army| army.display_name())
        .collect();
    if !stalemated.is_empty() {
        lines.push(Line::from(Span::raw(format!(
            "Stalemated: {}",
            stalemated.join(", ")
        ))));
    }

    if let Some(ref msg) = app.status_message {
        lines.push(Line::from(Span::styled(
            format!("Status: {}", msg),
            Style::default().fg(Color::White),
        )));
    }

    if let Some(ref err) = app.error_message {
        lines.push(Line::from(Span::styled(
            format!("Error: {}", err),
            Style::default().fg(Color::Red),
        )));
    }

    let history = app.history_lines();
    if !history.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("Last commands: {}", history.join(", ")),
            Style::default().fg(Color::Gray),
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
    for &army in Army::ALL.iter() {
        let status = if app.game.army_is_frozen(army) {
            "Frozen"
        } else if app.game.state.is_stalemated(army) {
            "Stalemated"
        } else {
            "Active"
        };
        let style = match army {
            Army::Blue => Style::default().fg(Color::Blue),
            Army::Black => Style::default().fg(Color::White),
            Army::Red => Style::default().fg(Color::Red),
            Army::Yellow => Style::default().fg(Color::Yellow),
        };
        let controller = controller_label(app.game.board.controller_for(army));
        lines.push(Line::from(Span::styled(
            format!(
                "{} ({}) - {} - {}",
                army.display_name(),
                army.team().name(),
                status,
                controller
            ),
            style,
        )));
    }
    lines
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
        (
            piece_character(army, kind),
            style,
        )
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
    "/arrays  /status  army: e2-e4  /array <name|next|prev>  /exchange <army>".to_string()
}
