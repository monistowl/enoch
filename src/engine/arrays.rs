use crate::engine::board::{ArmyState, Board, DEFAULT_PROMOTION_ZONES};
use crate::engine::types::{Army, Piece, PieceKind, PlayerId, Square, ARMY_COUNT};

#[derive(Debug, Clone)]
pub struct ArraySpec {
    pub name: &'static str,
    pub description: &'static str,
    pub turn_order: [Army; ARMY_COUNT],
    pub controller_map: [PlayerId; ARMY_COUNT],
    pub throne_squares: [[Square; 2]; ARMY_COUNT],
    pub promotion_zones: [u64; ARMY_COUNT],
    pub placements: &'static [(Army, PieceKind, u64)],
}

impl ArraySpec {
    pub fn board(&self) -> Board {
        let placements = self.expand_placements();
        Board::with_state(&placements, self.army_states(), self.promotion_zones)
    }

    fn expand_placements(&self) -> Vec<(Army, Piece, u64)> {
        let mut pieces = Vec::new();
        for &(army, kind, bitboard) in self.placements {
            let mut mask = bitboard;
            while mask != 0 {
                let square = mask.trailing_zeros() as Square;
                pieces.push((
                    army,
                    Piece {
                        army,
                        kind,
                        pawn_type: None,
                    },
                    1u64 << square,
                ));
                mask &= mask - 1;
            }
        }
        pieces
    }

    pub fn army_states(&self) -> [ArmyState; ARMY_COUNT] {
        let mut states =
            [ArmyState::new(Army::Blue, self.throne_squares[0], self.controller_map[0]);
                ARMY_COUNT];
        for (idx, &army) in Army::ALL.iter().enumerate() {
            states[idx] = ArmyState::new(army, self.throne_squares[idx], self.controller_map[idx]);
        }
        states
    }
}

const fn square(file: u8, rank: u8) -> Square {
    rank * 8 + file
}

pub const TABLET_OF_FIRE_PLACEMENTS: &[(Army, PieceKind, u64)] = &[
    (Army::Blue, PieceKind::Rook, 1 << 0),
    (Army::Blue, PieceKind::Knight, 1 << 1),
    (Army::Blue, PieceKind::Bishop, 1 << 2),
    (Army::Blue, PieceKind::Queen, 1 << 3),
    (Army::Blue, PieceKind::King, 1 << 4),
    (Army::Blue, PieceKind::Bishop, 1 << 5),
    (Army::Blue, PieceKind::Knight, 1 << 6),
    (Army::Blue, PieceKind::Rook, 1 << 7),
    (Army::Blue, PieceKind::Pawn, 0xFF00),
    (Army::Red, PieceKind::Rook, 1 << 56),
    (Army::Red, PieceKind::Knight, 1 << 57),
    (Army::Red, PieceKind::Bishop, 1 << 58),
    (Army::Red, PieceKind::Queen, 1 << 59),
    (Army::Red, PieceKind::King, 1 << 60),
    (Army::Red, PieceKind::Bishop, 1 << 61),
    (Army::Red, PieceKind::Knight, 1 << 62),
    (Army::Red, PieceKind::Rook, 1 << 63),
    (Army::Red, PieceKind::Pawn, 0xFF000000000000),
    (Army::Black, PieceKind::Rook, 1 << 24),
    (Army::Black, PieceKind::Knight, 1 << 16),
    (Army::Black, PieceKind::Bishop, 1 << 8),
    (Army::Black, PieceKind::Queen, 1 << 0),
    (Army::Black, PieceKind::King, 1 << 32),
    (Army::Black, PieceKind::Bishop, 1 << 40),
    (Army::Black, PieceKind::Knight, 1 << 48),
    (Army::Black, PieceKind::Rook, 1 << 56),
    (Army::Black, PieceKind::Pawn, 0x101010101010101),
    (Army::Yellow, PieceKind::Rook, 1 << 31),
    (Army::Yellow, PieceKind::Knight, 1 << 23),
    (Army::Yellow, PieceKind::Bishop, 1 << 15),
    (Army::Yellow, PieceKind::Queen, 1 << 7),
    (Army::Yellow, PieceKind::King, 1 << 39),
    (Army::Yellow, PieceKind::Bishop, 1 << 47),
    (Army::Yellow, PieceKind::Knight, 1 << 55),
    (Army::Yellow, PieceKind::Rook, 1 << 63),
    (Army::Yellow, PieceKind::Pawn, 0x8080808080808080),
];

