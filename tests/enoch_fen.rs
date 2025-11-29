use enoch::engine::game::Game;
use enoch::engine::types::{Army, PieceKind, Square};

fn square(file: char, rank: u8) -> Square {
    let file = file.to_ascii_lowercase() as u8 - b'a';
    let rank = rank - 1;
    rank as Square * 8 + file as Square
}

#[test]
fn test_fen_roundtrip_default() {
    let game = Game::default();
    let json = game.to_enoch_fen();
    
    // Basic validation
    assert!(json.contains("Blue"), "FEN should contain Army names");
    assert!(json.contains("turn_order"), "FEN should contain turn_order");

    let restored = Game::from_enoch_fen(&json).expect("Failed to deserialize FEN");

    assert_eq!(restored.state.current_turn_index, game.state.current_turn_index);
    // Check board equality via ASCII representation
    assert_eq!(restored.board.ascii_rows(), game.board.ascii_rows());
}

#[test]
fn test_fen_roundtrip_mid_game() {
    let mut game = Game::default();
    
    // Make a move: Blue King e1 -> e2 (Just to change state)
    // Default setup: King at e1. e2 is empty?
    // Tablet of Fire (Fire Setting):
    // Blue Ranks 1 & 2.
    // Rank 1: ... K(e1) ...
    // Rank 2: Pawns.
    // e2 is Pawn? Yes.
    // So King cannot move to e2.
    // Let's move a Pawn. e2 -> e3.
    
    game.apply_move(Army::Blue, square('e', 2), square('e', 3), None).expect("Valid pawn move");
    
    let json = game.to_enoch_fen();
    let restored = Game::from_enoch_fen(&json).expect("Failed to deserialize FEN");

    // Check board equality
    assert_eq!(restored.board.ascii_rows(), game.board.ascii_rows());
    
    // Check turn index (should be advanced)
    assert_eq!(restored.state.current_turn_index, game.state.current_turn_index);
    assert_eq!(restored.current_army(), game.current_army());
}
