use crate::engine::board::{
    diagonal_system_for_square, Board, MASK_FILE_A, MASK_FILE_B, MASK_FILE_G, MASK_FILE_H,
};
use crate::engine::piece_kind::ParsedMove;
use crate::engine::types::{Army, DiagonalSystem, Piece, PieceKind, Square};
use crate::precompute_moves;

/// Check if a piece can capture another piece based on Enochian chess rules.
/// Returns true if the capture is allowed, false if restricted.
///
/// Rules:
/// - Queens cannot capture enemy queens
/// - Bishops cannot capture enemy bishops
/// - Queens can only capture bishops on the same diagonal system
/// - Bishops can only capture queens on the same diagonal system
pub fn can_capture_piece(attacker: &Piece, target: &Piece) -> bool {
    match (attacker.kind, target.kind) {
        // Queens cannot capture queens
        (PieceKind::Queen, PieceKind::Queen) => false,

        // Bishops cannot capture bishops
        (PieceKind::Bishop, PieceKind::Bishop) => false,

        // Queen capturing bishop: must be same diagonal system
        (PieceKind::Queen, PieceKind::Bishop) => {
            match (attacker.diagonal_system, target.diagonal_system) {
                (Some(a), Some(t)) => a == t,
                // If either piece doesn't have a diagonal system set, allow capture
                // (backwards compatibility / test scenarios)
                _ => true,
            }
        }

        // Bishop capturing queen: must be same diagonal system
        (PieceKind::Bishop, PieceKind::Queen) => {
            match (attacker.diagonal_system, target.diagonal_system) {
                (Some(a), Some(t)) => a == t,
                _ => true,
            }
        }

        // All other captures are allowed
        _ => true,
    }
}

/// move generation related, only generate pseudo-legal moves which ensure that
/// moves are within bounds, exclude friendly pieces and exclude blocked pieces

pub const UP: (i8, i8) = (0, 1);
pub const UP_RIGHT: (i8, i8) = (1, 1);
pub const RIGHT: (i8, i8) = (1, 0);
pub const DOWN_RIGHT: (i8, i8) = (1, -1);
pub const DOWN: (i8, i8) = (0, -1);
pub const DOWN_LEFT: (i8, i8) = (-1, -1);
pub const LEFT: (i8, i8) = (-1, 0);
pub const UP_LEFT: (i8, i8) = (-1, 1);

pub const QUEEN_LEAPS: [u64; 64] = precompute_moves!(precompute_queen_leaps);

const fn precompute_queen_leaps(index: u8) -> u64 {
    let mut leaps = 0u64;
    let file = index % 8;
    let rank = index / 8;

    const DIRECTIONS: [(i8, i8); 8] = [
        (0, 2),
        (2, 2),
        (2, 0),
        (2, -2),
        (0, -2),
        (-2, -2),
        (-2, 0),
        (-2, 2),
    ];

    let mut i = 0;
    while i < DIRECTIONS.len() {
        let (dx, dy) = DIRECTIONS[i];
        let nf = file as i8 + dx;
        let nr = rank as i8 + dy;
        if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
            let dest = ((nr as u64) * 8 + nf as u64) as u8;
            leaps |= 1u64 << dest;
        }
        i += 1;
    }

    leaps
}

pub const KING_MOVES: [u64; 64] = precompute_moves!(precompute_king_moves);
// precompute all the moves available for knights at each bit index in the bitboard
const fn precompute_king_moves(index: u8) -> u64 {
    let mut moves = 0u64;
    let file = (index % 8) as i8;
    let rank = (index / 8) as i8;

    const KING_OFFSETS: [(i8, i8); 8] = [
        (0, 1),  // Up
        (1, 1),  // Up-Right
        (1, 0),  // Right
        (1, -1), // Down-Right
        (0, -1), // Down
        (-1, -1),// Down-Left
        (-1, 0), // Left
        (-1, 1), // Up-Left
    ];

    let mut i = 0;
    while i < KING_OFFSETS.len() {
        let (dx, dy) = KING_OFFSETS[i];
        let nf = file + dx;
        let nr = rank + dy;

        if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
            let dest_square_idx = (nr * 8 + nf) as u8;
            moves |= 1u64 << dest_square_idx;
        }
        i += 1;
    }
    moves
}

pub fn compute_king_moves(board: &Board, army: Army) -> u64 {
    let king = board.by_army_kind[army as usize][PieceKind::King as usize];
    if king == 0 {
        return 0;
    }
    let own_pieces = board.occupancy_by_army[army as usize];
    let index = king.trailing_zeros();
    // Add the king's precomputed moves, excluding occupied by own
    KING_MOVES[index as usize] & !own_pieces
}

