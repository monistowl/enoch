use enoch::engine::{
    board::Board,
    game::{Game, GameConfig},
};
use enoch::engine::types::{Army, PieceKind, Square};

fn square(file: char, rank: u8) -> Square {
    let file = file.to_ascii_lowercase() as u8 - b'a';
    let rank = rank - 1;
    rank as Square * 8 + file as Square
}

fn bit(square: Square) -> u64 {
    1u64 << square
}

fn build_game_with_pieces(placements: &[(Army, PieceKind, u64)]) -> Game {
    let board = board_from_placements(placements);
    Game::with_config(board, GameConfig::default())
}

fn board_from_placements(placements: &[(Army, PieceKind, u64)]) -> Board {
    let mut board = Board::new(&[]);
    for (army, kind, bitboard) in placements {
        let mut mask = *bitboard;
        while mask != 0 {
            let square = mask.trailing_zeros() as Square;
            board.place_piece(*army, *kind, square);
            mask &= mask - 1;
        }
    }
    board
}

#[test]
fn check_forces_king_move() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('a', 2))),
        (Army::Red, PieceKind::Rook, bit(square('e', 8))),
    ];

    let mut game = build_game_with_pieces(placements);
    assert!(game.king_in_check(Army::Blue));
    assert!(game.must_move_king(Army::Blue));

    let err = game.apply_move(Army::Blue, square('a', 2), square('a', 3), None);
    assert!(err.is_err());

    let ok = game.apply_move(Army::Blue, square('e', 1), square('f', 1), None);
    assert!(ok.is_ok());
}

#[test]
fn capture_king_freezes_army() {
    let placements = &[(Army::Blue, PieceKind::King, bit(square('e', 1)))];
    let mut game = build_game_with_pieces(placements);
    game.capture_king(Army::Blue);
    assert!(game.army_is_frozen(Army::Blue));
    assert!(game.state.king_square(Army::Blue).is_none());
}

#[test]
fn privileged_pawn_recognition() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Queen, bit(square('d', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('a', 2))),
    ];
    let game = build_game_with_pieces(placements);
    assert!(game.is_privileged_pawn(Army::Blue));
}

#[test]
fn privileged_pawn_demotes_existing_piece_on_promotion() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Queen, bit(square('d', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Queen),
    );
    assert!(result.is_ok());
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()],
        1
    );
    assert_eq!(
        game.board.piece_at(square('d', 1)).unwrap().1,
        PieceKind::Pawn
    );
}

#[test]
fn stalemate_detected_when_no_moves_exist() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('a', 1))), // King A1
        (Army::Red, PieceKind::Rook, bit(square('c', 2))), // Red Rook C2 (to block b1, b2)
        (Army::Red, PieceKind::Rook, bit(square('b', 3))), // Red Rook B3 (to block a2, b2)
    ];
    let mut game = build_game_with_pieces(placements);
    game.update_stalemate_status(Army::Blue);
    assert!(game.army_in_stalemate(Army::Blue), "Blue army should be in stalemate");
}

#[test]
fn prisoner_exchange_restores_kings() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Red, PieceKind::King, bit(square('e', 8))),
    ];
    let mut game = build_game_with_pieces(placements);
    game.capture_king(Army::Blue);
    game.capture_king(Army::Red);
    assert!(game.army_is_frozen(Army::Blue));
    assert!(game.army_is_frozen(Army::Red));

    let swapped = game.exchange_prisoners(Army::Blue, Army::Red);
    assert!(swapped);
    assert!(game.state.king_square(Army::Blue).is_some());
    assert!(game.state.king_square(Army::Red).is_some());
    assert!(!game.army_is_frozen(Army::Blue));
    assert!(!game.army_is_frozen(Army::Red));
}

#[test]
fn allows_non_king_move_when_king_stuck_in_check() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Rook, bit(square('d', 1))),
        (Army::Blue, PieceKind::Rook, bit(square('f', 1))),
        (Army::Blue, PieceKind::Rook, bit(square('h', 2))),
        (Army::Red, PieceKind::Rook, bit(square('e', 3))),
    ];
    let mut game = build_game_with_pieces(placements);
    assert!(game.king_in_check(Army::Blue));
    assert!(!game.must_move_king(Army::Blue));

    let result = game.apply_move(Army::Blue, square('h', 2), square('e', 2), None);
    assert!(result.is_ok());
}

#[test]
fn apply_move_rejects_opponent_move() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Red, PieceKind::Rook, bit(square('e', 8))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(Army::Red, square('e', 8), square('e', 7), None);
    assert!(result.is_err());
}

#[test]
fn promotion_targets_default_to_queen() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Rook, bit(square('h', 1))),
        (Army::Blue, PieceKind::Bishop, bit(square('c', 1))), // Added a Bishop to ensure non-privileged (Rook + Bishop)
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let game = build_game_with_pieces(placements);
    let targets = game.promotion_targets(Army::Blue);
    assert_eq!(targets, vec![PieceKind::Queen]);
}

#[test]
fn promotion_targets_privileged_pawn_returns_all_majors() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Queen, bit(square('d', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let game = build_game_with_pieces(placements);
    let targets = game.promotion_targets(Army::Blue);
    assert_eq!(
        targets,
        vec![
            PieceKind::Queen,
            PieceKind::Rook,
            PieceKind::Bishop,
            PieceKind::Knight
        ]
    );
}

#[test]
fn exchange_prisoners_requires_both_kings_missing() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Red, PieceKind::King, bit(square('e', 8))),
    ];
    let mut game = build_game_with_pieces(placements);
    game.capture_king(Army::Blue);
    let success = game.exchange_prisoners(Army::Blue, Army::Red);
    assert!(!success);
}

