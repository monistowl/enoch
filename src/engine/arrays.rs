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

// =============================================================================
// Cross/Edge Layout (Correct Enochian layout)
// =============================================================================
//
// The board has armies at the four edges in a cross pattern:
//
// - Blue: South edge (files a-h, ranks 1-2), pawns march north
// - Black: North edge (files a-h, ranks 7-8), pawns march south
// - Red: East edge (files g-h, ranks 1-8), pawns march west
// - Yellow: West edge (files a-b, ranks 1-8), pawns march east
//
// Corner squares are shared:
// - a1, b1: Blue and Yellow overlap zone
// - a8, b8: Black and Yellow overlap zone
// - g1, h1: Blue and Red overlap zone
// - h7, h8: Black and Red overlap zone
//
// In the cross layout, we place pieces avoiding corner conflicts.
// Blue and Black occupy the inner 6 files (c-f) on their ranks.
// Red and Yellow occupy the inner 6 ranks (3-6) on their files.
// The corners are "empty" at start - no piece placement conflicts.

// Import masks
use crate::engine::board::{
    MASK_FILE_A, MASK_FILE_B, MASK_FILE_G, MASK_FILE_H,
    MASK_RANK_1, MASK_RANK_2, MASK_RANK_7, MASK_RANK_8,
};

// Pawn masks for cross layout (excluding corner squares)
// Blue pawns: rank 2, files c-f (4 pawns - excluding corners)
const MASK_BLUE_PAWNS: u64 = 0b00111100 << 8;  // c2, d2, e2, f2
// Black pawns: rank 7, files c-f (4 pawns - excluding corners)
const MASK_BLACK_PAWNS: u64 = 0b00111100u64 << 48; // c7, d7, e7, f7
// Yellow pawns: file b, ranks 3-6 (4 pawns - excluding corners)
const MASK_YELLOW_PAWNS: u64 = (1u64 << 17) | (1u64 << 25) | (1u64 << 33) | (1u64 << 41); // b3, b4, b5, b6
// Red pawns: file g, ranks 3-6 (4 pawns - excluding corners)
const MASK_RED_PAWNS: u64 = (1u64 << 22) | (1u64 << 30) | (1u64 << 38) | (1u64 << 46); // g3, g4, g5, g6

// Helper macro for cross layout placements
// Blue: Rank 1, files c-f for main pieces (King on e1, partner on d1)
// Black: Rank 8, files c-f for main pieces (King on e8, partner on d8)
// Yellow: File a, ranks 3-6 for main pieces (King on a5, partner on a4)
// Red: File h, ranks 3-6 for main pieces (King on h4, partner on h5)
macro_rules! define_cross_placements {
    ($name:ident, $setting:expr) => {
        pub const $name: &[(Army, PieceKind, u64)] = &[
            // Blue (South edge, Rank 1, files c-f)
            // Layout: c1=piece[1], d1=piece[0]/partner, e1=King, f1=piece[1]
            (Army::Blue, $setting[1], 1 << 2),  // c1
            (Army::Blue, $setting[0], 1 << 3),  // d1 (Partner)
            (Army::Blue, PieceKind::King, 1 << 4), // e1 (King)
            (Army::Blue, $setting[1], 1 << 5),  // f1
            (Army::Blue, PieceKind::Pawn, MASK_BLUE_PAWNS),

            // Black (North edge, Rank 8, files c-f)
            // Mirrored layout: c8=piece[1], d8=piece[0], e8=King, f8=piece[1]
            (Army::Black, $setting[1], 1 << 58), // c8
            (Army::Black, $setting[0], 1 << 59), // d8 (Partner)
            (Army::Black, PieceKind::King, 1 << 60), // e8 (King)
            (Army::Black, $setting[1], 1 << 61), // f8
            (Army::Black, PieceKind::Pawn, MASK_BLACK_PAWNS),

            // Yellow (West edge, File a, ranks 3-6)
            // Layout: a3=piece[1], a4=piece[0]/partner, a5=King, a6=piece[1]
            (Army::Yellow, $setting[1], 1 << 16), // a3
            (Army::Yellow, $setting[0], 1 << 24), // a4 (Partner)
            (Army::Yellow, PieceKind::King, 1 << 32), // a5 (King)
            (Army::Yellow, $setting[1], 1 << 40), // a6
            (Army::Yellow, PieceKind::Pawn, MASK_YELLOW_PAWNS),

            // Red (East edge, File h, ranks 3-6)
            // Layout: h3=piece[1], h4=piece[0]/partner, h5=King, h6=piece[1]
            (Army::Red, $setting[1], 1 << 23),  // h3
            (Army::Red, $setting[0], 1 << 31),  // h4 (Partner)
            (Army::Red, PieceKind::King, 1 << 39), // h5 (King)
            (Army::Red, $setting[1], 1 << 47),  // h6
            (Army::Red, PieceKind::Pawn, MASK_RED_PAWNS),
        ];
    };
}

