use enoch::engine::game::{Game, GameConfig, Mode};
use enoch::engine::types::{Army, PieceKind, Square};

fn square(file: char, rank: u8) -> Square {
    let file = file.to_ascii_lowercase() as u8 - b'a';
    let rank = rank - 1;
    rank as Square * 8 + file as Square
}

#[test]
fn test_divination_constraint() {
    let mut game = Game::default();
    game.config.mode = Mode::Divination;
    
    // Force die to 5 (Rook)
    game.state.divination_die = Some(5);
    
    // Try moving a King (Blue King at e1)
    // Tablet of Fire: Blue King at e1.
    // Move e1 -> d1 (Throne? No, d1 is throne. King starts at e1. d1 is occupied by Rook? No, d1 is empty or occupied by partner?)
    // Let's check arrays.rs:
    // TABLET_OF_FIRE_FIRE (Group 1):
    // Blue Rank 1: A(Partner), B(Piece2), C(Piece1), D(Piece0/Partner), E(King), F(Piece1), G(Piece2), H(Piece3).
    // Wait, index 3 is D. Index 4 is E.
    // SETTING_FIRE_G1: [Knight, Queen, Bishop, Rook].
    // Index 0: Knight. Index 1: Queen. Index 2: Bishop. Index 3: Rook.
    // So:
    // A(0): Rook.
    // B(1): Bishop.
    // C(2): Queen.
    // D(3): Knight (Partner).
    // E(4): King.
    // F(5): Queen.
    // G(6): Bishop.
    // H(7): Rook.
    
    // Blue King at e1 (4).
    // Blue Rook at a1 (0) and h1 (7).
    
    // If die is 5 (Rook), King move should fail.
    // King e1 -> d1? d1 has Knight.
    // King e1 -> f1? f1 has Queen.
    // King e1 -> e2? e2 has Pawn.
    // So King has NO moves anyway?
    // Wait, standard array is crowded.
    // Let's clear board and place specific pieces.
    
    let mut game = Game::new(enoch::engine::board::Board::new(&[]));
    game.config.mode = Mode::Divination;
    game.state.divination_die = Some(5); // Rook
    
    // Place Blue King at e4.
    game.board.place_piece(Army::Blue, PieceKind::King, square('e', 4));
    
    // Place Blue Rook at a1.
    game.board.place_piece(Army::Blue, PieceKind::Rook, square('a', 1));
    
    // King move: e4 -> e5 (valid move, but forbidden by die)
    let result = game.apply_move(Army::Blue, square('e', 4), square('e', 5), None);
    assert!(result.is_err(), "King move should be forbidden when die is Rook");
    
    // Rook move: a1 -> a2 (valid move, allowed by die)
    // Note: apply_move also checks turn order. default turn is Blue.
    let result = game.apply_move(Army::Blue, square('a', 1), square('a', 2), None);
    assert!(result.is_ok(), "Rook move should be allowed when die is Rook");
}

#[test]
fn test_divination_king_pawn_rule() {
    let mut game = Game::new(enoch::engine::board::Board::new(&[]));
    game.config.mode = Mode::Divination;
    game.state.divination_die = Some(1); // King or Pawn
    
    // Place Blue King at e4.
    game.board.place_piece(Army::Blue, PieceKind::King, square('e', 4));
    // Place Blue Pawn at h2.
    game.board.place_piece(Army::Blue, PieceKind::Pawn, square('h', 2));
    
    // King move: e4 -> e5 (Allowed)
    let result = game.apply_move(Army::Blue, square('e', 4), square('e', 5), None);
    assert!(result.is_ok(), "King move allowed on die 1");
    
    // Reset turn to Blue (apply_move advances turn)
    game.state.current_turn_index = 0;
    game.state.divination_die = Some(1);
    
    // Pawn move: h2 -> h3 (Allowed)
    let result = game.apply_move(Army::Blue, square('h', 2), square('h', 3), None);
    assert!(result.is_ok(), "Pawn move allowed on die 1");
}
