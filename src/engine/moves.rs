use crate::engine::board::{
    diagonal_system, Board, MASK_FILE_A, MASK_FILE_B, MASK_FILE_G, MASK_FILE_H,
};
use crate::engine::piece_kind::ParsedMove;
use crate::engine::types::{Army, PieceKind, Square};
use crate::precompute_moves;
/// move generation related, only generate pseudo-legal moves which ensure that
/// moves are within bounds, exclude friendly pieces and exclude blocked pieces

pub const UP: usize = 0;
pub const UP_RIGHT: usize = 1;
pub const RIGHT: usize = 2;
pub const DOWN_RIGHT: usize = 3;
pub const DOWN: usize = 4;
pub const DOWN_LEFT: usize = 5;
pub const LEFT: usize = 6;
pub const UP_LEFT: usize = 7;

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
    let bitboard = 1u64 << index;
    // use mask to avoid wrap around
    ((bitboard << 8))                       // up
        | ((bitboard >> 8))                     // down
        | ((bitboard << 1) & !MASK_FILE_A)      // right
        | ((bitboard >> 1) & !MASK_FILE_H)      // left
        | ((bitboard << 9) & !MASK_FILE_A)      // up-right
        | ((bitboard << 7) & !MASK_FILE_H)      // up-left
        | ((bitboard >> 9) & !MASK_FILE_H)      // down-left
        | ((bitboard >> 7) & !MASK_FILE_A) // down-right
}

pub fn compute_king_moves(board: &Board, army: Army) -> u64 {
    let king = board.by_army_kind[army as usize][PieceKind::King as usize];
    if king == 0 {
        return 0;
    }
    let own_pieces = board.occupancy_by_army[army as usize];
    let index = king.trailing_zeros();
    let computed_moves = KING_MOVES[index as usize] & !own_pieces;
    computed_moves
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
    let mut knights = board.by_army_kind[army as usize][PieceKind::Knight as usize];

    while knights != 0 {
        let index = knights.trailing_zeros();
        // Add the knight's precomputed moves, excluding occupied by own
        moves |= KNIGHT_MOVES[index as usize] & !own_pieces;

        // Remove the processed knight (use lsb approach)
        knights &= knights - 1;
    }

    moves
}

/// Finds the blocker along the given ray for a given direction.
/// Once a blocker is found, all the remaining move for the ray is marked
/// as blocked and returns the tuple of first blocker and blocker mask.
/// Returns (0, 0) if no blocking found
/// Important: caller is responsible to pass the correct ray and direction
pub fn find_blocker_mask(ray: u64, occupied: u64, direction: usize) -> (u64, u64) {
    let blockers = ray & occupied;
    if blockers == 0 {
        (0, 0)
    } else {
        let blocker_idx;
        let available_moves;
        if matches!(direction, UP | UP_RIGHT | RIGHT | UP_LEFT) {
            blocker_idx = blockers.trailing_zeros();
            available_moves = ray & !(u64::MAX << blocker_idx);
        } else {
            // for directions down, left or down-left/down-right
            // 63 minus X is required because we are shifting to the left
            blocker_idx = 63 - blockers.leading_zeros();
            available_moves = ray & (u64::MAX << (blocker_idx + 1))
        };

        let blocker_pos = 1 << blocker_idx;

        // XOR with ray to get the blocked mask
        (blocker_pos, ray ^ available_moves)
    }
}

pub const ROOK_RAYS_DIRECTIONS: [usize; 4] = [UP, RIGHT, DOWN, LEFT];
pub const BISHOP_RAYS_DIRECTIONS: [usize; 4] = [UP_RIGHT, DOWN_RIGHT, DOWN_LEFT, UP_LEFT];
pub const QUEEN_RAYS_DIRECTIONS: [usize; 8] = [
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

fn get_sliding_moves(
    mut pieces: u64,
    directions: &[usize],
    own_pieces: u64,
    occupied: u64,
) -> u64 {
    let mut moves = 0u64;

    while pieces != 0 {
        let index = pieces.trailing_zeros();
        let rays = QUEEN_RAYS[index as usize];

        for &dir in directions {
            let ray = rays[dir];

            let (blocked_bit, blocked_mask) = find_blocker_mask(ray, occupied, dir);
            // ray & inverted block mask to show the available move in the ray
            moves |= ray & !blocked_mask;

            // if first blocked piece is an opponent, we can move here
            if blocked_bit & own_pieces == 0 {
                moves |= blocked_bit;
            }
        }

        // Remove the processed piece (use lsb approach)
        pieces &= pieces - 1;
    }
    moves
}

pub fn get_sliding_attacks(
    mut pieces: u64,
    directions: &[usize],
    occupied: u64,
) -> u64 {
    let mut attacks = 0u64;

    while pieces != 0 {
        let index = pieces.trailing_zeros();
        let rays = QUEEN_RAYS[index as usize]; // Assuming QUEEN_RAYS contain all directional rays

        for &dir in directions {
            let ray = rays[dir];
            let (blocked_bit, blocked_mask) = find_blocker_mask(ray, occupied, dir);
            attacks |= ray & !blocked_mask; // squares up to the blocker
            attacks |= blocked_bit; // include the blocker itself
        }

        pieces &= pieces - 1;
    }
    attacks
}

pub fn compute_rooks_moves(board: &Board, army: Army) -> u64 {
    let rooks = board.by_army_kind[army as usize][PieceKind::Rook as usize];
    let own_pieces = board.occupancy_by_army[army as usize];
    let occupied = board.all_occupancy;
    get_sliding_moves(rooks, &ROOK_RAYS_DIRECTIONS, own_pieces, occupied)
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

    const VECTORS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    while bishops != 0 {
        let index = bishops.trailing_zeros() as Square;
        bishops &= bishops - 1;
        let diag_system = diagonal_system(index);
        let file = (index % 8) as i8;
        let rank = (index / 8) as i8;

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
                if own_pieces & dest_mask != 0 {
                    break;
                }

                if let Some((target_army, target_kind)) = board.piece_at(dest) {
                    if target_army == army {
                        break;
                    }
                    match target_kind {
                        PieceKind::Bishop => {
                            break;
                        }
                        PieceKind::Queen => {
                            if diagonal_system(dest) == diag_system {
                                moves |= dest_mask;
                            }
                        }
                        _ => {
                            moves |= dest_mask;
                        }
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

    while queens != 0 {
        let index = queens.trailing_zeros() as u8;
        let diag_system = diagonal_system(index);
        let leaps = QUEEN_LEAPS[index as usize];

        let mut targets = leaps;
        while targets != 0 {
            let dest = targets.trailing_zeros() as Square;
            targets &= targets - 1;
            let dest_mask = 1u64 << dest;
            if own_pieces & dest_mask != 0 {
                continue;
            }

            match board.piece_at(dest) {
                None => moves |= dest_mask,
                Some((target_army, target_kind)) => {
                    if target_army == army {
                        continue;
                    }

                    match target_kind {
                        PieceKind::Queen => continue,
                        PieceKind::Bishop => {
                            if diagonal_system(dest) == diag_system {
                                moves |= dest_mask;
                            }
                        }
                        _ => {
                            moves |= dest_mask;
                        }
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
                if own_pieces & dest_mask == 0 {
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