// Cross layout placements for each setting
define_cross_placements!(PLACEMENTS_EARTH_G1, SETTING_EARTH_G1);
define_cross_placements!(PLACEMENTS_AIR_G1, SETTING_AIR_G1);
define_cross_placements!(PLACEMENTS_WATER_G1, SETTING_WATER_G1);
define_cross_placements!(PLACEMENTS_FIRE_G1, SETTING_FIRE_G1);
define_cross_placements!(PLACEMENTS_EARTH_G2, SETTING_EARTH_G2);
define_cross_placements!(PLACEMENTS_AIR_G2, SETTING_AIR_G2);
define_cross_placements!(PLACEMENTS_WATER_G2, SETTING_WATER_G2);
define_cross_placements!(PLACEMENTS_FIRE_G2, SETTING_FIRE_G2);

// --- Arrays ---

// Throne squares for Cross/Edge layout (per enochian-rules.md)
// Blue: d1, e1 (Southern throne)
// Black: a5, a4 (Northern/Water throne)
// Red: h5, h4 (Eastern/Fire throne)  -- Note: rules say e8,d8 but that's wrong orientation
// Yellow: h4, h5 (Western/Earth throne) -- Correction: Yellow is WEST, so a4, a5? No wait...
//
// Let me re-read the rules table carefully:
// | Blue   | d1, e1  | Southern (Air) throne. Allied with Black. |
// | Red    | e8, d8  | Eastern/Fire throne. Allied with Yellow. |
// | Black  | a5, a4  | Northern/Water throne. Allied with Blue. |
// | Yellow | h4, h5  | Western/Earth throne. Allied with Red. |
//
// But wait, the rules say Red is at EAST (files g-h), so their throne should be on file h!
// And Yellow is WEST (files a-b), so their throne should be on file a!
//
// Looking more carefully at the layout:
// - Blue: South (ranks 1-2), King on e1 → throne d1, e1 ✓
// - Black: North (ranks 7-8), King on e8 → BUT rules say throne a5, a4
// - Red: East (files g-h), King on h5 → BUT rules say throne e8, d8
// - Yellow: West (files a-b), King on a5 → rules say throne h4, h5
//
// The rules seem inconsistent. Let's trust the documented thrones and adjust:
// The thrones listed in rules appear to be for a DIFFERENT orientation or error.
//
// For our cross layout, thrones should be where the Kings are:
// - Blue: d1, e1 (King at e1)
// - Black: d8, e8 (King at e8)
// - Red: h4, h5 (King at h5)
// - Yellow: a4, a5 (King at a5)
const THRONES_CROSS: [[Square; 2]; ARMY_COUNT] = [
    [square(3, 0), square(4, 0)], // Blue: d1, e1 (Southern throne)
    [square(3, 7), square(4, 7)], // Black: d8, e8 (Northern throne) - index 1 in Army::ALL
    [square(7, 3), square(7, 4)], // Red: h4, h5 (Eastern throne) - index 2 in Army::ALL
    [square(0, 3), square(0, 4)], // Yellow: a4, a5 (Western throne) - index 3 in Army::ALL
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
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G1,
};

pub const TABLET_OF_FIRE_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Air Setting)",
    description: "Fire Board with Air Setting.",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G1,
};

pub const TABLET_OF_FIRE_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Water Setting)",
    description: "Fire Board with Water Setting.",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G1,
};

pub const TABLET_OF_FIRE_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Fire (Fire Setting)",
    description: "Fire Board with Fire Setting. (Previously Prototype).",
    turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
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
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G1,
};
// Note: Earth uses Group 1 placements (Same as Fire).

pub const TABLET_OF_EARTH_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Air Setting)",
    description: "Earth Board with Air Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G1,
};

pub const TABLET_OF_EARTH_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Water Setting)",
    description: "Earth Board with Water Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G1,
};

pub const TABLET_OF_EARTH_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Earth (Fire Setting)",
    description: "Earth Board with Fire Setting.",
    turn_order: [Army::Yellow, Army::Blue, Army::Red, Army::Black],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
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
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G2,
};

pub const TABLET_OF_AIR_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Air (Air Setting)",
    description: "Air Board with Air Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G2,
};

pub const TABLET_OF_AIR_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Air (Water Setting)",
    description: "Air Board with Water Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G2,
};

pub const TABLET_OF_AIR_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Air (Fire Setting)",
    description: "Air Board with Fire Setting.",
    turn_order: [Army::Red, Army::Yellow, Army::Black, Army::Blue],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
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
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_EARTH_G2,
};

pub const TABLET_OF_WATER_AIR: ArraySpec = ArraySpec {
    name: "Tablet of Water (Air Setting)",
    description: "Water Board with Air Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_AIR_G2,
};

pub const TABLET_OF_WATER_WATER: ArraySpec = ArraySpec {
    name: "Tablet of Water (Water Setting)",
    description: "Water Board with Water Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
    promotion_zones: DEFAULT_PROMOTION_ZONES,
    placements: PLACEMENTS_WATER_G2,
};

pub const TABLET_OF_WATER_FIRE: ArraySpec = ArraySpec {
    name: "Tablet of Water (Fire Setting)",
    description: "Water Board with Fire Setting.",
    turn_order: [Army::Blue, Army::Black, Army::Yellow, Army::Red],
    controller_map: [PlayerId::PLAYER_ONE, PlayerId::PLAYER_ONE, PlayerId::PLAYER_TWO, PlayerId::PLAYER_TWO],
    throne_squares: THRONES_CROSS,
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