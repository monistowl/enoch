use crate::engine::game::{Game, Status, Mode};
use crate::engine::types::{Army, PieceKind, Square, Team};
use crate::engine::moves::{
    compute_bishops_moves, compute_king_moves, compute_knights_moves, compute_pawns_moves,
    compute_queens_moves, compute_rooks_moves,
};
use rand::seq::SliceRandom;

// Basic material values
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 300;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

/// AI player for Enochian Chess.
/// Currently uses single-ply material evaluation.
/// TODO: Implement minimax with alpha-beta pruning using depth parameter.
pub struct Ai {
    /// Search depth for future minimax implementation
    #[allow(dead_code)]
    depth: u8,
}

impl Ai {
    /// Create a new AI with the given search depth.
    /// Note: depth is currently unused (single-ply only) but reserved for future minimax.
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }

    pub fn select_move(&self, game: &Game) -> Option<(Army, Square, Square, Option<PieceKind>)> {
        if game.config.mode == Mode::Divination {
            return self.random_legal_move(game);
        }

        let current_army = game.current_army();
        let legal_moves = self.generate_legal_moves(game, current_army);
        if legal_moves.is_empty() {
            return None;
        }

        let mut best_score = i32::MIN;
        let mut best_move: Option<(Army, Square, Square, Option<PieceKind>)> = None;
        
        // We'll shuffle to add variety
        let mut rng = rand::thread_rng();
        let mut shuffled_moves = legal_moves.clone();
        shuffled_moves.shuffle(&mut rng);

        for mv in shuffled_moves {
            let (army, from, to, promo) = mv;
            let mut clone = game.clone();
            
            // Apply move
            if clone.apply_move(army, from, to, promo).is_ok() {
                let score = self.evaluate(&clone, current_army.team());
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
            }
        }
        
        best_move
    }
    
    fn random_legal_move(&self, game: &Game) -> Option<(Army, Square, Square, Option<PieceKind>)> {
        let moves = self.generate_legal_moves(game, game.current_army());
        let mut valid_moves = Vec::new();
        
        for mv in moves {
             let mut clone = game.clone();
             if clone.apply_move(mv.0, mv.1, mv.2, mv.3).is_ok() {
                 valid_moves.push(mv);
             }
        }
        
        if valid_moves.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        valid_moves.choose(&mut rng).cloned()
    }

    fn generate_legal_moves(&self, game: &Game, army: Army) -> Vec<(Army, Square, Square, Option<PieceKind>)> {
        let mut moves = Vec::new();
        
        // Iterate all squares to find army pieces
        for square in 0..64 {
            if let Some((a, kind)) = game.board.piece_at(square) {
                if a == army {
                    let moves_bb = self.compute_piece_moves(game, army, kind, square);
                    let mut mask = moves_bb;
                    while mask != 0 {
                        let to = mask.trailing_zeros() as Square;
                        
                        // Check promotion
                        if kind == PieceKind::Pawn && game.can_promote_at(army, to) {
                            // Generate promotion variants
                            let targets = game.promotion_targets(army);
                            for t in targets {
                                moves.push((army, square, to, Some(t)));
                            }
                        } else {
                            moves.push((army, square, to, None));
                        }
                        
                        mask &= mask - 1;
                    }
                }
            }
        }
        moves
    }
    
    fn compute_piece_moves(&self, game: &Game, army: Army, kind: PieceKind, _from: Square) -> u64 {
        match kind {
            PieceKind::King => compute_king_moves(&game.board, army),
            PieceKind::Queen => compute_queens_moves(&game.board, army),
            PieceKind::Rook => compute_rooks_moves(&game.board, army),
            PieceKind::Bishop => compute_bishops_moves(&game.board, army),
            PieceKind::Knight => compute_knights_moves(&game.board, army),
            PieceKind::Pawn => {
                let (moves, attacks) = compute_pawns_moves(&game.board, army);
                moves | attacks
            }
        }
    }
    
    fn evaluate(&self, game: &Game, maximizing_team: Team) -> i32 {
        let mut score = 0;
        for square in 0..64 {
            if let Some((army, kind)) = game.board.piece_at(square) {
                let value = match kind {
                    PieceKind::Pawn => PAWN_VALUE,
                    PieceKind::Knight => KNIGHT_VALUE,
                    PieceKind::Bishop => BISHOP_VALUE,
                    PieceKind::Rook => ROOK_VALUE,
                    PieceKind::Queen => QUEEN_VALUE,
                    PieceKind::King => KING_VALUE,
                };
                
                if army.team() == maximizing_team {
                    score += value;
                } else {
                    score -= value;
                }
            }
        }
        
        // Bonus for Kings being alive
        let my_kings = game.state.kings_alive(maximizing_team);
        let enemy_kings = game.state.kings_alive(maximizing_team.opponent());
        
        score += (my_kings as i32) * 5000; // Extra safety buffer
        score -= (enemy_kings as i32) * 5000;
        
        score
    }
}
