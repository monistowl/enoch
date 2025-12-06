use crate::engine::types::{
    Army, DiagonalSystem, Piece, PieceKind, PlayerId, Square, Team, ARMY_COUNT, PIECE_KIND_COUNT, TEAM_COUNT,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct ArmyState {
    pub army: Army,
    pub throne_squares: [Square; 2],
    pub controller: PlayerId,
    pub is_frozen: bool,
}

impl ArmyState {
    pub const fn new(army: Army, throne_squares: [Square; 2], controller: PlayerId) -> Self {
        Self {
            army,
            throne_squares,
            controller,
            is_frozen: false,
        }
    }
}

/// Represents a piece stored in the throne overlay (hidden under a king on its throne).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OverlayPiece {
    pub army: Army,
    pub kind: PieceKind,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub by_army_kind: [[u64; PIECE_KIND_COUNT]; ARMY_COUNT],
    pub occupancy_by_army: [u64; ARMY_COUNT],
    pub occupancy_by_team: [u64; TEAM_COUNT],
    pub all_occupancy: u64,
    pub free: u64,
    pub armies: [ArmyState; ARMY_COUNT],
    pub promotion_zones: [u64; ARMY_COUNT],
    /// Throne overlay: stores a piece "under" a king on its own throne.
    /// Indexed by [army][throne_index], where throne_index is 0 or 1.
    /// A king on its own throne can share the square with one allied piece.
    pub throne_overlay: [[Option<OverlayPiece>; 2]; ARMY_COUNT],
    /// Per-square piece metadata (patron for pawns, diagonal system for queens/bishops).
    pub piece_map: HashMap<Square, Piece>,
}

impl Board {
    pub fn new(initial_placements: &[(Army, Piece, u64)]) -> Board {
        Board::with_state(
            initial_placements,
            DEFAULT_ARMY_STATES,
            DEFAULT_PROMOTION_ZONES,
        )
    }

    pub fn with_state(
        initial_placements: &[(Army, Piece, u64)],
        army_states: [ArmyState; ARMY_COUNT],
        promotion_zones: [u64; ARMY_COUNT],
    ) -> Board {
        let mut by_army_kind = [[0u64; PIECE_KIND_COUNT]; ARMY_COUNT];
        let mut piece_map = HashMap::new();

        for (army, piece, bitboard) in initial_placements {
            by_army_kind[army.index()][piece.kind.index()] |= *bitboard;

            // Populate piece_map for each square in the bitboard
            let mut bits = *bitboard;
            while bits != 0 {
                let sq = bits.trailing_zeros() as Square;
                piece_map.insert(sq, *piece);
                bits &= bits - 1; // clear lowest set bit
            }
        }

        let occupancy_by_army = compute_occupancy_by_army(&by_army_kind);
        let occupancy_by_team = compute_occupancy_by_team(&occupancy_by_army);
        let all_occupancy = occupancy_by_team[0] | occupancy_by_team[1];

        Board {
            by_army_kind,
            occupancy_by_army,
            occupancy_by_team,
            all_occupancy,
            free: !all_occupancy,
            armies: army_states,
            promotion_zones,
            throne_overlay: [[None; 2]; ARMY_COUNT],
            piece_map,
        }
    }

    pub fn piece_at(&self, square: Square) -> Option<(Army, PieceKind)> {
        let mask = 1u64 << square;
        for army in Army::ALL {
            for kind in PieceKind::ALL {
                if self.by_army_kind[army.index()][kind.index()] & mask != 0 {
                    return Some((army, kind));
                }
            }
        }
        None
    }

    /// Get full piece metadata (patron, diagonal system) for a piece at a square.
    pub fn get_piece(&self, square: Square) -> Option<&Piece> {
        self.piece_map.get(&square)
    }
}

impl Board {
    pub fn set_frozen(&mut self, army: Army, frozen: bool) {
        self.armies[army.index()].is_frozen = frozen;
    }

    pub fn is_army_frozen(&self, army: Army) -> bool {
        self.armies[army.index()].is_frozen
    }

    pub fn set_controller(&mut self, army: Army, controller: PlayerId) {
        self.armies[army.index()].controller = controller;
    }

    pub fn controller_for(&self, army: Army) -> PlayerId {
        self.armies[army.index()].controller
    }

