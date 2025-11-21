use enoch::engine::game::Game;
use enoch::engine::arrays::default_array;
use enoch::engine::types::Army;

#[test]
fn playtest_opening_moves() {
    let spec = default_array();
    let mut game = Game::from_array_spec(spec);
    
    println!("\n=== Initial Position ===");
    for row in game.board.ascii_rows() {
        println!("{}", row);
    }
    println!("Turn: {}", game.current_army().display_name());
    
    // Blue opens
    let result = game.apply_move(Army::Blue, 12, 20, None); // e2-e3
    println!("\n=== After Blue e2-e3 ===");
    println!("Result: {:?}", result);
    println!("{}", game.board.ascii_rows().join("\n"));
    println!("Turn: {}", game.current_army().display_name());
    
    // Red responds
    let result = game.apply_move(Army::Red, 51, 43, None); // e7-e6
    println!("\n=== After Red e7-e6 ===");
    println!("Result: {:?}", result);
    println!("{}", game.board.ascii_rows().join("\n"));
    println!("Turn: {}", game.current_army().display_name());
    
    // Black moves
    let result = game.apply_move(Army::Black, 62, 45, None); // g8-f6
    println!("\n=== After Black g8-f6 ===");
    println!("Result: {:?}", result);
    println!("{}", game.board.ascii_rows().join("\n"));
    println!("Turn: {}", game.current_army().display_name());
    
    // Yellow moves
    let result = game.apply_move(Army::Yellow, 1, 18, None); // b1-c3
    println!("\n=== After Yellow b1-c3 ===");
    println!("Result: {:?}", result);
    println!("{}", game.board.ascii_rows().join("\n"));
    println!("Turn: {}", game.current_army().display_name());
}

#[test]
fn playtest_piece_selection_flow() {
    let spec = default_array();
    let game = Game::from_array_spec(spec);
    
    println!("\n=== Testing Piece Selection ===");
    
    // Test what pieces Blue can move
    let legal_moves = game.generate_legal_moves(Army::Blue);
    println!("Blue has {} legal moves", legal_moves.len());
    
    // Group by piece
    let mut from_squares: Vec<u8> = legal_moves.iter().map(|m| m.from).collect();
    from_squares.sort();
    from_squares.dedup();
    
    println!("Blue can move pieces at:");
    for sq in from_squares {
        let file = (b'a' + (sq % 8)) as char;
        let rank = (b'1' + (sq / 8)) as char;
        let piece = game.board.piece_at(sq);
        println!("  {}{} - {:?}", file, rank, piece);
    }
}

#[test]
fn playtest_ui_workflow() {
    println!("\n=== UI Workflow Test ===");
    println!("1. User presses '1' to select Blue");
    println!("2. User types 'e2' and presses Enter");
    println!("   - Should highlight e2 in yellow");
    println!("   - Should show legal moves in green");
    println!("3. User types 'e4' and presses Enter");
    println!("   - Should execute move");
    println!("   - Should auto-select Red (next turn)");
    println!("4. User presses Tab to cycle to Black");
    println!("5. User presses ESC to clear selection");
}
