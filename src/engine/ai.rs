use crate::engine::game::Game;
use crate::engine::types::{Army, Move};
use rand::prelude::*;

/// Simple random AI that picks a random legal move
pub fn random_move(game: &mut Game, army: Army) -> Option<Move> {
    let moves = game.legal_moves(army);
    if moves.is_empty() {
        return None;
    }
    
    let mut rng = rand::thread_rng();
    moves.choose(&mut rng).copied()
}

/// AI that prefers captures over other moves
pub fn capture_preferring_move(game: &mut Game, army: Army) -> Option<Move> {
    let moves = game.legal_moves(army).to_vec();
    if moves.is_empty() {
        return None;
    }
    
    // Separate captures from non-captures
    let captures: Vec<Move> = moves.iter()
        .filter(|m| game.board.piece_at(m.to).is_some())
        .copied()
        .collect();
    
    let mut rng = rand::thread_rng();
    
    // Prefer captures if available
    if !captures.is_empty() {
        captures.choose(&mut rng).copied()
    } else {
        moves.choose(&mut rng).copied()
    }
}
