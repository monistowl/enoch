use crate::engine::types::{Army, Piece, PieceKind, Team};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub by_army_kind: [[u64; 6]; 4], // [Army][PieceKind]
    pub occupancy_by_army: [u64; 4],
    pub occupancy_by_team: [u64; 2],
    pub all_occupancy: u64,
    pub free: u64,
}

impl Board {
    pub fn new(initial_placements: &[(Army, Piece, u64)]) -> Board {
        let mut by_army_kind = [[0u64; 6]; 4];
        for (army, piece, bitboard) in initial_placements {
            by_army_kind[*army as usize][piece.kind as usize] = *bitboard;
        }

        let mut occupancy_by_army = [0u64; 4];
        for army in [Army::Blue, Army::Black, Army::Red, Army::Yellow] {
            let army_idx = army as usize;
            for kind in 0..6 {
                occupancy_by_army[army_idx] |= by_army_kind[army_idx][kind];
            }
        }

        let occupancy_by_team = [
            occupancy_by_army[Army::Blue as usize] | occupancy_by_army[Army::Black as usize],
            occupancy_by_army[Army::Red as usize] | occupancy_by_army[Army::Yellow as usize],
        ];

        let all_occupancy = occupancy_by_team[0] | occupancy_by_team[1];

        Board {
            by_army_kind,
            occupancy_by_army,
            occupancy_by_team,
            all_occupancy,
            free: !all_occupancy,
        }
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
                1 << 4,
            ),
            (
                Army::Red,
                Piece {
                    army: Army::Red,
                    kind: PieceKind::King,
                    pawn_type: None,
                },
                1 << (8 * 7 + 4),
            ),
            (
                Army::Black,
                Piece {
                    army: Army::Black,
                    kind: PieceKind::King,
                    pawn_type: None,
                },
                1 << (4 * 8),
            ),
            (
                Army::Yellow,
                Piece {
                    army: Army::Yellow,
                    kind: PieceKind::King,
                    pawn_type: None,
                },
                1 << (4 * 8 + 7),
            ),
        ];
        Board::new(&initial_placements)
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