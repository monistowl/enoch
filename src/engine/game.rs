use crate::engine::arrays::{ArraySpec, TABLET_OF_FIRE_PROTOTYPE};
use crate::engine::board::Board;
use crate::engine::moves::{
    compute_bishops_moves, compute_king_moves, compute_knights_moves, compute_pawns_moves,
    compute_queens_moves, compute_rooks_moves, get_sliding_attacks,
};
use crate::engine::piece_kind::{parse_move, ParsedMove, SpecialMove};
use crate::engine::types::{
    file_char, rank_char, Army, Move, PieceKind, PlayerId, Square, Team, ARMY_COUNT,
    PIECE_KIND_COUNT,
};

/// Game struct responsible for all game logics (pin, check, valid captures, etc)
#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub config: GameConfig,
    pub state: GameState,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub armies: [Army; ARMY_COUNT],
    pub turn_order: [Army; ARMY_COUNT],
    pub controller_map: [PlayerId; ARMY_COUNT],
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            armies: Army::ALL,
            turn_order: [Army::Blue, Army::Red, Army::Black, Army::Yellow],
            controller_map: [
                PlayerId::PLAYER_ONE,
                PlayerId::PLAYER_TWO,
                PlayerId::PLAYER_ONE,
                PlayerId::PLAYER_TWO,
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_turn_index: usize,
    pub army_frozen: [bool; ARMY_COUNT],
    pub king_positions: [Option<Square>; ARMY_COUNT],
    pub stalemated_armies: [bool; ARMY_COUNT],
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_turn_index: 0,
            army_frozen: [false; ARMY_COUNT],
            king_positions: [None; ARMY_COUNT],
            stalemated_armies: [false; ARMY_COUNT],
        }
    }

    pub fn sync_with_board(&mut self, board: &Board) {
        for army in Army::ALL {
            self.army_frozen[army.index()] = board.is_army_frozen(army);
            self.king_positions[army.index()] = board.king_square(army);
            self.stalemated_armies[army.index()] = false;
        }
    }

    pub fn current_army(&self, config: &GameConfig) -> Army {
        config.turn_order[self.current_turn_index]
    }

    pub fn advance_turn(&mut self, config: &GameConfig) {
        self.current_turn_index = (self.current_turn_index + 1) % config.turn_order.len();
    }

    pub fn king_square(&self, army: Army) -> Option<Square> {
        self.king_positions[army.index()]
    }

    pub fn set_king_square(&mut self, army: Army, square: Option<Square>) {
        self.king_positions[army.index()] = square;
    }

    pub fn set_frozen(&mut self, army: Army, frozen: bool) {
        self.army_frozen[army.index()] = frozen;
    }

    pub fn set_stalemate(&mut self, army: Army, stalemated: bool) {
        self.stalemated_armies[army.index()] = stalemated;
    }

    pub fn is_stalemated(&self, army: Army) -> bool {
        self.stalemated_armies[army.index()]
    }

    pub fn kings_alive(&self, team: Team) -> usize {
        team.armies()
            .iter()
            .filter(|&&army| self.king_positions[army.index()].is_some())
            .count()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum InvalidMoveReason {
    NoSourceOrTarget,
    InvalidSourceOrTarget,
    MultipleTargets,
    InvalidCaptureTarget,
    KingCaptureMove,
    PawnNonDiagonalCapture,
    PawnInvalidPromotion,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MoveError {
    AmbiguousSource,
    InvalidMove(InvalidMoveReason),
    Pinned,
    Checked,
    ParseError,
    GameOver,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    Ongoing,
    Draw,
    Checkmate,
}

impl Game {
    pub fn new(board: Board) -> Game {
        let config = GameConfig::default();
        Game::with_config(board, config)
    }

    pub fn from_array_spec(spec: &ArraySpec) -> Game {
        let mut config = GameConfig::default();
        config.turn_order = spec.turn_order;
        config.controller_map = spec.controller_map;
        let board = spec.board();
        Game::with_config(board, config)
    }

    pub fn with_config(board: Board, config: GameConfig) -> Game {
        let mut state = GameState::new();
        state.sync_with_board(&board);
        Game {
            board,
            config,
            state,
            status: Status::Ongoing,
        }
    }

    pub fn army_is_frozen(&self, army: Army) -> bool {
        self.state.army_frozen[army.index()]
    }

    pub fn king_moves_bitboard(&self, army: Army) -> u64 {
        if self.army_is_frozen(army) {
            return 0;
        }
        compute_king_moves(&self.board, army)
    }

    pub fn army_moves_bitboard(&self, army: Army) -> u64 {
        if self.army_is_frozen(army) {
            return 0;
        }

        let enemy_mask = self.board.all_occupancy & !self.board.occupancy_by_army[army.index()];
        let (pawn_moves, pawn_attacks) = compute_pawns_moves(&self.board, army);
        let pawn_attacks_filtered = pawn_attacks & enemy_mask;
        pawn_moves
            | pawn_attacks_filtered
            | compute_knights_moves(&self.board, army)
            | compute_bishops_moves(&self.board, army)
            | compute_rooks_moves(&self.board, army)
            | compute_queens_moves(&self.board, army)
            | compute_king_moves(&self.board, army)
    }

    pub fn is_square_attacked_by_army(&self, square: Square, army: Army) -> bool {
        if self.army_is_frozen(army) {
            return false;
        }
        let mask = 1u64 << square;
        let _enemy_mask = self.board.all_occupancy & !self.board.occupancy_by_army[army.index()];
        let (_pawn_moves, pawn_attacks) = compute_pawns_moves(&self.board, army);
        if pawn_attacks & mask != 0 {
            return true;
        }
        let king_moves = compute_king_moves(&self.board, army);
        if king_moves & mask != 0 {
            return true;
        }
        let knight_moves = compute_knights_moves(&self.board, army);
        if knight_moves & mask != 0 {
            return true;
        }
        let bishops_attacks = get_sliding_attacks(
            self.board.by_army_kind[army.index()][PieceKind::Bishop.index()],
            &crate::engine::moves::BISHOP_RAYS_DIRECTIONS,
            self.board.all_occupancy,
        );
        if bishops_attacks & mask != 0 {
            return true;
        }
        let rooks_attacks = get_sliding_attacks(
            self.board.by_army_kind[army.index()][PieceKind::Rook.index()],
            &crate::engine::moves::ROOK_RAYS_DIRECTIONS,
            self.board.all_occupancy,
        );
        if rooks_attacks & mask != 0 {
            return true;
        }
        let queens_attacks = get_sliding_attacks(
            self.board.by_army_kind[army.index()][PieceKind::Queen.index()],
            &crate::engine::moves::QUEEN_RAYS_DIRECTIONS,
            self.board.all_occupancy,
        );
        if queens_attacks & mask != 0 {
            return true;
        }
        false
    }

    pub fn is_square_attacked_by_team(&self, square: Square, team: Team) -> bool {
        for &army in team.armies().iter() {
            if self.is_square_attacked_by_army(square, army) {
                return true;
            }
        }
        false
    }

    pub fn king_in_check(&self, army: Army) -> bool {
        if let Some(square) = self.state.king_square(army) {
            self.is_square_attacked_by_team(square, army.team().opponent())
        } else {
            false
        }
    }

    pub fn must_move_king(&self, army: Army) -> bool {
        self.king_in_check(army) && self.generate_legal_non_king_moves(army).is_empty()
    }

    pub fn freeze_army(&mut self, army: Army) {
        self.board.set_frozen(army, true);
        self.state.set_frozen(army, true);
    }

    pub fn unfreeze_army(&mut self, army: Army) {
        self.board.set_frozen(army, false);
        self.state.set_frozen(army, false);
    }

    pub fn capture_king(&mut self, army: Army) {
        if let Some(square) = self.state.king_square(army) {
            self.board.clear_square(square);
        }
        self.freeze_army(army);
        self.state.set_king_square(army, None);
    }

    pub fn seize_throne_at(&mut self, army: Army, square: Square) {
        let team = army.team();
        for &ally in team.armies().iter() {
            if ally == army {
                continue;
            }
            if self.board.armies[ally.index()]
                .throne_squares
                .contains(&square)
            {
                let controller = self.board.controller_for(army);
                self.board.set_controller(ally, controller);
                self.unfreeze_army(ally);
            }
        }
    }

    pub fn winning_team(&self) -> Option<Team> {
        let air_kings = self.state.kings_alive(Team::Air);
        let earth_kings = self.state.kings_alive(Team::Earth);
        if earth_kings == 0 && air_kings > 0 {
            return Some(Team::Air);
        }
        if air_kings == 0 && earth_kings > 0 {
            return Some(Team::Earth);
        }
        None
    }

    pub fn draw_condition(&self) -> bool {
        let air_kings = self.state.kings_alive(Team::Air);
        let earth_kings = self.state.kings_alive(Team::Earth);
        if air_kings == 0 && earth_kings == 0 {
            return true;
        }
        if air_kings == 0 && earth_kings == 2 {
            return true;
        }
        if earth_kings == 0 && air_kings == 2 {
            return true;
        }
        false
    }

    pub fn piece_counts(&self, army: Army) -> [u32; PIECE_KIND_COUNT] {
        self.board.piece_counts(army)
    }

    pub fn is_privileged_pawn(&self, army: Army) -> bool {
        let counts = self.piece_counts(army);
        if counts[PieceKind::King.index()] == 0 || counts[PieceKind::Pawn.index()] == 0 {
            return false;
        }
        let queen = counts[PieceKind::Queen.index()];
        let bishop = counts[PieceKind::Bishop.index()];
        let knight = counts[PieceKind::Knight.index()];
        let rook = counts[PieceKind::Rook.index()];

        match (queen, bishop, knight, rook) {
            (1, 0, 0, 0) => true, // King + Queen + Pawn
            (0, 1, 0, 0) => true, // King + Bishop + Pawn
            (0, 0, 1, 0) => true, // King + Knight + Pawn
            (0, 0, 0, 1) => true, // King + Rook + Pawn
            (0, 0, 0, 0) => true, // King + Pawn
            _ => false,
        }
    }

    pub fn promotion_targets(&self, army: Army) -> Vec<PieceKind> {
        if self.is_privileged_pawn(army) {
            vec![
                PieceKind::Queen,
                PieceKind::Rook,
                PieceKind::Bishop,
                PieceKind::Knight,
            ]
        } else {
            vec![PieceKind::Queen]
        }
    }

    pub fn can_promote_at(&self, army: Army, square: Square) -> bool {
        let zone = self.board.promotion_zones[army.index()];
        (zone >> square) & 1 != 0
    }

    pub fn promote_pawn(&mut self, army: Army, pawn_square: Square, target: PieceKind) -> bool {
        let pawn_mask = 1u64 << pawn_square;
        let pawn_bits = self.board.by_army_kind[army.index()][PieceKind::Pawn.index()];
        if pawn_bits & pawn_mask == 0 {
            return false;
        }
        if !self.can_promote_at(army, pawn_square) {
            return false;
        }

        let target_kind = if self.is_privileged_pawn(army) {
            target
        } else {
            PieceKind::Queen
        };

        if target_kind == PieceKind::Pawn || target_kind == PieceKind::King {
            return false;
        }

        if self.board.by_army_kind[army.index()][target_kind.index()] != 0 {
            self.board.demote_piece_to_pawn(army, target_kind);
        }

        self.board.by_army_kind[army.index()][PieceKind::Pawn.index()] &= !pawn_mask;
        self.board.by_army_kind[army.index()][target_kind.index()] |= pawn_mask;
        self.board.refresh_occupancy();
        true
    }

    pub fn update_stalemate_status(&mut self, army: Army) {
        let king_in_check_status = self.king_in_check(army);
        if king_in_check_status {
            self.state.set_stalemate(army, false);
            return;
        }
        let legal_moves = self.generate_legal_moves(army);
        let stalemated = legal_moves.is_empty();
        self.state.set_stalemate(army, stalemated);
    }

    pub fn army_in_stalemate(&self, army: Army) -> bool {
        self.state.is_stalemated(army)
    }

    pub fn restore_king_to_throne(&mut self, army: Army) {
        let throne = self.board.armies[army.index()].throne_squares[0];
        self.board.clear_square(throne);
        self.board.place_piece(army, PieceKind::King, throne);
        self.state.set_king_square(army, Some(throne));
        self.unfreeze_army(army);
    }

    pub fn exchange_prisoners(&mut self, army_a: Army, army_b: Army) -> bool {
        if self.state.king_square(army_a).is_some() || self.state.king_square(army_b).is_some() {
            return false;
        }
        self.restore_king_to_throne(army_a);
        self.restore_king_to_throne(army_b);
        self.state.set_stalemate(army_a, false);
        self.state.set_stalemate(army_b, false);
        true
    }

    pub fn current_army(&self) -> Army {
        self.state.current_army(&self.config)
    }

    fn piece_moves(&self, army: Army, kind: PieceKind) -> u64 {
        match kind {
            PieceKind::King => compute_king_moves(&self.board, army),
            PieceKind::Queen => compute_queens_moves(&self.board, army),
            PieceKind::Rook => compute_rooks_moves(&self.board, army),
            PieceKind::Bishop => compute_bishops_moves(&self.board, army),
            PieceKind::Knight => compute_knights_moves(&self.board, army),
            PieceKind::Pawn => {
                let (forward_moves_bitboard, diagonal_attack_squares_bitboard) =
                    compute_pawns_moves(&self.board, army);

                let mut legal_pawn_moves = 0u64;

                // Add forward moves (only if empty)
                legal_pawn_moves |= forward_moves_bitboard & self.board.free;

                // Add diagonal capture moves (only if enemy piece is present)
                let enemy_occupancy =
                    self.board.all_occupancy & !self.board.occupancy_by_army[army.index()];
                legal_pawn_moves |= diagonal_attack_squares_bitboard & enemy_occupancy;

                legal_pawn_moves
            }
        }
    }

    pub fn generate_legal_king_moves(&self, army: Army) -> Vec<Move> {
        if self.army_is_frozen(army) {
            return Vec::new();
        }

        let mut legal_king_moves = Vec::new();
        if let Some(from_sq) = self.state.king_square(army) {
            let pseudo_legal_destinations = self.piece_moves(army, PieceKind::King);
            let mut destinations = pseudo_legal_destinations;

            while destinations != 0 {
                let to_sq = destinations.trailing_zeros() as Square;
                destinations &= destinations - 1;

                let mut next_game_state = self.clone();

                // Simulate the move on the cloned board
                // Save the piece at the destination square, if any, for later restoration if needed
                let original_piece_at_dest = next_game_state.board.piece_at(to_sq);

                next_game_state.board.clear_square(from_sq); // Clear the source square
                if let Some((target_army, target_kind)) = original_piece_at_dest {
                    // Remove the captured piece from the board before placing the moving piece
                    next_game_state
                        .board
                        .remove_piece(target_army, target_kind, to_sq);
                }
                next_game_state
                    .board
                    .place_piece(army, PieceKind::King, to_sq); // Place the piece on the destination square

                // Update king's square since king moved
                next_game_state.state.set_king_square(army, Some(to_sq));

                next_game_state.board.refresh_occupancy();
                next_game_state
                    .state
                    .sync_with_board(&next_game_state.board);

                if !next_game_state.king_in_check(army) {
                    legal_king_moves.push(Move {
                        from: from_sq,
                        to: to_sq,
                        kind: PieceKind::King,
                        promotion: None,
                    });
                }
            }
        }
        legal_king_moves
    }

    // New function to generate legal moves for non-king pieces
    pub fn generate_legal_non_king_moves(&self, army: Army) -> Vec<Move> {
        if self.army_is_frozen(army) {
            return Vec::new();
        }

        let mut legal_moves = Vec::new();
        for (from_sq, kind) in self.board.all_pieces_for_army(army) {
            if kind == PieceKind::King {
                continue; // Skip king moves
            }

            let pseudo_legal_destinations = self.piece_moves(army, kind);
            let mut destinations = pseudo_legal_destinations;

            while destinations != 0 {
                let to_sq = destinations.trailing_zeros() as Square;
                destinations &= destinations - 1;

                let mut next_game_state = self.clone();

                // Simulate the move on the cloned board
                let original_piece_at_dest = next_game_state.board.piece_at(to_sq);

                next_game_state.board.clear_square(from_sq);
                if let Some((target_army, target_kind)) = original_piece_at_dest {
                    next_game_state
                        .board
                        .remove_piece(target_army, target_kind, to_sq);
                }
                next_game_state.board.place_piece(army, kind, to_sq);

                if kind == PieceKind::Pawn && next_game_state.can_promote_at(army, to_sq) {
                    next_game_state.promote_pawn(army, to_sq, PieceKind::Queen);
                }

                next_game_state.board.refresh_occupancy();
                next_game_state
                    .state
                    .sync_with_board(&next_game_state.board);

                if !next_game_state.king_in_check(army) {
                    legal_moves.push(Move {
                        from: from_sq,
                        to: to_sq,
                        kind,
                        promotion: None,
                    });
                }
            }
        }
        legal_moves
    }

    pub fn generate_legal_moves(&self, army: Army) -> Vec<Move> {
        if self.army_is_frozen(army) {
            return Vec::new();
        }

        let mut legal_moves = Vec::new();

        // Add legal non-king moves
        legal_moves.extend(self.generate_legal_non_king_moves(army));

        // Add legal king moves
        legal_moves.extend(self.generate_legal_king_moves(army));

        legal_moves
    }

    pub fn apply_move(
        &mut self,
        army: Army,
        from: Square,
        to: Square,
        promotion: Option<PieceKind>,
    ) -> Result<String, String> {
        if army != self.current_army() {
            return Err(format!("It is not {}'s turn", army.display_name()));
        }

        let piece = self
            .board
            .piece_at(from)
            .ok_or_else(|| "No piece on source square".to_string())?;
        if piece.0 != army {
            return Err("Source square does not belong to the current army".to_string());
        }
        let piece_kind = piece.1;

        // If the king is in check and a non-king piece is trying to move, check if a non-king move can resolve the check.
        // If no non-king moves can resolve the check, then the king must move.
        if self.king_in_check(army) && piece_kind != PieceKind::King {
            let non_king_resolving_moves = self.generate_legal_non_king_moves(army);
            if non_king_resolving_moves.is_empty() {
                return Err("King must move while in check".to_string());
            }
        }

        let allowed = self.piece_moves(army, piece_kind);
        let dest_mask = 1u64 << to;
        if allowed & dest_mask == 0 {
            return Err("Destination is not a legal move".to_string());
        }

        if let Some((target_army, target_kind)) = self.board.piece_at(to) {
            if target_army == army {
                return Err("Cannot capture own piece".to_string());
            }
            if target_kind == PieceKind::King {
                self.capture_king(target_army);
            } else {
                self.board.remove_piece(target_army, target_kind, to);
            }
        }

        self.board.move_piece(army, piece_kind, from, to);
        if piece_kind == PieceKind::King {
            self.state.set_king_square(army, Some(to));
            self.seize_throne_at(army, to);
        }

        if piece_kind == PieceKind::Pawn && self.can_promote_at(army, to) {
            let target = promotion.unwrap_or(PieceKind::Queen);
            if !self.promote_pawn(army, to, target) {
                return Err("Promotion failed".to_string());
            }
        }

        self.state.sync_with_board(&self.board);
        for &other in Army::ALL.iter() {
            self.update_stalemate_status(other);
        }
        self.advance_to_next_army();

        Ok(format!(
            "{} moved {} to {}",
            army.display_name(),
            Self::piece_name(piece_kind),
            Self::square_notation(to)
        ))
    }

    fn advance_to_next_army(&mut self) {
        for _ in 0..self.config.turn_order.len() {
            self.state.advance_turn(&self.config);
            let candidate = self.state.current_army(&self.config);
            if !self.state.army_frozen[candidate.index()] && !self.state.is_stalemated(candidate) {
                break;
            }
        }
    }

    fn piece_name(kind: PieceKind) -> &'static str {
        match kind {
            PieceKind::King => "King",
            PieceKind::Queen => "Queen",
            PieceKind::Rook => "Rook",
            PieceKind::Bishop => "Bishop",
            PieceKind::Knight => "Knight",
            PieceKind::Pawn => "Pawn",
        }
    }

    fn square_notation(square: Square) -> String {
        let file = (square % 8) as u8;
        let rank = (square / 8) as u8;
        format!("{}{}", (b'a' + file) as char, rank + 1)
    }
}

impl Default for Game {
    fn default() -> Game {
        Self::from_array_spec(&TABLET_OF_FIRE_PROTOTYPE)
    }
}