pub const KNIGHT_MOVES: [u64; 64] = precompute_moves!(precompute_knight_moves);
// precompute all the moves available for knights at each bit index in the bitboard
const fn precompute_knight_moves(index: u8) -> u64 {
    let bitboard = 1u64 << index;
    // use mask to avoid wrap around
    ((bitboard << 17) & !MASK_FILE_A) // UP 2 + RIGHT 1
        | ((bitboard << 15) & !MASK_FILE_H) // UP 2 + LEFT 1
        | ((bitboard << 10) & !(MASK_FILE_A | MASK_FILE_B)) // UP 1 + RIGHT 2
        | ((bitboard << 6) & !(MASK_FILE_G | MASK_FILE_H)) // UP 1 + LEFT 2
        | ((bitboard >> 17) & !MASK_FILE_H) // DOWN 2 + LEFT 1
        | ((bitboard >> 15) & !MASK_FILE_A) // DOWN 2 + RIGHT 1
        | ((bitboard >> 10) & !(MASK_FILE_G | MASK_FILE_H)) // DOWN 1 + LEFT 2
        | ((bitboard >> 6) & !(MASK_FILE_A | MASK_FILE_B)) // DOWN 1 + RIGHT 2
}

pub fn compute_knights_moves(board: &Board, army: Army) -> u64 {
    let mut moves = 0u64;
    let own_pieces = board.occupancy_by_army[army as usize];
    // Allied king thrones are valid targets for double-occupancy
    let throne_targets = board.team_king_thrones_for_overlay(army);
    let mut knights = board.by_army_kind[army as usize][PieceKind::Knight as usize];

    while knights != 0 {
        let index = knights.trailing_zeros();
        // Add the knight's precomputed moves, excluding occupied by own (but allowing throne targets)
        moves |= KNIGHT_MOVES[index as usize] & (!own_pieces | throne_targets);

        // Remove the processed knight (use lsb approach)
        knights &= knights - 1;
    }

    moves
}



