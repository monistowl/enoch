use crate::engine::board::{diagonal_system_for_square, ArmyState, Board, DEFAULT_PROMOTION_ZONES};
use crate::engine::types::{Army, DiagonalSystem, Piece, PieceKind, PlayerId, Square, ARMY_COUNT};

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
        // First pass: collect major pieces by column to determine pawn patrons
        let mut major_pieces_by_column: std::collections::HashMap<(Army, u8), PieceKind> =
            std::collections::HashMap::new();

        for &(army, kind, bitboard) in self.placements {
            if kind != PieceKind::Pawn {
                let mut mask = bitboard;
                while mask != 0 {
                    let square = mask.trailing_zeros() as Square;
                    let file = square % 8;
                    // Store the major piece for this army+column
                    major_pieces_by_column.insert((army, file as u8), kind);
                    mask &= mask - 1;
                }
            }
        }

        // Second pass: create pieces with patron and diagonal metadata
        let mut pieces = Vec::new();
        for &(army, kind, bitboard) in self.placements {
            let mut mask = bitboard;
            while mask != 0 {
                let square = mask.trailing_zeros() as Square;
                let file = square % 8;

                // Determine pawn patron based on the major piece in the same column
                let pawn_type = if kind == PieceKind::Pawn {
                    major_pieces_by_column.get(&(army, file as u8)).copied()
                } else {
                    None
                };

                // Determine diagonal system for queens and bishops based on starting square
                let diagonal_system = match kind {
                    PieceKind::Queen | PieceKind::Bishop => {
                        Some(diagonal_system_for_square(square))
                    }
                    _ => None,
                };

                pieces.push((
                    army,
                    Piece {
                        army,
                        kind,
                        pawn_type,
                        diagonal_system,
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

// --- Placements Logic ---

// Helper to generate a full rank of pieces based on the 4-piece sequence.
// Sequence: [Square 1 Pair, Square 2 Pair, Square 3 Pair, Square 4 Pair]
// Layout on Rank:
// Index 0 (A): Piece 4
// Index 1 (B): Piece 3
// Index 2 (C): Piece 2
// Index 3 (D): Partner (Piece 1)
// Index 4 (E): King (Piece 1)
// Index 5 (F): Piece 2
// Index 6 (G): Piece 3
// Index 7 (H): Piece 4
//
// Note: King is always placed at Index 4 (File E). Partner is at Index 3 (File D).
// This matches standard chess (King on E).

// Piece sequence for the 8 Settings.
// Group 1: Fire and Earth Boards
// Earth: King/Rook, Bishop, Queen, Knight
// Air: King/Bishop, Rook, Knight, Queen
// Water: King/Queen, Knight, Rook, Bishop
// Fire: King/Knight, Queen, Bishop, Rook

const SETTING_EARTH_G1: [PieceKind; 4] = [PieceKind::Rook, PieceKind::Bishop, PieceKind::Queen, PieceKind::Knight];
const SETTING_AIR_G1: [PieceKind; 4] = [PieceKind::Bishop, PieceKind::Rook, PieceKind::Knight, PieceKind::Queen];
const SETTING_WATER_G1: [PieceKind; 4] = [PieceKind::Queen, PieceKind::Knight, PieceKind::Rook, PieceKind::Bishop];
const SETTING_FIRE_G1: [PieceKind; 4] = [PieceKind::Knight, PieceKind::Queen, PieceKind::Bishop, PieceKind::Rook];

// Group 2: Air and Water Boards
// Earth: King/Rook, Knight, Queen, Bishop
// Air: King/Bishop, Queen, Knight, Rook
// Water: King/Queen, Bishop, Rook, Knight
// Fire: King/Knight, Rook, Bishop, Queen

const SETTING_EARTH_G2: [PieceKind; 4] = [PieceKind::Rook, PieceKind::Knight, PieceKind::Queen, PieceKind::Bishop];
const SETTING_AIR_G2: [PieceKind; 4] = [PieceKind::Bishop, PieceKind::Queen, PieceKind::Knight, PieceKind::Rook];
const SETTING_WATER_G2: [PieceKind; 4] = [PieceKind::Queen, PieceKind::Bishop, PieceKind::Rook, PieceKind::Knight];
const SETTING_FIRE_G2: [PieceKind; 4] = [PieceKind::Knight, PieceKind::Rook, PieceKind::Bishop, PieceKind::Queen];

// We need to define static slices for `ArraySpec`.
// Since we can't easily generate them at compile time with logic, we'll define them manually or use macros if possible.
// But `&'static` requires const or static.
// We'll define the placements for each of the 16 combinations.

// Layout Bands:
// Blue: Rank 1 (Main), Rank 2 (Pawn)
// Black: Rank 3 (Main), Rank 4 (Pawn)
// Yellow: Rank 5 (Main), Rank 6 (Pawn)
// Red: Rank 8 (Main), Rank 7 (Pawn)

// Helper macro to define placements
macro_rules! define_placements {
    ($name:ident, $blue_s:expr, $black_s:expr, $yellow_s:expr, $red_s:expr) => {
        pub const $name: &[(Army, PieceKind, u64)] = &[
            // Blue (Rank 1)
            (Army::Blue, $blue_s[3], 1 << 0), // A
            (Army::Blue, $blue_s[2], 1 << 1), // B
            (Army::Blue, $blue_s[1], 1 << 2), // C
            (Army::Blue, $blue_s[0], 1 << 3), // D (Partner)
            (Army::Blue, PieceKind::King, 1 << 4), // E (King)
            (Army::Blue, $blue_s[1], 1 << 5), // F
            (Army::Blue, $blue_s[2], 1 << 6), // G
            (Army::Blue, $blue_s[3], 1 << 7), // H
            (Army::Blue, PieceKind::Pawn, MASK_RANK_2),

            // Black (Rank 3)
            (Army::Black, $black_s[3], 1 << 16), // A
            (Army::Black, $black_s[2], 1 << 17), // B
            (Army::Black, $black_s[1], 1 << 18), // C
            (Army::Black, $black_s[0], 1 << 19), // D
            (Army::Black, PieceKind::King, 1 << 20), // E
            (Army::Black, $black_s[1], 1 << 21), // F
            (Army::Black, $black_s[2], 1 << 22), // G
            (Army::Black, $black_s[3], 1 << 23), // H
            (Army::Black, PieceKind::Pawn, MASK_RANK_4),

            // Yellow (Rank 5)
            (Army::Yellow, $yellow_s[3], 1 << 32), // A
            (Army::Yellow, $yellow_s[2], 1 << 33), // B
            (Army::Yellow, $yellow_s[1], 1 << 34), // C
            (Army::Yellow, $yellow_s[0], 1 << 35), // D
            (Army::Yellow, PieceKind::King, 1 << 36), // E
            (Army::Yellow, $yellow_s[1], 1 << 37), // F
            (Army::Yellow, $yellow_s[2], 1 << 38), // G
            (Army::Yellow, $yellow_s[3], 1 << 39), // H
            (Army::Yellow, PieceKind::Pawn, MASK_RANK_6),

            // Red (Rank 8)
            (Army::Red, $red_s[3], 1 << 56), // A
            (Army::Red, $red_s[2], 1 << 57), // B
            (Army::Red, $red_s[1], 1 << 58), // C
            (Army::Red, $red_s[0], 1 << 59), // D
            (Army::Red, PieceKind::King, 1 << 60), // E
            (Army::Red, $red_s[1], 1 << 61), // F
            (Army::Red, $red_s[2], 1 << 62), // G
            (Army::Red, $red_s[3], 1 << 63), // H
            (Army::Red, PieceKind::Pawn, MASK_RANK_7),
        ];
    };
}

// Masks needed for macro
use crate::engine::board::{MASK_RANK_2, MASK_RANK_4, MASK_RANK_6, MASK_RANK_7};

// Placements for Group 1 Boards (Fire, Earth)
// All armies use the SAME setting in a given array variant.
define_placements!(PLACEMENTS_EARTH_G1, SETTING_EARTH_G1, SETTING_EARTH_G1, SETTING_EARTH_G1, SETTING_EARTH_G1);
define_placements!(PLACEMENTS_AIR_G1, SETTING_AIR_G1, SETTING_AIR_G1, SETTING_AIR_G1, SETTING_AIR_G1);
define_placements!(PLACEMENTS_WATER_G1, SETTING_WATER_G1, SETTING_WATER_G1, SETTING_WATER_G1, SETTING_WATER_G1);
define_placements!(PLACEMENTS_FIRE_G1, SETTING_FIRE_G1, SETTING_FIRE_G1, SETTING_FIRE_G1, SETTING_FIRE_G1);

// Placements for Group 2 Boards (Air, Water)
define_placements!(PLACEMENTS_EARTH_G2, SETTING_EARTH_G2, SETTING_EARTH_G2, SETTING_EARTH_G2, SETTING_EARTH_G2);
define_placements!(PLACEMENTS_AIR_G2, SETTING_AIR_G2, SETTING_AIR_G2, SETTING_AIR_G2, SETTING_AIR_G2);
define_placements!(PLACEMENTS_WATER_G2, SETTING_WATER_G2, SETTING_WATER_G2, SETTING_WATER_G2, SETTING_WATER_G2);
define_placements!(PLACEMENTS_FIRE_G2, SETTING_FIRE_G2, SETTING_FIRE_G2, SETTING_FIRE_G2, SETTING_FIRE_G2);

// --- Arrays ---

// Controller Maps and Thrones for Boards
// Fire Board: Blue S, Red N, Black W (Rank 3), Yellow E (Rank 5)
// Note: "Black W" and "Yellow E" labels refer to typical position, but here they are bands.
// Turn Order: Blue -> Red -> Black -> Yellow
const CONTROLLERS_FIRE: [PlayerId; 4] = [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO]; // Blue/Red P1, Black/Yellow P2?
// Wait, default was: Blue P1, Black P1, Red P2, Yellow P2.
// Let's stick to default controller map for consistency unless specified.
// Prototype used: Blue P1, Red P1, Black P2, Yellow P2.
// But `TABLET_OF_FIRE_PROTOTYPE` in old file had: [Blue, Red, Black, Yellow] order.
// Controller map: [P1, P1, P2, P2].
// So Blue/Red = P1. Black/Yellow = P2.
// Let's keep this.

// Thrones are FIXED by the Band layout?
// Blue (Rank 1): D1, E1.
// Black (Rank 3): D3, E3.
// Yellow (Rank 5): D5, E5.
// Red (Rank 8): D8, E8.
const THRONES_BANDS: [[Square; 2]; ARMY_COUNT] = [
    [square(3, 0), square(4, 0)], // Blue D1, E1
    [square(3, 2), square(4, 2)], // Black D3, E3 (Rank 3)
    [square(3, 7), square(4, 7)], // Red D8, E8 (Rank 8) - index 2 in Army::ALL is RED
    [square(3, 4), square(4, 4)], // Yellow D5, E5 (Rank 5) - index 3 in Army::ALL is YELLOW
];
// Army::ALL = [Blue, Black, Red, Yellow]
// Index 0: Blue.
// Index 1: Black.
// Index 2: Red.
// Index 3: Yellow.

// Wait, `TABLET_OF_FIRE_PROTOTYPE` had Red at Index 1 in `turn_order`, but `Army::ALL` is `[Blue, Black, Red, Yellow]`.
// `ArraySpec::army_states` iterates `Army::ALL`.
// So `throne_squares` must be in `Army::ALL` order.
// Blue, Black, Red, Yellow.

// Board 1: Fire Board (Group 1 Placements)
// Turn Order: Blue, Red, Black, Yellow.
pub const TABLET_OF_FIRE_EARTH: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Earth Setting)",
    description: "Fire Board with Earth Setting. South-North Bands layout.",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO, PlayerId::PLAYER_ONE], // Match Teams?
    // Team Air: Blue + Black. Team Earth: Red + Yellow.
    // Typically P1 = Air, P2 = Earth.
    // Blue (Air) -> P1.
    // Black (Air) -> P1.
    // Red (Earth) -> P2.
    // Yellow (Earth) -> P2.
    // So map should be: Blue=P1, Black=P1, Red=P2, Yellow=P2.
    // ArraySpec `controller_map` is indexed by `Army::ALL` (Blue, Black, Red, Yellow).
    // So [P1, P1, P2, P2].
    // Wait, `TABLET_OF_FIRE_PROTOTYPE` had `controller_map` based on `turn_order`?
    // No, `ArraySpec` struct doc says `controller_map: [PlayerId; ARMY_COUNT]`.
    // `army_states()` uses `self.controller_map[idx]` where idx is `Army::ALL` index.
    // So `controller_map` must be in `Army::ALL` order.
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G1,
};

pub const TABLET_OF_FIRE_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Air Setting)",
    description: "Fire Board with Air Setting.",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G1,
};

