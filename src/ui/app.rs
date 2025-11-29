use crate::engine::ai::Ai;
use crate::engine::arrays::{available_arrays, default_array, find_array_by_name};
use crate::engine::game::{Game, Mode};
use crate::engine::types::{Army, PieceKind, Square};
use std::fmt;
use std::fs;
use std::path::Path;

pub struct App {
    pub game: Game,
    pub current_screen: CurrentScreen,
    pub input_mode: InputMode,
    pub input: String,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    pub command_history: Vec<String>,
    pub selected_array: String,
    pub array_index: usize,
    pub cursor_pos: Square,
    pub selected_square: Option<Square>,
    pub valid_moves: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Board,
}

pub enum CurrentScreen {
    Main,
    Exiting,
}

const MAX_INPUT_LENGTH: usize = 64;

pub enum UiCommand {
    Move {
        army: Army,
        from: Square,
        to: Square,
        promotion: Option<PieceKind>,
    },
    ArraysList,
    Status,
    SelectArray(String),
    CycleArray(isize),
    Exchange(Army),
    Save(String),
    Load(String),
    New(String),
    SetMode(Mode),
    AiMove,
}

#[derive(Debug)]
pub struct CommandParseError(pub String);

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl App {
    pub fn new(_force_halfblocks: bool) -> Self {
        let spec = default_array();
        App {
            game: Game::from_array_spec(spec),
            current_screen: CurrentScreen::Main,
            input_mode: InputMode::Normal,
            input: String::new(),
            status_message: None,
            error_message: None,
            command_history: Vec::new(),
            selected_array: spec.name.to_string(),
            array_index: 0,
            cursor_pos: 0,
            selected_square: None,
            valid_moves: 0,
        }
    }

    pub fn add_char(&mut self, ch: char) {
        if self.input.chars().count() < MAX_INPUT_LENGTH {
            self.input.push(ch);
            self.error_message = None;
        }
    }

    pub fn delete_char(&mut self) {
        self.input.pop();
        self.error_message = None;
    }

    pub fn submit_command(&mut self) {
        let trimmed = self.input.trim();
        if trimmed.is_empty() {
            return;
        }
        match parse_ui_command(trimmed) {
            Ok(command) => {
                self.command_history.push(trimmed.to_string());
                self.execute_command(command);
                self.input.clear();
            }
            Err(err) => {
                self.error_message = Some(err.to_string());
            }
        }
    }

    fn execute_command(&mut self, command: UiCommand) {
        match command {
            UiCommand::Move {
                army,
                from,
                to,
                promotion,
            } => match self.game.apply_move(army, from, to, promotion) {
                Ok(msg) => {
                    self.status_message = Some(msg);
                    self.error_message = None;
                }
                Err(err) => {
                    self.error_message = Some(err);
                }
            },
            UiCommand::ArraysList => {
                let names: Vec<&str> = available_arrays().iter().map(|spec| spec.name).collect();
                self.status_message = Some(format!("Arrays: {}", names.join(", ")));
                self.error_message = None;
            }
            UiCommand::Status => {
                self.status_message = Some(self.build_status_message());
                self.error_message = None;
            }
            UiCommand::SelectArray(name) => {
                self.select_array(&name);
            }
            UiCommand::New(name) => {
                self.select_array(&name);
            }
            UiCommand::CycleArray(direction) => {
                let specs = available_arrays();
                if specs.is_empty() {
                    self.error_message = Some("No arrays available".into());
                    return;
                }
                let len = specs.len();
                let current = self.array_index;
                let offset = if direction >= 0 { 1 } else { len - 1 };
                let next = (current + offset) % len;
                self.load_array(next);
            }
            UiCommand::Exchange(target_army) => {
                let current = self.game.current_army();
                if self.game.exchange_prisoners(current, target_army) {
                    self.status_message = Some(format!(
                        "{} exchanged prisoners with {}",
                        current.display_name(),
                        target_army.display_name()
                    ));
                    self.error_message = None;
                } else {
                    self.error_message =
                        Some("Exchange failed: both kings must be captured and frozen".into());
                }
            }
            UiCommand::Save(path) => {
                match fs::write(&path, self.game.to_enoch_fen()) {
                    Ok(_) => {
                        self.status_message = Some(format!("Game saved to {}", path));
                        self.error_message = None;
                    }
                    Err(e) => self.error_message = Some(format!("Save failed: {}", e)),
                }
            }
            UiCommand::Load(path) => {
                match fs::read_to_string(&path) {
                    Ok(json) => match Game::from_enoch_fen(&json) {
                        Ok(game) => {
                            self.game = game;
                            self.status_message = Some(format!("Game loaded from {}", path));
                            self.error_message = None;
                            self.selected_array = "Custom (Loaded)".to_string(); 
                        }
                        Err(e) => self.error_message = Some(format!("Load invalid: {}", e)),
                    },
                    Err(e) => self.error_message = Some(format!("Read failed: {}", e)),
                }
            }
            UiCommand::SetMode(mode) => {
                self.game.config.mode = mode;
                self.status_message = Some(format!("Mode set to {:?}", mode));
                self.error_message = None;
            }
            UiCommand::AiMove => {
                let ai = Ai::new(2);
                if let Some((army, from, to, promo)) = ai.select_move(&self.game) {
                    match self.game.apply_move(army, from, to, promo) {
                        Ok(msg) => {
                            self.status_message = Some(format!("AI: {}", msg));
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("AI Error: {}", e));
                        }
                    }
                } else {
                    self.error_message = Some("AI could not find a move (Stalemate?)".into());
                }
            }
        }
        if self.status_message.is_some() {
            self.error_message = None;
        }
    }

    fn select_array(&mut self, name: &str) {
        if let Some(spec) = find_array_by_name(name) {
            self.game = Game::from_array_spec(spec);
            self.selected_array = spec.name.to_string();
            self.status_message = Some(format!("Loaded array: {}", spec.name));
            self.error_message = None;
            self.array_index = available_arrays()
                .iter()
                .position(|s| s.name == spec.name)
                .unwrap_or(self.array_index);
        } else {
            self.error_message = Some(format!("Unknown array: {}", name));
        }
    }

    fn build_status_message(&self) -> String {
        let army = self.game.state.current_army(&self.game.config);
        let mut parts = vec![format!("Turn: {}", army.display_name())];
        let frozen: Vec<&str> = Army::ALL
            .iter()
            .filter(|&&a| self.game.army_is_frozen(a))
            .map(|a| a.display_name())
            .collect();
        if !frozen.is_empty() {
            parts.push(format!("Frozen: {}", frozen.join(", ")));
        }
        let stalemated: Vec<&str> = Army::ALL
            .iter()
            .filter(|&&a| self.game.army_in_stalemate(a))
            .map(|a| a.display_name())
            .collect();
        if !stalemated.is_empty() {
            parts.push(format!("Stalemated: {}", stalemated.join(", ")));
        }
        if let Some(team) = self.game.winning_team() {
            parts.push(format!("Winner: {} team", team.name()));
        } else if self.game.draw_condition() {
            parts.push("Draw condition met".into());
        }
        parts.join(" | ")
    }

    pub fn board_rows(&self) -> Vec<String> {
        self.game.board.ascii_rows()
    }

    pub fn history_lines(&self) -> Vec<String> {
        self.command_history.iter().rev().take(4).cloned().collect()
    }

    fn load_array(&mut self, index: usize) {
        if let Some(spec) = available_arrays().get(index) {
            self.game = Game::from_array_spec(spec);
            self.array_index = index;
            self.selected_array = spec.name.to_string();
            self.status_message = Some(format!("Loaded array: {}", spec.name));
            self.error_message = None;
        }
    }

    fn cycle_array(&mut self, direction: isize) {
        let specs = available_arrays();
        if specs.is_empty() {
            self.error_message = Some("No arrays available".into());
            return;
        }
        let len = specs.len();
        let current = self.array_index;
        let offset = if direction >= 0 { 1 } else { len - 1 };
        let next = (current + offset) % len;
        self.load_array(next);
    }

    pub fn cycle_array_direction(&mut self, direction: isize) {
        self.cycle_array(direction);
    }

    pub fn move_cursor(&mut self, dx: i8, dy: i8) {
        let rank = (self.cursor_pos / 8) as i8;
        let file = (self.cursor_pos % 8) as i8;

        let new_rank = (rank + dy).clamp(0, 7);
        let new_file = (file + dx).clamp(0, 7);

        self.cursor_pos = (new_rank * 8 + new_file) as Square;
    }

    pub fn handle_board_enter(&mut self) {
        if let Some(selected) = self.selected_square {
            // Try to move
            if selected == self.cursor_pos {
                // Deselect if clicking same square
                self.selected_square = None;
                self.valid_moves = 0;
                self.status_message = Some("Deselected".into());
                return;
            }

            // Check if target is in valid moves
            if (self.valid_moves & (1u64 << self.cursor_pos)) != 0 {
                let from = selected;
                let to = self.cursor_pos;
                let army = self.game.current_army();

                // Auto-promote to Queen for now if applicable
                let promotion = if self.game.can_promote_at(army, to) {
                     if let Some((_, kind)) = self.game.board.piece_at(from) {
                        if kind == PieceKind::Pawn {
                             Some(PieceKind::Queen)
                        } else {
                            None
                        }
                     } else {
                         None
                     }
                } else {
                    None
                };

                let command = UiCommand::Move {
                    army,
                    from,
                    to,
                    promotion
                };
                self.execute_command(command);

                // Reset selection after move attempt
                self.selected_square = None;
                self.valid_moves = 0;
            } else {
                // Invalid move, maybe select this piece instead if it belongs to current army?
                self.select_square_at_cursor();
            }
        } else {
            self.select_square_at_cursor();
        }
    }

    fn select_square_at_cursor(&mut self) {
        let sq = self.cursor_pos;
        if let Some((army, kind)) = self.game.board.piece_at(sq) {
            if army == self.game.current_army() {
                // Valid selection
                self.selected_square = Some(sq);
                self.valid_moves = self.game.piece_moves(army, kind);
                self.status_message = Some(format!("Selected {:?} at {}", kind, self.square_name(sq)));
                self.error_message = None;
            } else {
                self.error_message = Some("Cannot select enemy/frozen piece or not your turn".into());
            }
        } else {
            self.selected_square = None;
            self.valid_moves = 0;
            self.status_message = Some(format!("Empty square {}", self.square_name(sq)));
        }
    }

    fn square_name(&self, sq: Square) -> String {
        let file = (sq % 8) as u8;
        let rank = (sq / 8) as u8;
        format!("{}{}", (b'a' + file) as char, rank + 1)
    }

    pub fn handle_board_esc(&mut self) {
        if self.selected_square.is_some() {
            self.selected_square = None;
            self.valid_moves = 0;
            self.status_message = Some("Selection cleared".into());
        } else {
            self.input_mode = InputMode::Normal;
            self.status_message = Some("Command Mode".into());
        }
    }
}

