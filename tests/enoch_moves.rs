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
