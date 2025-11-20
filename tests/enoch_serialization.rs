use enoch::engine::{
    game::Game,
    types::{Army, PieceKind, Square},
};

fn square(file: char, rank: u8) -> Square {
    let file = file.to_ascii_lowercase() as u8 - b'a';
    let rank = rank - 1;
    rank as Square * 8 + file as Square
}

#[test]
fn test_serialization_roundtrip() {
    let mut game = Game::default();
    
    println!("Blue Pawn count: {}", game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()]);
    println!("Piece at b2 (9): {:?}", game.board.piece_at(9));

    // Make a move to change state: b2 to b3 (Blue Pawn)
    let move_res = game.apply_move(Army::Blue, square('b', 2), square('b', 3), None);
    if let Err(e) = &move_res {
        println!("Move failed: {}", e);
    }
    assert!(move_res.is_ok());

    // Serialize
    let json = game.to_json().expect("Failed to serialize");
    
    // Deserialize
    let loaded_game = Game::from_json(&json).expect("Failed to deserialize");

    // Verify state matches
    assert_eq!(game.state.current_turn_index, loaded_game.state.current_turn_index);
    assert_eq!(game.config.turn_order, loaded_game.config.turn_order);
    
    // Verify board state (derived fields should be reconstructed)
    let blue_pawns_orig = game.board.by_army_kind[Army::Blue.index()][PieceKind::Pawn.index()];
    let blue_pawns_loaded = loaded_game.board.by_army_kind[Army::Blue.index()][PieceKind::Pawn.index()];
    assert_eq!(blue_pawns_orig, blue_pawns_loaded);

    assert_eq!(game.board.all_occupancy, loaded_game.board.all_occupancy);
    assert_eq!(game.board.free, loaded_game.board.free);
}