#[test]
fn draw_detected_when_both_kings_bare() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Red, PieceKind::King, bit(square('e', 8))),
    ];
    let mut game = build_game_with_pieces(placements);
    game.capture_king(Army::Blue);
    game.capture_king(Army::Red);
    assert!(game.draw_condition());
}

#[test]
fn apply_move_rejects_moving_into_own_piece() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 2))),
    ];
    let mut game = build_game_with_pieces(placements);
    let err = game.apply_move(Army::Blue, square('e', 1), square('e', 2), None);
    assert!(err.is_err());
}

#[test]
fn default_array_has_all_army_kings() {
    let game = Game::default();
    for &army in Army::ALL.iter() {
        assert!(game.state.king_square(army).is_some());
    }
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::King.index()],
        1
    );
}

#[test]
fn stalemate_clears_after_any_move() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('a', 1))), // King A1
        (Army::Red, PieceKind::Rook, bit(square('c', 2))), // Red Rook C2
        (Army::Red, PieceKind::Rook, bit(square('b', 3))), // Red Rook B3
    ];
    let mut game = build_game_with_pieces(placements);
    game.update_stalemate_status(Army::Blue);
    assert!(game.army_in_stalemate(Army::Blue), "Blue army should be in stalemate initially");

    // Simulate clearing the stalemate by removing one of the attacking Red Rooks
    // Remove Red Rook from c2
    game.board.remove_piece(Army::Red, PieceKind::Rook, square('c', 2));
    game.state.sync_with_board(&game.board); // Sync game state with the modified board
    game.update_stalemate_status(Army::Blue); // Recalculate stalemate status

    assert!(!game.army_in_stalemate(Army::Blue), "Stalemate should be cleared after removing an attacking piece");
}

#[test]
fn test_is_privileged_pawn_king_pawn_only() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('a', 2))),
    ];
    let game = build_game_with_pieces(placements);
    assert!(game.is_privileged_pawn(Army::Blue));
}

#[test]
fn test_privileged_promotion_demotes_queen_to_pawn() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Queen, bit(square('d', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Queen),
    );
    assert!(result.is_ok());
    // The pawn count should remain 1 (original pawn replaced by new queen, old queen demoted to pawn)
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()],
        1
    );
    // The piece at d1 (original Queen's square) should now be a Pawn
    assert_eq!(
        game.board.piece_at(square('d', 1)).unwrap().1,
        PieceKind::Pawn
    );
    // The piece at e8 (promoted pawn's square) should be a Queen
    assert_eq!(
        game.board.piece_at(square('e', 8)).unwrap().1,
        PieceKind::Queen
    );
}

#[test]
fn test_privileged_promotion_demotes_bishop_to_pawn() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Bishop, bit(square('c', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Bishop),
    );
    assert!(result.is_ok());
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()],
        1
    );
    assert_eq!(
        game.board.piece_at(square('c', 1)).unwrap().1,
        PieceKind::Pawn
    );
    assert_eq!(
        game.board.piece_at(square('e', 8)).unwrap().1,
        PieceKind::Bishop
    );
}

#[test]
fn test_privileged_promotion_demotes_rook_to_pawn() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Rook, bit(square('a', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Rook),
    );
    assert!(result.is_ok());
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()],
        1
    );
    assert_eq!(
        game.board.piece_at(square('a', 1)).unwrap().1,
        PieceKind::Pawn
    );
    assert_eq!(
        game.board.piece_at(square('e', 8)).unwrap().1,
        PieceKind::Rook
    );
}

#[test]
fn test_privileged_promotion_demotes_knight_to_pawn() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Knight, bit(square('b', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Knight),
    );
    assert!(result.is_ok());
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()],
        1
    );
    assert_eq!(
        game.board.piece_at(square('b', 1)).unwrap().1,
        PieceKind::Pawn
    );
    assert_eq!(
        game.board.piece_at(square('e', 8)).unwrap().1,
        PieceKind::Knight
    );
}

#[test]
fn test_privileged_promotion_no_demotion() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);
    let result = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Queen),
    );
    assert!(result.is_ok());
    // Pawn count should be 0 as no demotion occurs
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Pawn.index()],
        0
    );
    // Queen count should be 1
    assert_eq!(
        game.board.piece_counts(Army::Blue)[PieceKind::Queen.index()],
        1
    );
    assert_eq!(
        game.board.piece_at(square('e', 8)).unwrap().1,
        PieceKind::Queen
    );
}

#[test]
fn test_promotion_targets_cannot_be_king_or_pawn() {
    let placements = &[
        (Army::Blue, PieceKind::King, bit(square('e', 1))),
        (Army::Blue, PieceKind::Pawn, bit(square('e', 7))),
    ];
    let mut game = build_game_with_pieces(placements);

    // Try to promote to Pawn
    let result_pawn = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::Pawn),
    );
    assert!(result_pawn.is_err()); // Should be an error

    // Try to promote to King
    let result_king = game.apply_move(
        Army::Blue,
        square('e', 7),
        square('e', 8),
        Some(PieceKind::King),
    );
    assert!(result_king.is_err()); // Should be an error
}