pub const TABLET_OF_FIRE_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Water Setting)",
    description: "Fire Board with Water Setting.",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G1,
};

pub const TABLET_OF_FIRE_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Fire Setting)",
    description: "Fire Board with Fire Setting. (Previously Prototype).",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_FIRE_G1,
};

// Board 2: Earth Board (Group 1 Placements)
// Turn Order: Yellow, Blue, Red, Black.
pub const TABLET_OF_EARTH_EARTH: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Earth Setting)",
    description: "Earth Board with Earth Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G1,
};
// Note: Earth uses Group 1 placements (Same as Fire).

pub const TABLET_OF_EARTH_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Air Setting)",
    description: "Earth Board with Air Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G1,
};

pub const TABLET_OF_EARTH_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Water Setting)",
    description: "Earth Board with Water Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G1,
};

pub const TABLET_OF_EARTH_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Fire Setting)",
    description: "Earth Board with Fire Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_FIRE_G1,
};


// Board 3: Air Board (Group 2 Placements)
// Turn Order: Red, Yellow, Black, Blue.
pub const TABLET_OF_AIR_EARTH: ArraySpec = ArraySpec {
    name: "Tablet of Air (Earth Setting)",
    description: "Air Board with Earth Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G2,
};

pub const TABLET_OF_AIR_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Air (Air Setting)",
    description: "Air Board with Air Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G2,
};