pub const ROOK_RAYS_DIRECTIONS: [(i8, i8); 4] = [UP, RIGHT, DOWN, LEFT];
pub const BISHOP_RAYS_DIRECTIONS: [(i8, i8); 4] = [UP_RIGHT, DOWN_RIGHT, DOWN_LEFT, UP_LEFT];
pub const QUEEN_RAYS_DIRECTIONS: [(i8, i8); 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

pub const ROOK_RAYS: [[u64; 4]; 64] = precompute_moves!(4, precompute_rook_rays);
pub const BISHOP_RAYS: [[u64; 4]; 64] = precompute_moves!(4, precompute_bishop_rays);
pub const QUEEN_RAYS: [[u64; 8]; 64] = precompute_moves!(8, precompute_queen_rays);

// clockwise direction
const fn precompute_rook_rays(index: u8) -> [u64; 4] {
    let mut top: u64 = 0;
    let mut right: u64 = 0;
    let mut bottom: u64 = 0;
    let mut left: u64 = 0;

    let file = index % 8;
    let rank = index / 8;

    let mut r: u8;
    let mut f: u8;

    r = rank + 1;
    while r < 8 {
        top |= 1u64 << (r * 8 + file);
        r += 1;
    }

    f = file + 1;
    while f < 8 {
        right |= 1u64 << (rank * 8 + f);
        f += 1;
    }

    r = 0;
    while r < rank {
        bottom |= 1u64 << (r * 8 + file);
        r += 1;
    }

    f = 0;
    while f < file {
        left |= 1u64 << (rank * 8 + f);
        f += 1;
    }

    [top, right, bottom, left]
}

fn compute_sliding_moves(
    mut pieces: u64,
    directions: &[(i8, i8)], // Change to (dx, dy) tuples for iterative movement
    own_pieces: u64,
    occupied: u64,
    throne_targets: u64, // Allied king thrones available for double-occupancy
) -> u64 {
    let mut moves = 0u64;

    while pieces != 0 {
        let start_square_idx = pieces.trailing_zeros() as Square;
        pieces &= pieces - 1; // Remove the processed piece

        let start_file = (start_square_idx % 8) as i8;
        let start_rank = (start_square_idx / 8) as i8;

        for &(dx, dy) in directions {
            let mut current_file = start_file;
            let mut current_rank = start_rank;

            loop {
                current_file += dx;
                current_rank += dy;

                // Check if the square is on the board
                if current_file < 0 || current_file >= 8 || current_rank < 0 || current_rank >= 8 {
                    break; // Off board, stop
                }

                let dest_square_idx = (current_rank * 8 + current_file) as Square;
                let dest_mask = 1u64 << dest_square_idx;

                // Check if occupied by own piece
                if occupied & dest_mask != 0 {
                    // There's a piece on this square
                    if own_pieces & dest_mask != 0 {
                        // It's our own piece - but check if it's a throne target for double-occupancy
                        if throne_targets & dest_mask != 0 {
                            // This is a king on throne, we can move here (overlay)
                            moves |= dest_mask;
                        }
                        // Cannot move through own pieces (including throne targets)
                        break;
                    } else {
                        // It's an opponent's piece, can capture but cannot move through
                        moves |= dest_mask;
                        break;
                    }
                } else {
                    // Square is empty, can move here
                    moves |= dest_mask;
                }
            }
        }
    }
    moves
}

pub fn compute_rooks_moves(board: &Board, army: Army) -> u64 {
    let rooks = board.by_army_kind[army as usize][PieceKind::Rook as usize];
    let own_pieces = board.occupancy_by_army[army as usize];
    let occupied = board.all_occupancy;
    let throne_targets = board.team_king_thrones_for_overlay(army);
    compute_sliding_moves(rooks, &ROOK_RAYS_DIRECTIONS, own_pieces, occupied, throne_targets)
}

const fn precompute_bishop_rays(index: u8) -> [u64; 4] {
    let mut top_right: u64 = 0;
    let mut bottom_right: u64 = 0;
    let mut bottom_left: u64 = 0;
    let mut top_left: u64 = 0;

    let file = index % 8;
    let rank = index / 8;

    let mut f: u8;
    let mut r: u8;

    f = file + 1;
    r = rank + 1;
    while f < 8 && r < 8 {
        top_right |= 1u64 << (r * 8 + f);
        f = f + 1;
        r = r + 1;
    }

    f = file + 1;
    r = rank.wrapping_sub(1);
    while f < 8 && r < 8 {
        bottom_right |= 1u64 << (r * 8 + f);
        f = f + 1;
        r = r.wrapping_sub(1); // when out of bound this will go back to 255
    }

    f = file.wrapping_sub(1);
    r = rank.wrapping_sub(1);
    while f < 8 && r < 8 {
        bottom_left |= 1u64 << (r * 8 + f);
        f = f.wrapping_sub(1);
        r = r.wrapping_sub(1); // when out of bound this will go back to 255
    }

    f = file.wrapping_sub(1);
    r = rank + 1;
    while f < 8 && r < 8 {
        top_left |= 1u64 << (r * 8 + f);
        f = f.wrapping_sub(1);
        r = r + 1; // when out of bound this will go back to 255
    }

    [top_right, bottom_right, bottom_left, top_left]
}

pub fn compute_bishops_moves(board: &Board, army: Army) -> u64 {
    let mut moves = 0u64;
    let mut bishops = board.by_army_kind[army.index()][PieceKind::Bishop.index()];
    let own_pieces = board.occupancy_by_army[army.index()];
    let throne_targets = board.team_king_thrones_for_overlay(army);

    const VECTORS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    while bishops != 0 {
        let index = bishops.trailing_zeros() as Square;
        bishops &= bishops - 1;
        let file = (index % 8) as i8;
        let rank = (index / 8) as i8;

        // Get the attacking bishop's piece data for capture restriction checks
        let attacker = board.get_piece(index).cloned().unwrap_or(Piece {
            army,
            kind: PieceKind::Bishop,
            pawn_type: None,
            diagonal_system: Some(diagonal_system_for_square(index)),
        });

        for &(dx, dy) in &VECTORS {
            let mut search_file = file;
            let mut search_rank = rank;
            loop {
                search_file += dx;
                search_rank += dy;
                if search_file < 0 || search_file >= 8 || search_rank < 0 || search_rank >= 8 {
                    break;
                }
                let dest = (search_rank as u64 * 8 + search_file as u64) as Square;
                let dest_mask = 1u64 << dest;

                // Check own pieces, but allow throne targets for double-occupancy
                if own_pieces & dest_mask != 0 {
                    if throne_targets & dest_mask != 0 {
                        // This is a king on throne, we can move here (overlay)
                        moves |= dest_mask;
                    }
                    // Cannot move through own pieces (including throne targets)
                    break;
                }

                if let Some((target_army, _)) = board.piece_at(dest) {
                    if target_army == army {
                        break;
                    }
                    // Check capture restrictions using piece metadata
                    if let Some(target) = board.get_piece(dest) {
                        if can_capture_piece(&attacker, target) {
                            moves |= dest_mask;
                        }
                    } else {
                        // Fallback if no piece_map entry (shouldn't happen)
                        moves |= dest_mask;
                    }
                    break;
                } else {
                    moves |= dest_mask;
                }
            }
        }
    }

    moves
}

// clockwise direction
const fn precompute_queen_rays(index: u8) -> [u64; 8] {
    let rook_rays = ROOK_RAYS[index as usize];
    let bishop_rays = BISHOP_RAYS[index as usize];
    let mut rays: [u64; 8] = [0; 8];
    let mut i: usize = 0;
    while i < 4 {
        rays[i * 2] = rook_rays[i];
        rays[i * 2 + 1] = bishop_rays[i];
        i += 1;
    }
    rays
}

pub fn compute_queens_moves(board: &Board, army: Army) -> u64 {
    let mut moves = 0u64;
    let mut queens = board.by_army_kind[army.index()][PieceKind::Queen.index()];
    let own_pieces = board.occupancy_by_army[army.index()];
    let throne_targets = board.team_king_thrones_for_overlay(army);

    while queens != 0 {
        let index = queens.trailing_zeros() as u8;
        let leaps = QUEEN_LEAPS[index as usize];

        // Get the attacking queen's piece data for capture restriction checks
        let attacker = board.get_piece(index).cloned().unwrap_or(Piece {
            army,
            kind: PieceKind::Queen,
            pawn_type: None,
            diagonal_system: Some(diagonal_system_for_square(index)),
        });

        let mut targets = leaps;
        while targets != 0 {
            let dest = targets.trailing_zeros() as Square;
            targets &= targets - 1;
            let dest_mask = 1u64 << dest;

            // Check own pieces, but allow throne targets for double-occupancy
            if own_pieces & dest_mask != 0 {
                if throne_targets & dest_mask != 0 {
                    // This is a king on throne, we can move here (overlay)
                    moves |= dest_mask;
                }
                continue;
            }

            match board.piece_at(dest) {
                None => moves |= dest_mask,
                Some((target_army, _)) => {
                    if target_army == army {
                        continue;
                    }

                    // Check capture restrictions using piece metadata
                    if let Some(target) = board.get_piece(dest) {
                        if can_capture_piece(&attacker, target) {
                            moves |= dest_mask;
                        }
                    } else {
                        // Fallback if no piece_map entry (shouldn't happen)
                        moves |= dest_mask;
                    }
                }
            }
        }

        queens &= queens - 1;
    }

    moves
}

pub fn compute_pawns_moves(board: &Board, army: Army) -> (u64, u64) {
    let mut moves = 0u64;
    let mut attack_moves = 0u64;
    let own_pieces = board.occupancy_by_army[army.index()];
    let throne_targets = board.team_king_thrones_for_overlay(army);
    let mut pawns = board.by_army_kind[army.index()][PieceKind::Pawn.index()];

    while pawns != 0 {
        let index = pawns.trailing_zeros() as usize;
        pawns &= pawns - 1;

        let file = (index % 8) as i8;
        let rank = (index / 8) as i8;

        let (forward, diag_left, diag_right) = match army {
            Army::Blue => (
                offset_square(file, rank, 0, 1),
                offset_square(file, rank, -1, 1),
                offset_square(file, rank, 1, 1),
            ),
            Army::Red => (
                offset_square(file, rank, 0, -1),
                offset_square(file, rank, -1, -1),
                offset_square(file, rank, 1, -1),
            ),
            Army::Black => (
                offset_square(file, rank, 1, 0),
                offset_square(file, rank, 1, 1),
                offset_square(file, rank, 1, -1),
            ),
            Army::Yellow => (
                offset_square(file, rank, -1, 0),
                offset_square(file, rank, -1, 1),
                offset_square(file, rank, -1, -1),
            ),
        };

        if let Some(dest) = forward {
            let dest_mask = 1u64 << dest;
            if board.all_occupancy & dest_mask == 0 {
                moves |= dest_mask;
            }
        }

        for diag in [diag_left, diag_right] {
            if let Some(dest) = diag {
                let dest_mask = 1u64 << dest;
                // Allow throne targets for double-occupancy (pawn moves diagonally to overlay)
                if own_pieces & dest_mask == 0 || throne_targets & dest_mask != 0 {
                    attack_moves |= dest_mask;
                }
            }
        }
    }

    (moves, attack_moves)
}

fn offset_square(file: i8, rank: i8, df: i8, dr: i8) -> Option<u8> {
    let nf = file + df;
    let nr = rank + dr;
    if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
        Some(((nr as u64) * 8 + nf as u64) as u8)
    } else {
        None
    }
}