    pub fn king_square(&self, army: Army) -> Option<Square> {
        let mask = self.by_army_kind[army.index()][PieceKind::King.index()];
        if mask == 0 {
            None
        } else {
            Some(mask.trailing_zeros() as Square)
        }
    }

    pub fn clear_square(&mut self, square: Square) {
        let bit = 1u64 << square;
        for army in Army::ALL {
            for kind in PieceKind::ALL {
                self.by_army_kind[army.index()][kind.index()] &= !bit;
            }
        }
        self.piece_map.remove(&square);
        self.refresh_occupancy();
    }

    pub fn refresh_occupancy(&mut self) {
        self.occupancy_by_army = compute_occupancy_by_army(&self.by_army_kind);
        self.occupancy_by_team = compute_occupancy_by_team(&self.occupancy_by_army);
        self.all_occupancy = self.occupancy_by_team[0] | self.occupancy_by_team[1];
        self.free = !self.all_occupancy;
    }

    /// Place a piece on a square. If you need to preserve patron/diagonal metadata,
    /// use `place_piece_with_metadata` instead.
    pub fn place_piece(&mut self, army: Army, kind: PieceKind, square: Square) {
        let mask = 1u64 << square;
        self.by_army_kind[army.index()][kind.index()] |= mask;
        self.piece_map.insert(square, Piece {
            army,
            kind,
            pawn_type: None,
            diagonal_system: None,
        });
        self.refresh_occupancy();
    }

    /// Place a piece with full metadata (patron, diagonal system).
    pub fn place_piece_with_metadata(&mut self, square: Square, piece: Piece) {
        let mask = 1u64 << square;
        self.by_army_kind[piece.army.index()][piece.kind.index()] |= mask;
        self.piece_map.insert(square, piece);
        self.refresh_occupancy();
    }

    pub fn remove_piece(&mut self, army: Army, kind: PieceKind, square: Square) {
        let mask = 1u64 << square;
        self.by_army_kind[army.index()][kind.index()] &= !mask;
        self.piece_map.remove(&square);
        self.refresh_occupancy();
    }

    /// Demote a piece to a pawn (for privileged pawn promotion demotion rule).
    /// The demoted pawn becomes a "pawn of [kind]" - its patron is set to the demoted type.
    pub fn demote_piece_to_pawn(&mut self, army: Army, kind: PieceKind) -> Option<Square> {
        if kind == PieceKind::Pawn {
            return None;
        }
        let mask = self.by_army_kind[army.index()][kind.index()];
        if mask == 0 {
            return None;
        }
        let square = mask.trailing_zeros() as Square;
        let bit = 1u64 << square;
        self.by_army_kind[army.index()][kind.index()] &= !bit;
        self.by_army_kind[army.index()][PieceKind::Pawn.index()] |= bit;

        // Update piece_map: the demoted piece becomes a pawn with patron = original kind
        self.piece_map.insert(square, Piece {
            army,
            kind: PieceKind::Pawn,
            pawn_type: Some(kind), // patron is what it was demoted from
            diagonal_system: None,
        });

        self.refresh_occupancy();
        Some(square)
    }

    /// Move a piece from one square to another, preserving its metadata.
    pub fn move_piece(&mut self, army: Army, kind: PieceKind, from: Square, to: Square) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;
        self.by_army_kind[army.index()][kind.index()] &= !from_mask;
        self.by_army_kind[army.index()][kind.index()] |= to_mask;

        // Preserve piece metadata when moving
        if let Some(piece) = self.piece_map.remove(&from) {
            self.piece_map.insert(to, piece);
        }