fn parse_ui_command(input: &str) -> Result<UiCommand, CommandParseError> {
    if input.starts_with('/') {
        let mut parts = input[1..].split_whitespace();
        if let Some(cmd) = parts.next() {
            match cmd.to_lowercase().as_str() {
                "arrays" => Ok(UiCommand::ArraysList),
                "status" => Ok(UiCommand::Status),
                "ai" => Ok(UiCommand::AiMove),
                "array" => {
                    if let Some(arg) = parts.next() {
                        match arg.to_lowercase().as_str() {
                            "next" => Ok(UiCommand::CycleArray(1)),
                            "prev" | "previous" => Ok(UiCommand::CycleArray(-1)),
                            _ => Ok(UiCommand::SelectArray(arg.to_string())),
                        }
                    } else {
                        Err(CommandParseError("Missing array name".into()))
                    }
                }
                "new" => {
                    if let Some(arg) = parts.next() {
                        Ok(UiCommand::New(arg.to_string()))
                    } else {
                        Err(CommandParseError("Missing array name".into()))
                    }
                }
                "exchange" => {
                    if let Some(name) = parts.next() {
                        match Army::from_str(name) {
                            Some(army) => Ok(UiCommand::Exchange(army)),
                            None => Err(CommandParseError("Unknown army".into())),
                        }
                    } else {
                        Err(CommandParseError("Missing army name".into()))
                    }
                }
                "save" => {
                    if let Some(path) = parts.next() {
                        Ok(UiCommand::Save(path.to_string()))
                    } else {
                        Err(CommandParseError("Missing file path".into()))
                    }
                }
                "load" => {
                    if let Some(path) = parts.next() {
                        Ok(UiCommand::Load(path.to_string()))
                    } else {
                        Err(CommandParseError("Missing file path".into()))
                    }
                }
                "mode" => {
                    if let Some(mode_str) = parts.next() {
                        match mode_str.to_lowercase().as_str() {
                            "normal" => Ok(UiCommand::SetMode(Mode::Normal)),
                            "divination" => Ok(UiCommand::SetMode(Mode::Divination)),
                            _ => Err(CommandParseError("Unknown mode. Use 'normal' or 'divination'".into())),
                        }
                    } else {
                        Err(CommandParseError("Missing mode (normal/divination)".into()))
                    }
                }
                _ => Err(CommandParseError("Unknown command".into())),
            }
        } else {
            Err(CommandParseError("Empty command".into()))
        }
    } else {
        parse_move_command(input)
    }
}

