use enoch::engine::game::Game;
use enoch::engine::types::{Army, PieceKind, Square, Team};
use std::fs;

fn load_fixture(name: &str) -> Game {
    let json = fs::read_to_string(format!("tests/data/{}.json", name)).expect("Unable to read fixture");
    Game::from_enoch_fen(&json).expect("Unable to deserialize fixture")
}

#[test]
fn test_fixture_checkmate_blue() {
    let game = load_fixture("checkmate_blue");
    
    // Blue King should be in check
    assert!(game.king_in_check(Army::Blue), "Blue King should be in check");
    
    // Blue MUST move King (even if into check, it's the only option)
    assert!(game.must_move_king(Army::Blue), "Blue must move King");
}

#[test]
fn test_fixture_stalemate_blue() {
    let game = load_fixture("stalemate_blue");
    
    // Blue King should NOT be in check
    assert!(!game.king_in_check(Army::Blue), "Blue King should NOT be in check");
    
    // Check specific squares
    let opponent = Team::Earth; // Red
    let a2 = 8; // a2
    let b1 = 1; // b1
    let b2 = 9; // b2
    
    let a2_attacked = game.is_square_attacked_by_team(a2, opponent);
    let b1_attacked = game.is_square_attacked_by_team(b1, opponent);
    let b2_attacked = game.is_square_attacked_by_team(b2, opponent);
    
    assert!(a2_attacked, "a2 should be attacked by c2 Rook");
    assert!(b1_attacked, "b1 should be attacked by b8 Rook");
    assert!(b2_attacked, "b2 should be attacked by both Rooks");
    
    // Blue should be marked as stalemated
    assert!(game.state.is_stalemated(Army::Blue), "Blue should be marked as stalemated");
}
