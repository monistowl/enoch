use crate::engine::types::{
    Army, Piece, PieceKind, PlayerId, Square, Team, ARMY_COUNT, PIECE_KIND_COUNT, TEAM_COUNT,
};

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

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub by_army_kind: [[u64; PIECE_KIND_COUNT]; ARMY_COUNT],
    pub occupancy_by_army: [u64; ARMY_COUNT],
    pub occupancy_by_team: [u64; TEAM_COUNT],
    pub all_occupancy: u64,
    pub free: u64,
    pub armies: [ArmyState; ARMY_COUNT],
    pub promotion_zones: [u64; ARMY_COUNT],
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
        for (army, piece, bitboard) in initial_placements {
            by_army_kind[army.index()][piece.kind.index()] = *bitboard;
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
        self.refresh_occupancy();
    }

    pub fn refresh_occupancy(&mut self) {
        self.occupancy_by_army = compute_occupancy_by_army(&self.by_army_kind);
        self.occupancy_by_team = compute_occupancy_by_team(&self.occupancy_by_army);
        self.all_occupancy = self.occupancy_by_team[0] | self.occupancy_by_team[1];
        self.free = !self.all_occupancy;
    }

    pub fn place_piece(&mut self, army: Army, kind: PieceKind, square: Square) {
        let mask = 1u64 << square;
        self.by_army_kind[army.index()][kind.index()] |= mask;
        self.refresh_occupancy();
    }

    pub fn remove_piece(&mut self, army: Army, kind: PieceKind, square: Square) {
        let mask = 1u64 << square;
        self.by_army_kind[army.index()][kind.index()] &= !mask;
        self.refresh_occupancy();
    }

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
        self.refresh_occupancy();
        Some(square)
    }

    pub fn move_piece(&mut self, army: Army, kind: PieceKind, from: Square, to: Square) {
        let from_mask = 1u64 << from;
        let to_mask = 1u64 << to;
        self.by_army_kind[army.index()][kind.index()] &= !from_mask;
        self.by_army_kind[army.index()][kind.index()] |= to_mask;
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

    pub fn all_pieces_for_army(&self, army: Army) -> impl Iterator<Item = (Square, PieceKind)> + '_ {
        let mut pieces = Vec::new();
        for kind in PieceKind::ALL {
            let mut bitboard = self.by_army_kind[army.index()][kind.index()];
            while bitboard != 0 {
                let square = bitboard.trailing_zeros() as Square;
                pieces.push((square, kind));
                bitboard &= bitboard - 1;
            }
        }
        pieces.into_iter()
    }

    pub fn throne_owner(&self, square: Square) -> Option<Army> {
        for army in Army::ALL {
            if self.armies[army.index()].throne_squares.contains(&square) {
                return Some(army);
            }
        }
        None
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
                },
                1 << coord(4, 0),
            ),
            (
                Army::Red,
                Piece {
                    army: Army::Red,
                    kind: PieceKind::King,
                    pawn_type: None,
                },
                1 << coord(4, 7),
            ),
            (
                Army::Black,
                Piece {
                    army: Army::Black,
                    kind: PieceKind::King,
                    pawn_type: None,
                },
                1 << coord(0, 4),
            ),
            (
                Army::Yellow,
                Piece {
                    army: Army::Yellow,
                    kind: PieceKind::King,
                    pawn_type: None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagonalSystem {
    Aries,
    Cancer,
}

pub fn diagonal_system(square: Square) -> DiagonalSystem {
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