fn parse_move_command(input: &str) -> Result<UiCommand, CommandParseError> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err(CommandParseError(
            "Move must follow format `army: e2-e4`".into(),
        ));
    }
    let army_name = parts[0].trim();
    let army = Army::from_str(army_name).ok_or_else(|| CommandParseError("Unknown army".into()))?;
    let move_part = parts[1].trim();
    let promo_split: Vec<&str> = move_part.split('=').collect();
    let (move_segment, promotion) = if promo_split.len() == 2 {
        (promo_split[0], Some(promo_split[1]))
    } else {
        (move_part, None)
    };
    let move_segment = move_segment.replace('x', "-");
    let coords: Vec<&str> = move_segment.split('-').collect();
    if coords.len() != 2 {
        return Err(CommandParseError(
            "Move must contain source and destination".into(),
        ));
    }
    let from = parse_square(coords[0].trim())
        .ok_or_else(|| CommandParseError("Invalid source square".into()))?;
    let to = parse_square(coords[1].trim())
        .ok_or_else(|| CommandParseError("Invalid destination square".into()))?;
    let promotion_kind = promotion
        .map(|code| match code.to_uppercase().as_str() {
            "Q" => Some(PieceKind::Queen),
            "R" => Some(PieceKind::Rook),
            "B" => Some(PieceKind::Bishop),
            "N" => Some(PieceKind::Knight),
            _ => None,
        })
        .flatten();

    if promotion.is_some() && promotion_kind.is_none() {
        return Err(CommandParseError("Invalid promotion piece".into()));
    }

    Ok(UiCommand::Move {
        army,
        from,
        to,
        promotion: promotion_kind,
    })
}

fn parse_square(token: &str) -> Option<Square> {
    let chars: Vec<char> = token.chars().collect();
    if chars.len() != 2 {
        return None;
    }
    let file_char = chars[0].to_ascii_lowercase();
    let rank_char = chars[1];
    if !('a'..='h').contains(&file_char) || !('1'..='8').contains(&rank_char) {
        return None;
    }
    let file = file_char as u8 - b'a';
    let rank = rank_char as u8 - b'1';
    Some(rank as Square * 8 + file as Square)
}