use enoch::engine::{
    board::Board,
    moves,
    types::{Army, PieceKind, Square},
};

fn square(file: char, rank: u8) -> Square {
    let file = file.to_ascii_lowercase() as u8 - b'a';
    let rank = rank - 1;
    rank as Square * 8 + file as Square
}

fn bit(square: Square) -> u64 {
    1u64 << square
}

#[test]
fn queen_leap_pattern_from_center() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Queen, square('e', 4));

    let moves = moves::compute_queens_moves(&board, Army::Blue);
    let expected = bit(square('e', 6))
        | bit(square('g', 6))
        | bit(square('g', 4))
        | bit(square('g', 2))
        | bit(square('e', 2))
        | bit(square('c', 2))
        | bit(square('c', 4))
        | bit(square('c', 6));

    assert_eq!(moves, expected);
}

#[test]
fn queen_leap_pattern_from_edge() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Queen, square('h', 4));

    let moves = moves::compute_queens_moves(&board, Army::Blue);
    let expected = bit(square('h', 6))
        | bit(square('h', 2))
        | bit(square('f', 2))
        | bit(square('f', 4))
        | bit(square('f', 6));

    assert_eq!(moves, expected);
}

#[test]
fn bishop_moves_follow_aries_diagonal() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Bishop, square('e', 4));

    let moves = moves::compute_bishops_moves(&board, Army::Blue);
    let expected = bit(square('a', 8))
        | bit(square('b', 7))
        | bit(square('c', 6))
        | bit(square('d', 5))
        | bit(square('f', 5))
        | bit(square('g', 6))
        | bit(square('h', 7))
        | bit(square('b', 1))
        | bit(square('c', 2))
        | bit(square('d', 3))
        | bit(square('f', 3))
        | bit(square('g', 2))
        | bit(square('h', 1));

    assert_eq!(moves, expected);
}

#[test]
fn pawns_move_in_army_direction() {
    // Cross/Edge layout directions:
    // Blue (South edge): march north (+rank)
    // Black (North edge): march south (-rank)
    // Yellow (West edge): march east (+file)
    // Red (East edge): march west (-file)
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Pawn, square('d', 2));
    board.place_piece(Army::Black, PieceKind::Pawn, square('e', 7));
    board.place_piece(Army::Yellow, PieceKind::Pawn, square('b', 5));
    board.place_piece(Army::Red, PieceKind::Pawn, square('g', 5));

    // Blue: d2 -> d3 (north)
    let (blue_moves, _) = moves::compute_pawns_moves(&board, Army::Blue);
    assert_eq!(blue_moves, bit(square('d', 3)));

    // Black: e7 -> e6 (south)
    let (black_moves, _) = moves::compute_pawns_moves(&board, Army::Black);
    assert_eq!(black_moves, bit(square('e', 6)));

    // Yellow: b5 -> c5 (east)
    let (yellow_moves, _) = moves::compute_pawns_moves(&board, Army::Yellow);
    assert_eq!(yellow_moves, bit(square('c', 5)));

    // Red: g5 -> f5 (west)
    let (red_moves, _) = moves::compute_pawns_moves(&board, Army::Red);
    assert_eq!(red_moves, bit(square('f', 5)));
}

#[test]
fn queen_capture_restrictions() {
    let mut board = Board::new(&[]);
    // Blue Queen at e4 (Aries system? e4 is light square? Need to check diagonal system)
    board.place_piece(Army::Blue, PieceKind::Queen, square('e', 4));

    // Enemy Queen at e6 (Leap reachable). Should NOT be capturable.
    board.place_piece(Army::Red, PieceKind::Queen, square('e', 6));

    // Enemy Bishop at g6 (Leap reachable).
    // e4 and g6.
    // e4 (28): 0011100 (Rank 4, File 4).
    // g6 (46): 101110 (Rank 6, File 6).
    // Are they same system?
    // Board::diagonal_system uses bitmask.
    // Let's assume valid capture for now and verify with assertion.
    board.place_piece(Army::Red, PieceKind::Bishop, square('g', 6));

    let moves = moves::compute_queens_moves(&board, Army::Blue);

    // Check Queen vs Queen capture (Should be 0)
    assert_eq!(moves & bit(square('e', 6)), 0, "Queen cannot capture Queen");

    // Check Queen vs Bishop capture (Should be 1 IF same system)
    // We need to know if they are same system.
    // In `board.rs`, `ARIES_DIAGONALS` mask determines it.
    // If both are Aries or both Cancer, capture is allowed.
    // If different, capture is forbidden.
    // Let's check if e4 and g6 are same system.
    // e4 is index 28. g6 is index 46.
    // They are on same diagonal (c2-h7?). c2(10), d3(19), e4(28), f5(37), g6(46).
    // Yes, they are on same diagonal. So same system.
    // So capture should be allowed.
    assert_ne!(moves & bit(square('g', 6)), 0, "Queen should capture Bishop on same system");
}

#[test]
fn bishop_capture_restrictions() {
    let mut board = Board::new(&[]);
    // Blue Bishop at e4.
    board.place_piece(Army::Blue, PieceKind::Bishop, square('e', 4));

    // Enemy Bishop at g6. Should NOT be capturable.
    board.place_piece(Army::Red, PieceKind::Bishop, square('g', 6));

    // Enemy Queen at c2. (Same diagonal).
    board.place_piece(Army::Red, PieceKind::Queen, square('c', 2));

    let moves = moves::compute_bishops_moves(&board, Army::Blue);

    // Check Bishop vs Bishop capture
    assert_eq!(moves & bit(square('g', 6)), 0, "Bishop cannot capture Bishop");

    // Check Bishop vs Queen capture (Same system)
    assert_ne!(moves & bit(square('c', 2)), 0, "Bishop should capture Queen on same system");
}

#[test]
fn pawn_capture_edge_cases() {
    let mut board = Board::new(&[]);
    // Blue Pawn at h2 (Right Edge). Moves North (+8). Captures North-West (+7) or North-East (+9).
    // h2 is index 15.
    // +8 = 23 (h3).
    // +7 = 22 (g3). (Capture Left).
    // +9 = 24 (a4)? No. 15+9 = 24.
    // h2 is file 7. +1 goes to file 0 (wrap)? No.
    // Check if compute_pawns_moves filters wrapping.
    board.place_piece(Army::Blue, PieceKind::Pawn, square('h', 2));

    // Enemy at g3 (Valid capture target)
    board.place_piece(Army::Red, PieceKind::Pawn, square('g', 3));

    let (moves, attacks) = moves::compute_pawns_moves(&board, Army::Blue);

    // Moves: h3 (quiet).
    assert_eq!(moves, bit(square('h', 3)));

    // Attacks: g3 (capture).
    // Should NOT capture "i3" (off board).
    assert_eq!(attacks, bit(square('g', 3)));
}

#[test]
fn pawn_promotion_zone_generation() {
    let mut board = Board::new(&[]);
    // Blue Pawn at e7. Moving North to e8.
    // e8 is Rank 8. Rank 8 is promotion zone for Blue.
    board.place_piece(Army::Blue, PieceKind::Pawn, square('e', 7));

    // The compute_pawns_moves function returns (moves, attacks).
    // It doesn't explicitly return "Promotion Move".
    // But the move to e8 should be generated.
    // The Game::apply_move logic handles the actual promotion effect.
    // Here we just check if the move to the promotion square is generated.

    let (moves, _) = moves::compute_pawns_moves(&board, Army::Blue);
    assert_eq!(moves, bit(square('e', 8)));
}


