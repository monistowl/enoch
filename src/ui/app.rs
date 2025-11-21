use crate::engine::arrays::{available_arrays, default_array, find_array_by_name};
use crate::engine::game::Game;
use crate::engine::types::{Army, PieceKind, Square};
use std::fmt;
use std::fs;

pub struct App {
    pub game: Game,
    pub current_screen: CurrentScreen,
    pub input: String,
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    pub command_history: Vec<String>,
    pub selected_array: String,
    pub array_index: usize,
    pub help_scroll: usize,
}

pub enum CurrentScreen {
    Main,
    Help,
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
    ToggleDivination,
    RollDie,
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
            input: String::new(),
            status_message: None,
            error_message: None,
            command_history: Vec::new(),
            selected_array: spec.name.to_string(),
            array_index: 0,
            help_scroll: 0,
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
                if let Some(spec) = find_array_by_name(&name) {
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
            UiCommand::Save(filename) => match self.game.to_json() {
                Ok(json) => match fs::write(&filename, json) {
                    Ok(_) => {
                        self.status_message = Some(format!("Game saved to {}", filename));
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to write file: {}", e));
                    }
                },
                Err(e) => {
                    self.error_message = Some(format!("Serialization error: {}", e));
                }
            },
            UiCommand::Load(filename) => match fs::read_to_string(&filename) {
                Ok(json) => match Game::from_json(&json) {
                    Ok(game) => {
                        self.game = game;
                        self.status_message = Some(format!("Game loaded from {}", filename));
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Deserialization error: {}", e));
                    }
                },
                Err(e) => {
                    self.error_message = Some(format!("Failed to read file: {}", e));
                }
            },
            UiCommand::ToggleDivination => {
                self.game.config.divination_mode = !self.game.config.divination_mode;
                let mode = if self.game.config.divination_mode {
                    "enabled"
                } else {
                    "disabled"
                };
                self.status_message = Some(format!("Divination mode {}", mode));
                self.error_message = None;
            }
            UiCommand::RollDie => {
                if !self.game.config.divination_mode {
                    self.error_message = Some("Divination mode not enabled. Use /divination to enable.".into());
                    return;
                }
                let roll = Game::roll_die();
                let kinds = Game::die_to_piece_kind(roll);
                let kind_names: Vec<&str> = kinds.iter().map(|k| k.name()).collect();
                let army = self.game.current_army();
                let moves = self.game.generate_divination_moves(army, roll);
                
                if moves.is_empty() {
                    self.status_message = Some(format!(
                        "🎲 Rolled {}: {} - No moves available (recorded as 'No Move')",
                        roll,
                        kind_names.join(" or ")
                    ));
                } else {
                    self.status_message = Some(format!(
                        "🎲 Rolled {}: {} - {} legal moves available",
                        roll,
                        kind_names.join(" or "),
                        moves.len()
                    ));
                }
                self.error_message = None;
            }
        }
        if self.status_message.is_some() {
            self.error_message = None;
        }
    }

    fn build_status_message(&self) -> String {
        let army = self.game.state.current_army(&self.game.config);
        let mut parts = vec![format!("Turn: {}", army.display_name())];
        
        if self.game.config.divination_mode {
            parts.push("🎲 Divination".to_string());
        }
        
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

    pub fn scroll_help(&mut self, delta: isize) {
        if delta > 0 {
            self.help_scroll = self.help_scroll.saturating_add(delta as usize);
        } else {
            self.help_scroll = self.help_scroll.saturating_sub((-delta) as usize);
        }
    }

    pub fn get_help_text() -> Vec<String> {
        vec![
            "═══════════════════════════════════════════════════════════════════".to_string(),
            "                    ENOCHIAN CHESS - QUICK REFERENCE".to_string(),
            "═══════════════════════════════════════════════════════════════════".to_string(),
            "".to_string(),
            "GAME BASICS".to_string(),
            "───────────".to_string(),
            "• Four armies: Blue, Black, Red, Yellow".to_string(),
            "• Teams: Air (Blue + Black) vs Earth (Red + Yellow)".to_string(),
            "• Turn order: Blue → Red → Black → Yellow (default array)".to_string(),
            "• Goal: Capture enemy kings to freeze their armies".to_string(),
            "".to_string(),
            "PIECE MOVEMENTS".to_string(),
            "───────────────".to_string(),
            "• King: 1 square in any direction".to_string(),
            "• Queen: Leaps exactly 2 squares (orthogonal or diagonal)".to_string(),
            "• Rook: Slides any distance orthogonally".to_string(),
            "• Bishop: Slides any distance diagonally".to_string(),
            "• Knight: L-shape (2+1 squares)".to_string(),
            "• Pawn: 1 square forward, captures diagonally".to_string(),
            "".to_string(),
            "SPECIAL RULES".to_string(),
            "──────────────".to_string(),
            "• Queens CANNOT capture queens".to_string(),
            "• Bishops CANNOT capture bishops".to_string(),
            "• Queens CAN capture bishops, bishops CAN capture queens".to_string(),
            "• Bishops/queens use different diagonal systems (Aries/Cancer)".to_string(),
            "".to_string(),
            "CHECK & KING CAPTURE".to_string(),
            "────────────────────".to_string(),
            "• No checkmate - kings are CAPTURED".to_string(),
            "• If in check with legal king moves: MUST move king".to_string(),
            "• If in check with no king moves: may move other pieces".to_string(),
            "• Captured king = army becomes FROZEN (cannot move/attack)".to_string(),
            "".to_string(),
            "THRONE SQUARES".to_string(),
            "──────────────".to_string(),
            "• Each army has a throne square (king's starting position)".to_string(),
            "• Moving your king onto ally's throne = gain control of that army".to_string(),
            "• Frozen pieces revive when you control their throne".to_string(),
            "".to_string(),
            "PROMOTION".to_string(),
            "──────────".to_string(),
            "• Blue pawns promote on rank 8 (north)".to_string(),
            "• Red pawns promote on rank 1 (south)".to_string(),
            "• Black pawns promote on file h (east)".to_string(),
            "• Yellow pawns promote on file a (west)".to_string(),
            "• Privileged pawn: with only K+Q+P or K+B+P, pawn can promote to any piece".to_string(),
            "".to_string(),
            "STALEMATE & DRAWS".to_string(),
            "─────────────────".to_string(),
            "• Stalemate: King not in check but no legal moves → skip turns".to_string(),
            "• Draw: Both allied kings bare, or four bare kings".to_string(),
            "".to_string(),
            "DIVINATION MODE".to_string(),
            "───────────────".to_string(),
            "• Ancient mode using dice to select pieces (like Chaturanga)".to_string(),
            "• Roll 1: King or Pawn | 2: Knight | 3: Bishop".to_string(),
            "• Roll 4: Queen | 5: Rook | 6: Pawn".to_string(),
            "• Must move the piece type rolled (if legal moves exist)".to_string(),
            "• 'No Move' recorded if no legal moves for rolled piece".to_string(),
            "".to_string(),
            "COMMANDS".to_string(),
            "────────".to_string(),
            "• Move: blue: e2-e4 or blue: e2xe4".to_string(),
            "• Promote: blue: e7-e8=Q".to_string(),
            "• /arrays - List available starting arrays".to_string(),
            "• /array <name> - Load specific array".to_string(),
            "• /array next - Cycle to next array".to_string(),
            "• /array prev - Cycle to previous array".to_string(),
            "• /status - Show game status".to_string(),
            "• /divination - Toggle divination mode (dice-based play)".to_string(),
            "• /roll - Roll die for divination mode".to_string(),
            "• /exchange <army> - Exchange prisoners with army".to_string(),
            "• /save <file> - Save game to file".to_string(),
            "• /load <file> - Load game from file".to_string(),
            "• [ ] - Cycle arrays with bracket keys".to_string(),
            "• ? or F1 - Toggle this help screen".to_string(),
            "• ESC - Exit help or quit game".to_string(),
            "".to_string(),
            "TIPS".to_string(),
            "────".to_string(),
            "• Watch for frozen armies (❄) - they can't move!".to_string(),
            "• Check indicator (⚠) shows when king is in danger".to_string(),
            "• Throne squares have bronze background (◆ when empty)".to_string(),
            "• Current army's pieces shown in BOLD".to_string(),
            "• Teams matter for victory - capture both enemy kings to win!".to_string(),
            "".to_string(),
            "Press ↑/↓ or PgUp/PgDn to scroll • ESC to close help".to_string(),
        ]
    }
}

fn parse_ui_command(input: &str) -> Result<UiCommand, CommandParseError> {
    if input.starts_with('/') {
        let mut parts = input[1..].split_whitespace();
        if let Some(cmd) = parts.next() {
            match cmd.to_lowercase().as_str() {
                "arrays" => Ok(UiCommand::ArraysList),
                "status" => Ok(UiCommand::Status),
                "divination" | "div" => Ok(UiCommand::ToggleDivination),
                "roll" | "dice" => Ok(UiCommand::RollDie),
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
                    if let Some(filename) = parts.next() {
                        Ok(UiCommand::Save(filename.to_string()))
                    } else {
                        Err(CommandParseError("Missing filename".into()))
                    }
                }
                "load" => {
                    if let Some(filename) = parts.next() {
                        Ok(UiCommand::Load(filename.to_string()))
                    } else {
                        Err(CommandParseError("Missing filename".into()))
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