pub const TABLET_OF_AIR_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Air (Water Setting)",
    description: "Air Board with Water Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G2,
};

pub const TABLET_OF_AIR_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Air (Fire Setting)",
    description: "Air Board with Fire Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_FIRE_G2,
};


// Board 4: Water Board (Group 2 Placements)
// Turn Order: Blue, Black, Yellow, Red.
pub const TABLET_OF_WATER_EARTH: ArraySpec = ArraySpec {
    name: "Tablet of Water (Earth Setting)",
    description: "Water Board with Earth Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G2,
};

pub const TABLET_OF_WATER_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Water (Air Setting)",
    description: "Water Board with Air Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G2,
};

pub const TABLET_OF_WATER_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Water (Water Setting)",
    description: "Water Board with Water Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G2,
};

pub const TABLET_OF_WATER_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Water (Fire Setting)",
    description: "Water Board with Fire Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_BANDS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_FIRE_G2,
};

// All Arrays
pub const ALL_ARRAYS: [&ArraySpec; 16] = [
    &TABLET_OF_FIRE_EARTH, &TABLET_OF_FIRE_AIR, &TABLET_OF_FIRE_WATER, &TABLET_OF_FIRE_FIRE,
    &TABLET_OF_EARTH_EARTH, &TABLET_OF_EARTH_AIR, &TABLET_OF_EARTH_WATER, &TABLET_OF_EARTH_FIRE,
    &TABLET_OF_AIR_EARTH, &TABLET_OF_AIR_AIR, &TABLET_OF_AIR_WATER, &TABLET_OF_AIR_FIRE,
    &TABLET_OF_WATER_EARTH, &TABLET_OF_WATER_AIR, &TABLET_OF_WATER_WATER, &TABLET_OF_WATER_FIRE,
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

// Default to Fire of Fire (Traditional?)
pub fn default_array() -> &'static ArraySpec {
    &TABLET_OF_FIRE_FIRE
}