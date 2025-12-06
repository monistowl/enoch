//! WASM bindings for Enochian Chess engine
//!
//! Exposes the game engine to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::engine::arrays::TABLET_OF_FIRE_FIRE;
use crate::engine::board::Board;
use crate::engine::game::{Game, Status};
use crate::engine::moves::{
    compute_bishops_moves, compute_king_moves, compute_knights_moves,
    compute_pawns_moves, compute_queens_moves, compute_rooks_moves,
};
use crate::engine::types::{Army, PieceKind, Square, ARMY_COUNT, PIECE_KIND_COUNT};

// Initialize panic hook for better error messages in console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Helper to serialize data to JsValue, returning an error object on failure
fn to_js_value<T: Serialize>(data: &T) -> JsValue {
    serde_wasm_bindgen::to_value(data).unwrap_or_else(|e| {
        // Return an error object that JavaScript can handle
        let error = SerializationError {
            error: true,
            message: format!("Serialization failed: {}", e),
        };
        serde_wasm_bindgen::to_value(&error).unwrap_or(JsValue::NULL)
    })
}

/// Error response for serialization failures
#[derive(Serialize)]
struct SerializationError {
    error: bool,
    message: String,
}

/// Square data for JSON serialization
#[derive(Serialize, Deserialize)]
pub struct SquareData {
    pub index: u8,
    pub notation: String,
    pub piece: Option<PieceData>,
}

/// Piece data for JSON serialization
#[derive(Serialize, Deserialize)]
pub struct PieceData {
    pub army: String,
    pub kind: String,
    pub code: String,  // e.g., "BK" for Blue King
    pub glyph: String, // e.g., "♚"
    pub frozen: bool,
}