        self.refresh_occupancy();
    }

    pub fn piece_counts(&self, army: Army) -> [u32; PIECE_KIND_COUNT] {
        let mut counts = [0u32; PIECE_KIND_COUNT];
        for kind in PieceKind::ALL {
            counts[kind.index()] = self.by_army_kind[army.index()][kind.index()].count_ones();
        }
        counts
    }

    pub fn ascii_rows(&self) -> Vec<String> {
        let mut rows = Vec::with_capacity(8);
        for rank in (0..8).rev() {
            let mut line = String::new();
            line.push_str(&format!("{} ", rank + 1));
            for file in 0..8 {
                let square = square_index(file, rank);
                let ch = match self.piece_at(square) {
                    Some((army, kind)) => piece_char(army, kind),
                    None => '.',
                };
                line.push(ch);
                line.push(' ');
            }
            rows.push(line.trim_end().to_string());
        }
        rows
    }

    pub fn throne_owner(&self, square: Square) -> Option<Army> {
        for army in Army::ALL {
            if self.armies[army.index()].throne_squares.contains(&square) {
                return Some(army);
            }
        }
        None
    }

    /// Returns the throne index (0 or 1) for a given square within an army's thrones.
    /// Returns None if the square is not one of the army's throne squares.
    pub fn throne_index_for(&self, army: Army, square: Square) -> Option<usize> {
        let thrones = self.armies[army.index()].throne_squares;
        if square == thrones[0] {
            Some(0)
        } else if square == thrones[1] {
            Some(1)
        } else {
            None
        }
    }

    /// Gets the overlay piece at a throne square, if any.
    pub fn get_throne_overlay(&self, army: Army, throne_index: usize) -> Option<OverlayPiece> {
        self.throne_overlay[army.index()][throne_index]
    }

    /// Sets an overlay piece at a throne square.
    pub fn set_throne_overlay(&mut self, army: Army, throne_index: usize, piece: OverlayPiece) {
        self.throne_overlay[army.index()][throne_index] = Some(piece);
    }

    /// Clears the overlay piece at a throne square, returning it if present.
    pub fn clear_throne_overlay(&mut self, army: Army, throne_index: usize) -> Option<OverlayPiece> {
        self.throne_overlay[army.index()][throne_index].take()
    }

    /// Checks if the king of the given army is on one of its own throne squares.
    /// Returns the throne index if so.
    pub fn king_on_own_throne(&self, army: Army) -> Option<usize> {
        let king_square = self.king_square(army)?;
        self.throne_index_for(army, king_square)
    }

    /// Checks if a square is a throne with a king on it (for double-occupancy).
    /// Returns (throne_owner_army, throne_index) if the square is a throne with its own king.
    pub fn is_king_occupied_throne(&self, square: Square) -> Option<(Army, usize)> {
        let throne_army = self.throne_owner(square)?;
        let throne_idx = self.throne_index_for(throne_army, square)?;
        let king_square = self.king_square(throne_army)?;
        if king_square == square {
            Some((throne_army, throne_idx))
        } else {
            None
        }
    }

    /// Returns a bitboard mask of team king-occupied thrones available for double-occupancy.
    /// These are squares where any same-team king sits on their own throne AND no overlay piece exists.
    /// Allied pieces (same team) can target these squares to share the throne with the king.
    /// Note: Kings cannot move to overlay positions (only non-king pieces can).
    pub fn team_king_thrones_for_overlay(&self, moving_army: Army) -> u64 {
        let team = moving_army.team();
        let mut mask = 0u64;

        for army in Army::ALL {
            // Must be same team (includes own army and allied army)
            if army.team() != team {
                continue;
            }

            // Check if this army's king is on their own throne
            if let Some(throne_idx) = self.king_on_own_throne(army) {
                // Only allow if no overlay piece already exists
                if self.throne_overlay[army.index()][throne_idx].is_none() {
                    let throne_square = self.armies[army.index()].throne_squares[throne_idx];
                    mask |= 1u64 << throne_square;
                }
            }
        }

        mask
    }
}

const fn square_index(file: u8, rank: u8) -> Square {
    rank * 8 + file
}

fn piece_char(army: Army, kind: PieceKind) -> char {
    let letter = match kind {
        PieceKind::King => 'K',
        PieceKind::Queen => 'Q',
        PieceKind::Rook => 'R',
        PieceKind::Bishop => 'B',
        PieceKind::Knight => 'N',
        PieceKind::Pawn => 'P',
    };
    match army {
        Army::Blue => letter,
        Army::Black => letter.to_ascii_lowercase(),
        Army::Red => letter,
        Army::Yellow => letter.to_ascii_lowercase(),
    }
}