pub const TABLET_OF_FIRE_PROTOTYPE: ArraySpec = ArraySpec {
    name: "Tablet of Fire (prototype)",
    description: "A developer-facing transcription of the Zalewski Tablet of Fire array.",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [
        PlayerId::PLAYER_ONE,
        PlayerId::PLAYER_ONE,
        PlayerId::PLAYER_TWO,
        PlayerId::PLAYER_TWO,
    ],
    throne_squares: [
        [square(3, 0), square(4, 0)],
        [square(0, 3), square(0, 4)],
        [square(3, 7), square(4, 7)],
        [square(7, 3), square(7, 4)],
    ],
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: TABLET_OF_FIRE_PLACEMENTS,
};

pub const PLACEHOLDER_PLACEMENTS: &[(Army, PieceKind, u64)] = &[];

pub const TABLET_OF_WATER_PLACEHOLDER: ArraySpec = ArraySpec {
    name: "Tablet of Water (placeholder)",
    description: "Turn order: [Blue, Black, Yellow, Red]. Actual diagram to follow.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [
        PlayerId::PLAYER_ONE,
        PlayerId::PLAYER_ONE,
        PlayerId::PLAYER_TWO,
        PlayerId::PLAYER_TWO,
    ],
    throne_squares: [
        [square(3, 0), square(4, 0)],
        [square(0, 3), square(0, 4)],
        [square(7, 3), square(7, 4)],
        [square(3, 7), square(4, 7)],
    ],
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEHOLDER_PLACEMENTS,
};

pub const TABLET_OF_AIR_PLACEHOLDER: ArraySpec = ArraySpec {
    name: "Tablet of Air (placeholder)",
    description: "Rotated turn order (Red → Yellow → Black → Blue). Piece layout TBD.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [
        PlayerId::PLAYER_TWO,
        PlayerId::PLAYER_TWO,
        PlayerId::PLAYER_ONE,
        PlayerId::PLAYER_ONE,
    ],
    throne_squares: [
        [square(3, 7), square(4, 7)],
        [square(7, 3), square(7, 4)],
        [square(0, 3), square(0, 4)],
        [square(3, 0), square(4, 0)],
    ],
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEHOLDER_PLACEMENTS,
};

pub const TABLET_OF_EARTH_PLACEHOLDER: ArraySpec = ArraySpec {
    name: "Tablet of Earth (placeholder)",
    description: "Counter-clockwise order (Yellow → Blue → Red → Black); layout pending.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [
        PlayerId::PLAYER_TWO,
        PlayerId::PLAYER_ONE,
        PlayerId::PLAYER_TWO,
        PlayerId::PLAYER_ONE,
    ],
    throne_squares: [
        [square(7, 3), square(7, 4)],
        [square(3, 0), square(4, 0)],
        [square(3, 7), square(4, 7)],
        [square(0, 3), square(0, 4)],
    ],
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEHOLDER_PLACEMENTS,
};

pub const ALL_ARRAYS: [&ArraySpec; 4] = [
    &TABLET_OF_FIRE_PROTOTYPE,
    &TABLET_OF_WATER_PLACEHOLDER,
    &TABLET_OF_AIR_PLACEHOLDER,
    &TABLET_OF_EARTH_PLACEHOLDER,
];

pub fn available_arrays() -> &'static [&'static ArraySpec] {
    &ALL_ARRAYS
}

pub fn find_array_by_name(name: &str) -> Option<&'static ArraySpec> {
    let lookup = ALL_ARRAYS
        .iter()
        .find(|spec| spec.name.eq_ignore_ascii_case(name));
    lookup.cloned()
}

pub fn default_array() -> &'static ArraySpec {
    &TABLET_OF_FIRE_PROTOTYPE
}