/// Game state for JSON serialization
#[derive(Serialize, Deserialize)]
pub struct GameStateData {
    pub current_army: String,
    pub current_team: String,
    pub frozen: FrozenState,
    pub status: String,
    pub in_check: CheckState,
    pub winner: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FrozenState {
    pub blue: bool,
    pub black: bool,
    pub red: bool,
    pub yellow: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CheckState {
    pub blue: bool,
    pub black: bool,
    pub red: bool,
    pub yellow: bool,
}

/// Legal moves result
#[derive(Serialize, Deserialize)]
pub struct LegalMovesResult {
    pub from: u8,
    pub moves: Vec<u8>,
    pub captures: Vec<u8>,
}

/// Move result
#[derive(Serialize, Deserialize)]
pub struct MoveResult {
    pub success: bool,
    pub message: String,
    pub captured: Option<PieceData>,
    pub promotion: Option<String>,
}

/// WASM-exposed game wrapper
#[wasm_bindgen]
pub struct WasmGame {
    game: Game,
}

#[wasm_bindgen]
impl WasmGame {
    /// Create a new game with the default Tablet of Fire array
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        let game = Game::from_array_spec(&TABLET_OF_FIRE_FIRE);
        WasmGame { game }
    }

    /// Get the current board state as JSON
    /// Returns an array of 64 squares with piece data
    #[wasm_bindgen(js_name = getBoardState)]
    pub fn get_board_state(&self) -> JsValue {
        let mut squares: Vec<SquareData> = Vec::with_capacity(64);

        for sq in 0..64u8 {
            let notation = square_to_notation(sq);
            let piece = self.game.board.piece_at(sq).map(|(army, kind)| {
                let frozen = self.game.army_is_frozen(army);
                PieceData {
                    army: army.display_name().to_string(),
                    kind: kind_name(kind).to_string(),
                    code: format!("{}{}", army_char(army), kind_char(kind)),
                    glyph: piece_glyph(kind).to_string(),
                    frozen,
                }
            });

            squares.push(SquareData {
                index: sq,
                notation,
                piece,
            });
        }

        to_js_value(&squares)
    }

    /// Get current game state (turn, frozen armies, status)
    #[wasm_bindgen(js_name = getGameState)]
    pub fn get_game_state(&self) -> JsValue {
        let current = self.game.current_army();
        let state = GameStateData {
            current_army: current.display_name().to_string(),
            current_team: current.team().name().to_string(),
            frozen: FrozenState {
                blue: self.game.army_is_frozen(Army::Blue),
                black: self.game.army_is_frozen(Army::Black),
                red: self.game.army_is_frozen(Army::Red),
                yellow: self.game.army_is_frozen(Army::Yellow),
            },
            status: match self.game.status {
                Status::Ongoing => "ongoing".to_string(),
                Status::Draw => "draw".to_string(),
                Status::Checkmate => "checkmate".to_string(),
            },
            in_check: CheckState {
                blue: self.game.king_in_check(Army::Blue),
                black: self.game.king_in_check(Army::Black),
                red: self.game.king_in_check(Army::Red),
                yellow: self.game.king_in_check(Army::Yellow),
            },
            winner: self.game.winning_team().map(|t| t.name().to_string()),
        };

        to_js_value(&state)
    }

    /// Get legal moves for a piece at a given square
    /// Returns squares the piece can move to
    #[wasm_bindgen(js_name = getLegalMoves)]
    pub fn get_legal_moves(&self, square: u8) -> JsValue {
        let piece = self.game.board.piece_at(square);

        let result = if let Some((army, kind)) = piece {
            // Only show moves if it's this army's turn and army isn't frozen
            if army != self.game.current_army() || self.game.army_is_frozen(army) {
                LegalMovesResult {
                    from: square,
                    moves: vec![],
                    captures: vec![],
                }
            } else {
                let moves_bitboard = self.piece_moves_from_square(army, kind, square);
                // Enemy mask = pieces belonging to the opposing TEAM
                let opponent_team = army.team().opponent();
                let enemy_mask = opponent_team.armies().iter()
                    .map(|&a| self.game.board.occupancy_by_army[a.index()])
                    .fold(0u64, |acc, occ| acc | occ);

                let mut moves = Vec::new();
                let mut captures = Vec::new();

                for sq in 0..64u8 {
                    let mask = 1u64 << sq;
                    if moves_bitboard & mask != 0 {
                        if enemy_mask & mask != 0 {
                            captures.push(sq);
                        } else {
                            moves.push(sq);
                        }
                    }
                }

                LegalMovesResult {
                    from: square,
                    moves,
                    captures,
                }
            }
        } else {
            LegalMovesResult {
                from: square,
                moves: vec![],
                captures: vec![],
            }
        };

        to_js_value(&result)
    }

    /// Apply a move from one square to another
    /// Returns success/failure with message
    #[wasm_bindgen(js_name = applyMove)]
    pub fn apply_move(&mut self, from: u8, to: u8) -> JsValue {
        let army = self.game.current_army();

        // Check for piece at destination before move (might be captured or overlaid)
        let piece_before = self.game.board.piece_at(to);

        match self.game.apply_move(army, from, to, None) {
            Ok(msg) => {
                // Check if piece was actually captured (gone after move) vs overlaid (still there)
                let piece_after = self.game.board.piece_at(to);
                let captured_piece = if let Some((cap_army, cap_kind)) = piece_before {
                    // Only report as captured if the original piece is no longer at destination
                    // (throne overlays keep both pieces, so original is still there)
                    let still_there = piece_after.map(|(a, k)| a == cap_army && k == cap_kind).unwrap_or(false);
                    if still_there {
                        None // Throne overlay, not a capture
                    } else {
                        Some(PieceData {
                            army: cap_army.display_name().to_string(),
                            kind: kind_name(cap_kind).to_string(),
                            code: format!("{}{}", army_char(cap_army), kind_char(cap_kind)),
                            glyph: piece_glyph(cap_kind).to_string(),
                            frozen: false,
                        })
                    }
                } else {
                    None
                };

                let result = MoveResult {
                    success: true,
                    message: msg,
                    captured: captured_piece,
                    promotion: None,
                };
                to_js_value(&result)
            }
            Err(err) => {
                let result = MoveResult {
                    success: false,
                    message: err,
                    captured: None,
                    promotion: None,
                };
                to_js_value(&result)
            }
        }
    }

    /// Get all legal moves for the current army
    #[wasm_bindgen(js_name = getAllLegalMoves)]
    pub fn get_all_legal_moves(&self) -> JsValue {
        let army = self.game.current_army();
        let mut all_moves: Vec<(u8, Vec<u8>)> = Vec::new();

        if self.game.army_is_frozen(army) {
            return to_js_value(&all_moves);
        }

        for sq in 0..64u8 {
            if let Some((piece_army, kind)) = self.game.board.piece_at(sq) {
                if piece_army == army {
                    let moves_bb = self.piece_moves_from_square(army, kind, sq);
                    let moves: Vec<u8> = (0..64)
                        .filter(|&s| (moves_bb >> s) & 1 != 0)
                        .collect();
                    if !moves.is_empty() {
                        all_moves.push((sq, moves));
                    }
                }
            }
        }

        to_js_value(&all_moves)
    }

    /// Helper to get legal moves for a specific piece at a square
    fn piece_moves_from_square(&self, army: Army, kind: PieceKind, square: Square) -> u64 {
        let board = &self.game.board;
        let own_pieces = board.occupancy_by_army[army.index()];
        let ally = army.ally();
        let ally_pieces = board.occupancy_by_army[ally.index()];
        let team_pieces = own_pieces | ally_pieces;

        // For pawns, compute single-piece moves
        if kind == PieceKind::Pawn {
            return self.pawn_moves_from_square(army, square);
        }

        // For other pieces, compute all moves and filter to those reachable from this square
        let all_moves = match kind {
            PieceKind::King => compute_king_moves(board, army),
            PieceKind::Queen => compute_queens_moves(board, army),
            PieceKind::Rook => compute_rooks_moves(board, army),
            PieceKind::Bishop => compute_bishops_moves(board, army),
            PieceKind::Knight => compute_knights_moves(board, army),
            PieceKind::Pawn => unreachable!(),
        };

        // Filter out moves to allied squares (except throne overlays)
        let throne_targets = board.team_king_thrones_for_overlay(army);
        all_moves & !(team_pieces & !throne_targets)
    }

    /// Compute moves for a single pawn at a specific square
    fn pawn_moves_from_square(&self, army: Army, square: Square) -> u64 {
        let board = &self.game.board;
        let file = (square % 8) as i8;
        let rank = (square / 8) as i8;

        let own_pieces = board.occupancy_by_army[army.index()];
        let ally = army.ally();
        let ally_pieces = board.occupancy_by_army[ally.index()];
        let team_pieces = own_pieces | ally_pieces;

        // Get opponent team's pieces for captures
        let opponent_team = army.team().opponent();
        let enemy_mask = opponent_team.armies().iter()
            .map(|&a| board.occupancy_by_army[a.index()])
            .fold(0u64, |acc, occ| acc | occ);

        let throne_targets = board.team_king_thrones_for_overlay(army);

        // Helper to convert file,rank offset to destination square
        let offset_square = |df: i8, dr: i8| -> Option<u8> {
            let nf = file + df;
            let nr = rank + dr;
            if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                Some((nr * 8 + nf) as u8)
            } else {
                None
            }
        };

        let (forward, diag_left, diag_right) = match army {
            Army::Blue => (offset_square(0, 1), offset_square(-1, 1), offset_square(1, 1)),
            Army::Red => (offset_square(0, -1), offset_square(-1, -1), offset_square(1, -1)),
            Army::Black => (offset_square(1, 0), offset_square(1, 1), offset_square(1, -1)),
            Army::Yellow => (offset_square(-1, 0), offset_square(-1, 1), offset_square(-1, -1)),
        };

        let mut moves = 0u64;

        // Forward move (only if square is empty)
        if let Some(dest) = forward {
            let dest_mask = 1u64 << dest;
            if board.all_occupancy & dest_mask == 0 {
                moves |= dest_mask;
            }
        }

        // Diagonal captures (only if enemy piece is there, or throne overlay)
        for diag in [diag_left, diag_right] {
            if let Some(dest) = diag {
                let dest_mask = 1u64 << dest;
                // Can capture enemy, or overlay on allied king's throne
                if (enemy_mask & dest_mask != 0) || (throne_targets & dest_mask != 0 && team_pieces & dest_mask == 0) {
                    moves |= dest_mask;
                }
            }
        }

        moves
    }
}

