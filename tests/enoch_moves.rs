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
        | bit(square('f', 5))
        | bit(square('g', 6))
        | bit(square('h', 7))
        | bit(square('d', 5))
        | bit(square('h', 1))
        | bit(square('d', 3))
        | bit(square('c', 2))
        | bit(square('b', 1))
        | bit(square('f', 3))
        | bit(square('g', 2));

    assert_eq!(moves, expected);
}

#[test]
fn pawns_move_in_army_direction() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Pawn, square('d', 2));
    board.place_piece(Army::Red, PieceKind::Pawn, square('e', 7));
    board.place_piece(Army::Black, PieceKind::Pawn, square('g', 5));
    board.place_piece(Army::Yellow, PieceKind::Pawn, square('b', 5));

    let (blue_moves, _) = moves::compute_pawns_moves(&board, Army::Blue);
    assert_eq!(blue_moves, bit(square('d', 3)));

    let (red_moves, _) = moves::compute_pawns_moves(&board, Army::Red);
    assert_eq!(red_moves, bit(square('e', 6)));

    let (black_moves, _) = moves::compute_pawns_moves(&board, Army::Black);
    assert_eq!(black_moves, bit(square('h', 5)));

    let (yellow_moves, _) = moves::compute_pawns_moves(&board, Army::Yellow);
    assert_eq!(yellow_moves, bit(square('a', 5)));
}

#[test]
fn rook_sliding_moves() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Rook, square('d', 4));
    
    let moves = moves::compute_rooks_moves(&board, Army::Blue);
    
    // Should move along rank and file
    assert!(moves & bit(square('d', 1)) != 0);
    assert!(moves & bit(square('d', 8)) != 0);
    assert!(moves & bit(square('a', 4)) != 0);
    assert!(moves & bit(square('h', 4)) != 0);
}

#[test]
fn rook_blocked_by_own_piece() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Rook, square('d', 4));
    board.place_piece(Army::Blue, PieceKind::Pawn, square('d', 6));
    
    let moves = moves::compute_rooks_moves(&board, Army::Blue);
    
    // Should not move through own piece
    assert!(moves & bit(square('d', 6)) == 0);
    assert!(moves & bit(square('d', 7)) == 0);
    assert!(moves & bit(square('d', 8)) == 0);
    // But should move up to the blocker
    assert!(moves & bit(square('d', 5)) != 0);
}

#[test]
fn knight_moves_l_shape() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Knight, square('e', 4));
    
    let moves = moves::compute_knights_moves(&board, Army::Blue);
    
    // All 8 L-shaped moves from e4
    assert!(moves & bit(square('d', 6)) != 0);
    assert!(moves & bit(square('f', 6)) != 0);
    assert!(moves & bit(square('g', 5)) != 0);
    assert!(moves & bit(square('g', 3)) != 0);
    assert!(moves & bit(square('f', 2)) != 0);
    assert!(moves & bit(square('d', 2)) != 0);
    assert!(moves & bit(square('c', 3)) != 0);
    assert!(moves & bit(square('c', 5)) != 0);
}

#[test]
fn king_moves_one_square() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::King, square('e', 4));
    
    let moves = moves::compute_king_moves(&board, Army::Blue);
    
    // All 8 adjacent squares
    assert!(moves & bit(square('d', 5)) != 0);
    assert!(moves & bit(square('e', 5)) != 0);
    assert!(moves & bit(square('f', 5)) != 0);
    assert!(moves & bit(square('f', 4)) != 0);
    assert!(moves & bit(square('f', 3)) != 0);
    assert!(moves & bit(square('e', 3)) != 0);
    assert!(moves & bit(square('d', 3)) != 0);
    assert!(moves & bit(square('d', 4)) != 0);
}

#[test]
fn queen_blocked_by_bishop_same_diagonal() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Queen, square('e', 4));
    board.place_piece(Army::Blue, PieceKind::Bishop, square('c', 6));
    
    let moves = moves::compute_queens_moves(&board, Army::Blue);
    
    // Queen on e4 can leap to c6 (2 squares diagonally)
    // But c6 has a bishop on same diagonal system (Aries)
    // So the leap should be blocked (can only move to empty squares)
    assert!(moves & bit(square('c', 6)) == 0);
}

#[test]
fn pawn_diagonal_captures() {
    let mut board = Board::new(&[]);
    board.place_piece(Army::Blue, PieceKind::Pawn, square('e', 4));
    board.place_piece(Army::Red, PieceKind::Pawn, square('d', 5));
    board.place_piece(Army::Red, PieceKind::Pawn, square('f', 5));
    
    let (_, attacks) = moves::compute_pawns_moves(&board, Army::Blue);
    
    // Blue pawn should attack diagonally forward
    assert!(attacks & bit(square('d', 5)) != 0);
    assert!(attacks & bit(square('f', 5)) != 0);
}
