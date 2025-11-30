use enoch::engine::piece_kind::parse_move;
use enoch::engine::types::PieceKind;

#[test]
fn test_parse_simple_move() {
    // "e4" -> Pawn to e4
    let m = parse_move("e4").expect("Should parse e4");
    assert_eq!(m.piece, PieceKind::Pawn);
    // e4 is rank 4 (index 3), file e (index 4). 3*8 + 4 = 28.
    // Wait, the parser returns a bitboard for `to`.
    // 1 << 28.
    assert_eq!(m.to, 1u64 << 28);
}

#[test]
fn test_parse_piece_move() {
    // "Nf3" -> Knight to f3
    let m = parse_move("Nf3").expect("Should parse Nf3");
    assert_eq!(m.piece, PieceKind::Knight);
    // f3 is rank 3 (index 2), file f (index 5). 2*8 + 5 = 21.
    assert_eq!(m.to, 1u64 << 21);
}