// Helper functions
fn square_to_notation(sq: u8) -> String {
    let file = (b'a' + (sq % 8)) as char;
    let rank = (sq / 8) + 1;
    format!("{}{}", file, rank)
}

fn army_char(army: Army) -> char {
    match army {
        Army::Blue => 'B',
        Army::Black => 'K',
        Army::Red => 'R',
        Army::Yellow => 'Y',
    }
}

fn kind_char(kind: PieceKind) -> char {
    match kind {
        PieceKind::King => 'K',
        PieceKind::Queen => 'Q',
        PieceKind::Rook => 'R',
        PieceKind::Bishop => 'B',
        PieceKind::Knight => 'N',
        PieceKind::Pawn => 'P',
    }
}

fn kind_name(kind: PieceKind) -> &'static str {
    match kind {
        PieceKind::King => "King",
        PieceKind::Queen => "Queen",
        PieceKind::Rook => "Rook",
        PieceKind::Bishop => "Bishop",
        PieceKind::Knight => "Knight",
        PieceKind::Pawn => "Pawn",
    }
}

fn piece_glyph(kind: PieceKind) -> char {
    match kind {
        PieceKind::King => '♚',
        PieceKind::Queen => '♛',
        PieceKind::Rook => '♜',
        PieceKind::Bishop => '♝',
        PieceKind::Knight => '♞',
        PieceKind::Pawn => '♟',
    }
}