impl Default for Board {
    fn default() -> Board {
        let initial_placements = [
            (
                Army::Blue,
                Piece {
                    army: Army::Blue,
                    kind: PieceKind::King,
                    pawn_type: None,
                    diagonal_system: None,
                },
                1 << coord(4, 0),
            ),
            (
                Army::Red,
                Piece {
                    army: Army::Red,
                    kind: PieceKind::King,
                    pawn_type: None,
                    diagonal_system: None,
                },
                1 << coord(4, 7),
            ),
            (
                Army::Black,
                Piece {
                    army: Army::Black,
                    kind: PieceKind::King,
                    pawn_type: None,
                    diagonal_system: None,
                },
                1 << coord(0, 4),
            ),
            (
                Army::Yellow,
                Piece {
                    army: Army::Yellow,
                    kind: PieceKind::King,
                    pawn_type: None,
                    diagonal_system: None,
                },
                1 << coord(7, 4),
            ),
        ];
        Board::new(&initial_placements)
    }
}

const fn coord(file: u8, rank: u8) -> u8 {
    rank * 8 + file
}

const DEFAULT_ARMY_STATES: [ArmyState; ARMY_COUNT] = [
    ArmyState::new(Army::Blue, [coord(3, 0), coord(4, 0)], PlayerId::PLAYER_ONE),
    ArmyState::new(
        Army::Black,
        [coord(0, 3), coord(0, 4)],
        PlayerId::PLAYER_ONE,
    ),
    ArmyState::new(Army::Red, [coord(3, 7), coord(4, 7)], PlayerId::PLAYER_TWO),
    ArmyState::new(
        Army::Yellow,
        [coord(7, 3), coord(7, 4)],
        PlayerId::PLAYER_TWO,
    ),
];

pub const DEFAULT_PROMOTION_ZONES: [u64; ARMY_COUNT] = [
    MASK_RANK_8, // Blue marches north
    MASK_FILE_H, // Black moves east
    MASK_RANK_1, // Red marches south
    MASK_FILE_A, // Yellow moves west
];

fn compute_occupancy_by_army(
    by_army_kind: &[[u64; PIECE_KIND_COUNT]; ARMY_COUNT],
) -> [u64; ARMY_COUNT] {
    let mut occupancy_by_army = [0u64; ARMY_COUNT];
    for army in Army::ALL {
        let mut bits = 0u64;
        for kind in PieceKind::ALL {
            bits |= by_army_kind[army.index()][kind.index()];
        }
        occupancy_by_army[army.index()] = bits;
    }
    occupancy_by_army
}

fn compute_occupancy_by_team(occupancy_by_army: &[u64; ARMY_COUNT]) -> [u64; TEAM_COUNT] {
    let mut occupancy_by_team = [0u64; TEAM_COUNT];
    for army in Army::ALL {
        let team_idx = army.team().index();
        occupancy_by_team[team_idx] |= occupancy_by_army[army.index()];
    }
    occupancy_by_team
}

pub const ARIES_DIAGONALS: u64 = 0x55AA55AA55AA55AA;
pub const CANCER_DIAGONALS: u64 = 0xAA55AA55AA55AA55;

/// Determine the diagonal system (Aries or Cancer) for a given square.
pub fn diagonal_system_for_square(square: Square) -> DiagonalSystem {
    if (ARIES_DIAGONALS >> square) & 1 != 0 {
        DiagonalSystem::Aries
    } else {
        DiagonalSystem::Cancer
    }
}

pub const MASK_RANK_1: u64 =
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
pub const MASK_RANK_2: u64 =
    0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
pub const MASK_RANK_3: u64 =
    0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000;
pub const MASK_RANK_4: u64 =
    0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
pub const MASK_RANK_5: u64 =
    0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
pub const MASK_RANK_6: u64 =
    0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;
pub const MASK_RANK_7: u64 =
    0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
pub const MASK_RANK_8: u64 =
    0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
pub const MASK_FILE_A: u64 =
    0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const MASK_FILE_B: u64 =
    0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
pub const MASK_FILE_C: u64 =
    0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100;
pub const MASK_FILE_D: u64 =
    0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000;
pub const MASK_FILE_E: u64 =
    0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000;
pub const MASK_FILE_F: u64 =
    0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000;
pub const MASK_FILE_G: u64 =
    0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
pub const MASK_FILE_H: u64 =
    0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
