use crate::engine::board::Board;
use crate::engine::piece_kind::{parse_move, ParsedMove, SpecialMove};
use crate::engine::types::{Army, PieceKind, Team};

/// Game struct responsible for all game logics (pin, check, valid captures, etc)
pub struct Game {
    pub board: Board,
    pub turn: Army,
    // check
    pub check: bool,

    // pin
    pub pinned_pieces: [u64; 4],

    // en passant target square (not piece)
    pub en_passant_target: u64,

    // end game (checkmate, draw)
    pub status: Status,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InvalidMoveReason {
    NoSourceOrTarget,
    InvalidSourceOrTarget,
    MultipleTargets,
    InvalidCaptureTarget,
    KingCaptureMove,
    PawnNonDiagonalCapture,
    PawnInvalidPromotion,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MoveError {
    AmbiguousSource,
    InvalidMove(InvalidMoveReason),
    Pinned,
    Checked,
    ParseError,
    GameOver,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ongoing,
    Draw,
    Checkmate,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game {
            board,
            turn: Army::Blue,
            check: false,
            pinned_pieces: [0, 0, 0, 0],
            en_passant_target: 0,
            status: Status::Ongoing,
        }
    }

    fn get_pieces(board: &Board, piece_kind: PieceKind, army: Army) -> u64 {
        board.by_army_kind[army as usize][piece_kind as usize]
    }
}

impl Default for Game {
    fn default() -> Game {
        Self::new(Board::default())
    }
}
