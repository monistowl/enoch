//! WASM bindings for Enochian Chess engine
//!
//! Exposes the game engine to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::engine::arrays::TABLET_OF_FIRE_PROTOTYPE;
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
        let game = Game::from_array_spec(&TABLET_OF_FIRE_PROTOTYPE);
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

        serde_wasm_bindgen::to_value(&squares).unwrap()
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

        serde_wasm_bindgen::to_value(&state).unwrap()
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
                let moves_bitboard = self.piece_legal_moves(army, kind);
                let enemy_mask = self.game.board.all_occupancy
                    & !self.game.board.occupancy_by_army[army.index()];

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

        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    /// Apply a move from one square to another
    /// Returns success/failure with message
    #[wasm_bindgen(js_name = applyMove)]
    pub fn apply_move(&mut self, from: u8, to: u8) -> JsValue {
        let army = self.game.current_army();

        // Check for capture before move
        let captured_piece = self.game.board.piece_at(to).map(|(cap_army, cap_kind)| {
            PieceData {
                army: cap_army.display_name().to_string(),
                kind: kind_name(cap_kind).to_string(),
                code: format!("{}{}", army_char(cap_army), kind_char(cap_kind)),
                glyph: piece_glyph(cap_kind).to_string(),
                frozen: false,
            }
        });

        match self.game.apply_move(army, from, to, None) {
            Ok(msg) => {
                let result = MoveResult {
                    success: true,
                    message: msg,
                    captured: captured_piece,
                    promotion: None,
                };
                serde_wasm_bindgen::to_value(&result).unwrap()
            }
            Err(err) => {
                let result = MoveResult {
                    success: false,
                    message: err,
                    captured: None,
                    promotion: None,
                };
                serde_wasm_bindgen::to_value(&result).unwrap()
            }
        }
    }

    /// Get all legal moves for the current army
    #[wasm_bindgen(js_name = getAllLegalMoves)]
    pub fn get_all_legal_moves(&self) -> JsValue {
        let army = self.game.current_army();
        let mut all_moves: Vec<(u8, Vec<u8>)> = Vec::new();

        if self.game.army_is_frozen(army) {
            return serde_wasm_bindgen::to_value(&all_moves).unwrap();
        }

        for sq in 0..64u8 {
            if let Some((piece_army, kind)) = self.game.board.piece_at(sq) {
                if piece_army == army {
                    let moves_bb = self.piece_legal_moves(army, kind);
                    let moves: Vec<u8> = (0..64)
                        .filter(|&s| (moves_bb >> s) & 1 != 0)
                        .collect();
                    if !moves.is_empty() {
                        all_moves.push((sq, moves));
                    }
                }
            }
        }

        serde_wasm_bindgen::to_value(&all_moves).unwrap()
    }

    /// Helper to get legal moves bitboard for a piece
    fn piece_legal_moves(&self, army: Army, kind: PieceKind) -> u64 {
        match kind {
            PieceKind::King => compute_king_moves(&self.game.board, army),
            PieceKind::Queen => compute_queens_moves(&self.game.board, army),
            PieceKind::Rook => compute_rooks_moves(&self.game.board, army),
            PieceKind::Bishop => compute_bishops_moves(&self.game.board, army),
            PieceKind::Knight => compute_knights_moves(&self.game.board, army),
            PieceKind::Pawn => {
                let (moves, attacks) = compute_pawns_moves(&self.game.board, army);
                let enemy_mask = self.game.board.all_occupancy
                    & !self.game.board.occupancy_by_army[army.index()];
                moves | (attacks & enemy_mask)
            }
        }
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
