use serde::{Deserialize, Serialize};
use crate::engine::arrays::{ArraySpec, default_array};
use crate::engine::board::{Board, OverlayPiece};
use crate::engine::fen::EnochFen;
use crate::engine::moves::{
    compute_bishops_moves, compute_king_moves, compute_knights_moves, compute_pawns_moves,
    compute_queens_moves, compute_rooks_moves,
};
use crate::engine::piece_kind::{parse_move, ParsedMove, SpecialMove};
use crate::engine::types::{Army, PieceKind, PlayerId, Square, Team, ARMY_COUNT, PIECE_KIND_COUNT};
use rand::Rng;

/// Game struct responsible for all game logics (pin, check, valid captures, etc)
#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub config: GameConfig,
    pub state: GameState,
    pub status: Status,
}

impl Game {
    pub fn to_enoch_fen(&self) -> String {
        let fen = EnochFen::from_game(self);
        serde_json::to_string_pretty(&fen).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn from_enoch_fen(json: &str) -> Result<Game, String> {
        let fen: EnochFen = serde_json::from_str(json).map_err(|e| e.to_string())?;
        fen.into_game()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    Divination,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub armies: [Army; ARMY_COUNT],
    pub turn_order: [Army; ARMY_COUNT],
    pub controller_map: [PlayerId; ARMY_COUNT],
    pub mode: Mode,
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
            mode: Mode::default(),
        }
    }
}

/// Result of detecting a Concourse formation (Rules 5.10-5.11)
#[derive(Debug, Clone)]
pub struct ConcourseResult {
    pub piece_kind: PieceKind,
    pub enemy1: (Square, Army),
    pub enemy2: (Square, Army),
    pub ally: (Square, Army),
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_turn_index: usize,
    pub army_frozen: [bool; ARMY_COUNT],
    pub king_positions: [Option<Square>; ARMY_COUNT],
    pub stalemated_armies: [bool; ARMY_COUNT],
    pub divination_die: Option<u8>,
    /// Armies that have withdrawn from the game (Rule 9.1-9.3).
    /// A withdrawn army's pieces are controlled by the ally, but the army itself
    /// is marked as withdrawn for turn order purposes.
    pub withdrawn_armies: [bool; ARMY_COUNT],
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_turn_index: 0,
            army_frozen: [false; ARMY_COUNT],
            king_positions: [None; ARMY_COUNT],
            stalemated_armies: [false; ARMY_COUNT],
            divination_die: None,
            withdrawn_armies: [false; ARMY_COUNT],
        }
    }

    pub fn sync_with_board(&mut self, board: &Board) {
        for army in Army::ALL {
            self.army_frozen[army.index()] = board.is_army_frozen(army);
            self.king_positions[army.index()] = board.king_square(army);
            self.stalemated_armies[army.index()] = false;
        }
        // Preserve divination_die
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

    pub fn is_withdrawn(&self, army: Army) -> bool {
        self.withdrawn_armies[army.index()]
    }

    pub fn set_withdrawn(&mut self, army: Army, withdrawn: bool) {
        self.withdrawn_armies[army.index()] = withdrawn;
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

#[derive(Debug, PartialEq, Clone, Copy)]
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

    /// Check if we're in 4-player mode (each player controls one army).
    /// In 2-player mode, allies share a controller; in 4-player mode, each army has its own.
    pub fn is_four_player_mode(&self) -> bool {
        // In 4-player mode, Blue and Black have different controllers, and Red and Yellow have different controllers
        let blue_ctrl = self.config.controller_map[Army::Blue.index()];
        let black_ctrl = self.config.controller_map[Army::Black.index()];
        let red_ctrl = self.config.controller_map[Army::Red.index()];
        let yellow_ctrl = self.config.controller_map[Army::Yellow.index()];

        // If any ally pair has different controllers, it's 4-player mode
        blue_ctrl != black_ctrl || red_ctrl != yellow_ctrl
    }

    /// Ally captures their teammate's king (Rule 11.3).
    /// The ally gains control of both elemental sides without freezing pieces.
    /// The captured king is removed, but pieces remain active under ally's control.
    pub fn ally_capture_king(&mut self, capturing_army: Army, captured_army: Army) {
        // Remove the king from the board
        if let Some(square) = self.state.king_square(captured_army) {
            // If king was on own throne with overlay piece, clear the overlay too
            if let Some(throne_idx) = self.board.throne_index_for(captured_army, square) {
                self.board.clear_throne_overlay(captured_army, throne_idx);
            }
            self.board.clear_square(square);
        }
        self.state.set_king_square(captured_army, None);

        // Transfer control to the capturing ally (not freeze!)
        let controller = self.board.controller_for(capturing_army);
        self.board.set_controller(captured_army, controller);
        // Pieces are NOT frozen - they remain active under ally's control
    }

    /// Withdraw an army from the game (Rules 9.1-9.3).
    ///
    /// In 4-player mode (Rule 9.1-9.2):
    /// - Army's pieces are transferred to ally's control
    /// - Army is marked as withdrawn
    /// - If `bare_king` is true (Rule 9.1), the ally takes both turns and controls the King
    ///
    /// In 2-player mode (Rule 9.3):
    /// - Only applies when withdrawing an army that has only its king left
    /// - The withdrawn army's king becomes frozen
    /// - The withdrawing player loses that army's turn
    ///
    /// Returns an error if:
    /// - The army is already frozen or withdrawn
    /// - In 2-player mode, trying to withdraw an army that isn't reduced to just a king
    pub fn withdraw_army(&mut self, army: Army) -> Result<(), &'static str> {
        // Can't withdraw an already-frozen or withdrawn army
        if self.army_is_frozen(army) {
            return Err("Cannot withdraw: army is already frozen");
        }
        if self.state.is_withdrawn(army) {
            return Err("Cannot withdraw: army has already withdrawn");
        }

        let ally = army.ally();
        let ally_controller = self.board.controller_for(ally);

        if self.is_four_player_mode() {
            // Rule 9.1 & 9.2: Transfer control to ally
            self.board.set_controller(army, ally_controller);
            self.state.set_withdrawn(army, true);
            // Note: Turn handling for "both turns" (Rule 9.1) should be managed by
            // the turn advancement logic, checking is_withdrawn() and bare_king status
        } else {
            // Rule 9.3 (2-player mode): Can only withdraw if reduced to just king
            // Check if army has only its king left
            let piece_counts = self.board.piece_counts(army);
            let total_pieces: u32 = piece_counts.iter().sum();
            let has_only_king = total_pieces == 1 && piece_counts[PieceKind::King.index()] == 1;

            if !has_only_king {
                return Err("In 2-player mode, can only withdraw an army reduced to just its king");
            }

            // Freeze the army's king and mark as withdrawn
            self.state.set_frozen(army, true);
            self.state.set_withdrawn(army, true);
            // Note: The withdrawing player loses this army's turn
        }

        Ok(())
    }

    /// Check if an army can withdraw (has valid withdrawal conditions)
    pub fn can_withdraw(&self, army: Army) -> bool {
        if self.army_is_frozen(army) || self.state.is_withdrawn(army) {
            return false;
        }

        if self.is_four_player_mode() {
            // In 4-player mode, any non-frozen army can withdraw at any time
            true
        } else {
            // In 2-player mode, can only withdraw if reduced to just king
            let piece_counts = self.board.piece_counts(army);
            let total_pieces: u32 = piece_counts.iter().sum();
            total_pieces == 1 && piece_counts[PieceKind::King.index()] == 1
        }
    }

    /// Check if a move to `square` by `moving_army` completes a Concourse formation (Rules 5.10-5.11).
    /// A Concourse occurs when 4 Bishops or 4 Queens occupy a 2×2 square.
    /// Returns Some((enemy1, enemy2, ally)) if a concourse is detected, where:
    /// - enemy1, enemy2 are enemy piece positions to capture
    /// - ally is the ally piece position whose control is gained
    pub fn detect_concourse(&self, moving_army: Army, square: Square, piece_kind: PieceKind) -> Option<ConcourseResult> {
        // Only bishops and queens can form a concourse
        if piece_kind != PieceKind::Bishop && piece_kind != PieceKind::Queen {
            return None;
        }

        let file = (square % 8) as i8;
        let rank = (square / 8) as i8;

        // Check all 2×2 formations that include the destination square
        // A 2×2 has corners at (r, c), (r+1, c), (r, c+1), (r+1, c+1)
        // The destination could be any of these 4 corners
        let offsets: [(i8, i8); 4] = [
            (0, 0),   // destination is bottom-left
            (-1, 0),  // destination is top-left
            (0, -1),  // destination is bottom-right
            (-1, -1), // destination is top-right
        ];

        for (dr, dc) in offsets {
            let base_rank = rank + dr;
            let base_file = file + dc;

            // Check bounds for the entire 2×2
            if base_rank < 0 || base_rank > 6 || base_file < 0 || base_file > 6 {
                continue;
            }

            let corners: [Square; 4] = [
                (base_rank as u8 * 8 + base_file as u8),           // bottom-left
                ((base_rank as u8 + 1) * 8 + base_file as u8),     // top-left
                (base_rank as u8 * 8 + (base_file as u8 + 1)),     // bottom-right
                ((base_rank as u8 + 1) * 8 + (base_file as u8 + 1)), // top-right
            ];

            // Check if all 4 corners have pieces of the same kind
            let mut pieces: Vec<(Square, Army)> = Vec::new();
            let mut all_same_kind = true;

            for &corner in &corners {
                if let Some((army, kind)) = self.board.piece_at(corner) {
                    if kind == piece_kind {
                        pieces.push((corner, army));
                    } else {
                        all_same_kind = false;
                        break;
                    }
                } else if corner != square {
                    // Empty square that's not our destination
                    all_same_kind = false;
                    break;
                } else {
                    // This is the destination square - count the moving piece
                    pieces.push((corner, moving_army));
                }
            }

            if !all_same_kind || pieces.len() != 4 {
                continue;
            }

            // We have 4 pieces of the same kind in a 2×2!
            // Identify enemies vs ally
            let moving_team = moving_army.team();
            let mut enemies: Vec<(Square, Army)> = Vec::new();
            let mut allies: Vec<(Square, Army)> = Vec::new();

            for (sq, army) in pieces {
                if sq == square {
                    continue; // Skip the moving piece
                }
                if army.team() == moving_team {
                    allies.push((sq, army));
                } else {
                    enemies.push((sq, army));
                }
            }

            // A valid concourse has exactly 2 enemies and 1 ally
            if enemies.len() == 2 && allies.len() == 1 {
                return Some(ConcourseResult {
                    piece_kind,
                    enemy1: enemies[0],
                    enemy2: enemies[1],
                    ally: allies[0],
                });
            }
        }

        None
    }

    /// Apply the effects of a Concourse formation (Rules 5.10-5.11).
    /// Captures the two enemy pieces and transfers control of the ally piece.
    pub fn apply_concourse(&mut self, moving_army: Army, result: &ConcourseResult) -> String {
        let (enemy1_sq, enemy1_army) = result.enemy1;
        let (enemy2_sq, enemy2_army) = result.enemy2;
        let (_ally_sq, ally_army) = result.ally;

        // Capture the two enemy pieces
        self.board.remove_piece(enemy1_army, result.piece_kind, enemy1_sq);
        self.board.remove_piece(enemy2_army, result.piece_kind, enemy2_sq);

        // Transfer control of ally's army to the moving player
        // Note: The rule says "takes control of the ally Bishop" - this could mean:
        // 1. Just control of that specific piece's moves, or
        // 2. Control of the entire ally army
        // Based on Enochian Chess conventions, we'll interpret this as gaining control
        // of the ally army (similar to seizing a throne)
        let controller = self.board.controller_for(moving_army);
        self.board.set_controller(ally_army, controller);

        format!(
            "Concourse of {}! {} captures enemy {} at {} and {}, gains control of {} {}",
            if result.piece_kind == PieceKind::Bishop { "Bishoping" } else { "Queens" },
            moving_army.display_name(),
            if result.piece_kind == PieceKind::Bishop { "Bishops" } else { "Queens" },
            Self::square_notation(enemy1_sq),
            Self::square_notation(enemy2_sq),
            ally_army.display_name(),
            if result.piece_kind == PieceKind::Bishop { "Bishop" } else { "Queen" },
        )
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
        let pawn_attacks = pawn_attacks & enemy_mask;
        pawn_moves
            | pawn_attacks
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
        let enemy_mask = self.board.all_occupancy & !self.board.occupancy_by_army[army.index()];
        let (_, pawn_attacks) = compute_pawns_moves(&self.board, army);
        let pawn_capture_mask = pawn_attacks & enemy_mask;
        if pawn_capture_mask & mask != 0 {
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
        if compute_bishops_moves(&self.board, army) & mask != 0 {
            return true;
        }
        if compute_rooks_moves(&self.board, army) & mask != 0 {
            return true;
        }
        if compute_queens_moves(&self.board, army) & mask != 0 {
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
        self.king_in_check(army) && self.king_moves_bitboard(army) != 0
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
            // If king was on own throne with overlay piece, clear the overlay too
            // (both pieces are captured when double-occupied throne is taken)
            if let Some(throne_idx) = self.board.throne_index_for(army, square) {
                self.board.clear_throne_overlay(army, throne_idx);
            }
            self.board.clear_square(square);
        }

        // Rule 8.9: If captured army was controlling its ally, revert ally control
        // to original controller
        self.revert_controlled_armies(army);

        self.freeze_army(army);
        self.state.set_king_square(army, None);
    }

    /// Rule 8.9: When a king is captured, if that army was controlling its ally,
    /// revert the ally's controller back to the original controller.
    fn revert_controlled_armies(&mut self, captured_army: Army) {
        let captured_controller = self.board.controller_for(captured_army);
        let team = captured_army.team();

        for &ally in team.armies().iter() {
            if ally == captured_army {
                continue;
            }

            // Check if the captured army was controlling this ally
            let ally_controller = self.board.controller_for(ally);
            if ally_controller == captured_controller {
                // Revert to original controller from config
                let original = self.config.controller_map[ally.index()];
                self.board.set_controller(ally, original);
            }
        }
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
        let no_secondary = knight == 0 && rook == 0;

        match (queen, bishop) {
            (1, 0) if no_secondary => true,
            (0, 1) if no_secondary => true,
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

    /// Check if a pawn can be promoted at a given square.
    /// Per Rule 10.1a-b: Promotion only occurs if the player has already lost one or more pawns.
    /// If a player still controls four pawns, promotion is delayed until a pawn is lost.
    pub fn can_promote_at(&self, army: Army, square: Square) -> bool {
        let zone = self.board.promotion_zones[army.index()];
        let in_zone = (zone >> square) & 1 != 0;
        if !in_zone {
            return false;
        }

        // Rule 10.1a-b: Must have lost at least one pawn to promote
        // (i.e., current pawn count must be < 4)
        let pawn_count = self.board.piece_counts(army)[PieceKind::Pawn.index()];
        pawn_count < 4
    }

    /// Check if the army can promote any pawn (has lost at least one pawn).
    pub fn can_promote_pawns(&self, army: Army) -> bool {
        let pawn_count = self.board.piece_counts(army)[PieceKind::Pawn.index()];
        pawn_count < 4
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

        // Determine the promotion target
        let target_kind = if self.is_privileged_pawn(army) {
            // Privileged pawns can choose any major piece
            target
        } else {
            // Normal pawns promote to their patron piece
            // Get patron from piece_map, fall back to Queen if no patron specified
            self.board
                .get_piece(pawn_square)
                .and_then(|p| p.pawn_type)
                .unwrap_or(PieceKind::Queen)
        };

        if target_kind == PieceKind::Pawn || target_kind == PieceKind::King {
            return false;
        }

        // If the target piece type already exists, demote it to a pawn
        if self.board.by_army_kind[army.index()][target_kind.index()] != 0 {
            self.board.demote_piece_to_pawn(army, target_kind);
        }

        // Remove the pawn and add the promoted piece
        self.board.by_army_kind[army.index()][PieceKind::Pawn.index()] &= !pawn_mask;
        self.board.by_army_kind[army.index()][target_kind.index()] |= pawn_mask;

        // Update piece_map with the promoted piece (preserving diagonal system if applicable)
        let diagonal_system = match target_kind {
            PieceKind::Queen | PieceKind::Bishop => {
                Some(crate::engine::board::diagonal_system_for_square(pawn_square))
            }
            _ => None,
        };
        self.board.piece_map.insert(pawn_square, crate::engine::types::Piece {
            army,
            kind: target_kind,
            pawn_type: None,
            diagonal_system,
        });

        self.board.refresh_occupancy();
        true
    }

    pub fn update_stalemate_status(&mut self, army: Army) {
        if self.king_in_check(army) {
            self.state.set_stalemate(army, false);
            return;
        }
        
        // Check if any geometric king move is safe
        let geometric_king_moves = self.king_moves_bitboard(army);
        let mut safe_king_moves = 0u64;
        let opponent = army.team().opponent();
        
        let mut mask = geometric_king_moves;
        while mask != 0 {
            let sq = mask.trailing_zeros() as Square;
            // Note: We verify if the DESTINATION is attacked.
            if !self.is_square_attacked_by_team(sq, opponent) {
                safe_king_moves |= 1u64 << sq;
            }
            mask &= mask - 1;
        }

        // Non-king moves
        let all_moves = self.army_moves_bitboard(army);
        let non_king_moves = all_moves & !geometric_king_moves;
        
        let stalemated = safe_king_moves == 0 && non_king_moves == 0;
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

    pub fn piece_moves(&self, army: Army, kind: PieceKind) -> u64 {
        match kind {
            PieceKind::King => compute_king_moves(&self.board, army),
            PieceKind::Queen => compute_queens_moves(&self.board, army),
            PieceKind::Rook => compute_rooks_moves(&self.board, army),
            PieceKind::Bishop => compute_bishops_moves(&self.board, army),
            PieceKind::Knight => compute_knights_moves(&self.board, army),
            PieceKind::Pawn => {
                let (moves, attacks) = compute_pawns_moves(&self.board, army);
                moves | attacks
            }
        }
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

        if self.config.mode == Mode::Divination {
            if let Some(die) = self.state.divination_die {
                if !self.is_allowed_by_die(piece_kind, die) {
                    return Err(format!(
                        "Die roll {} requires {}",
                        die,
                        self.die_piece_name(die)
                    ));
                }
            }
        }

        if self.must_move_king(army) && piece_kind != PieceKind::King {
            return Err("King must move while in check".to_string());
        }

        let allowed = self.piece_moves(army, piece_kind);
        let dest_mask = 1u64 << to;
        if allowed & dest_mask == 0 {
            return Err("Destination is not a legal move".to_string());
        }

        // Check if moving to a throne with an ally's king (double-occupancy)
        let throne_overlay_target = self.board.is_king_occupied_throne(to);

        if let Some((target_army, target_kind)) = self.board.piece_at(to) {
            // Check for throne double-occupancy: same team king on their own throne
            if let Some((throne_army, throne_idx)) = throne_overlay_target {
                if target_army.team() == army.team() && target_kind == PieceKind::King {
                    // This is a valid throne overlay move - store the moving piece in overlay
                    // Don't remove the king, just store our piece in the overlay
                    self.board.remove_piece(army, piece_kind, from);
                    self.board.set_throne_overlay(
                        throne_army,
                        throne_idx,
                        OverlayPiece { army, kind: piece_kind },
                    );

                    self.state.sync_with_board(&self.board);
                    for &other in Army::ALL.iter() {
                        self.update_stalemate_status(other);
                    }
                    self.advance_to_next_army();

                    return Ok(format!(
                        "{} moved {} to share {}'s throne",
                        army.display_name(),
                        Self::piece_name(piece_kind),
                        target_army.display_name()
                    ));
                }
            }

            if target_army == army {
                return Err("Cannot capture own piece".to_string());
            }

            // Check for ally captures
            let is_ally = target_army.team() == army.team();
            if is_ally && target_kind != PieceKind::King {
                // Rule 11.2: Cannot capture ally's non-king pieces
                return Err("Cannot capture ally's piece".to_string());
            }

            if target_kind == PieceKind::King {
                if is_ally {
                    // Rule 11.3: Ally can capture ally's king in 4-player mode
                    if !self.is_four_player_mode() {
                        return Err("Cannot capture ally's king in 2-player mode".to_string());
                    }
                    // Ally captures ally's king - transfer control without freezing
                    self.ally_capture_king(army, target_army);
                } else {
                    // Enemy captures king - freeze the army
                    self.capture_king(target_army);
                }
            } else {
                self.board.remove_piece(target_army, target_kind, to);
            }
        }

        // Check if king is leaving a throne with an overlay piece
        if piece_kind == PieceKind::King {
            if let Some(throne_idx) = self.board.king_on_own_throne(army) {
                // Restore overlay piece to the board before moving the king
                if let Some(overlay) = self.board.clear_throne_overlay(army, throne_idx) {
                    self.board.place_piece(overlay.army, overlay.kind, from);
                }
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

        // Check for Concourse formation (Rules 5.10-5.11)
        let concourse_msg = if let Some(concourse) = self.detect_concourse(army, to, piece_kind) {
            Some(self.apply_concourse(army, &concourse))
        } else {
            None
        };

        self.state.sync_with_board(&self.board);
        for &other in Army::ALL.iter() {
            self.update_stalemate_status(other);
        }
        self.advance_to_next_army();

        // Build the return message
        let base_msg = format!(
            "{} moved {} to {}",
            army.display_name(),
            Self::piece_name(piece_kind),
            Self::square_notation(to)
        );

        if let Some(concourse) = concourse_msg {
            Ok(format!("{}. {}", base_msg, concourse))
        } else {
            Ok(base_msg)
        }
    }

    fn advance_to_next_army(&mut self) {
        for _ in 0..self.config.turn_order.len() {
            self.state.advance_turn(&self.config);
            let candidate = self.state.current_army(&self.config);
            if !self.state.army_frozen[candidate.index()] && !self.state.is_stalemated(candidate) {
                if self.config.mode == Mode::Divination {
                    self.roll_divination_die_for_turn(candidate);
                }
                break;
            }
        }
    }

    fn roll_divination_die_for_turn(&mut self, army: Army) {
        let mut rng = rand::thread_rng();
        // Loop until we find a die roll that allows at least one move.
        // To prevent infinite loop if NO moves possible (which should be stalemate, but maybe not detected yet?),
        // we cap iterations. But theoretically if not stalemated, there is a move.
        // Since we have 6 die types, and at least one piece can move, eventually we hit it.
        // Max pieces = 16.
        
        for _ in 0..1000 {
            let roll = rng.gen_range(1..=6);
            self.state.divination_die = Some(roll);
            if self.has_move_for_die(army, roll) {
                return;
            }
        }
        // If we failed 1000 times, something is wrong or very unlucky.
        // Fallback: Just leave the last roll.
    }

    fn has_move_for_die(&self, army: Army, die: u8) -> bool {
        let allowed_kinds = match die {
            1 => vec![PieceKind::King, PieceKind::Pawn],
            2 => vec![PieceKind::Knight],
            3 => vec![PieceKind::Bishop],
            4 => vec![PieceKind::Queen],
            5 => vec![PieceKind::Rook],
            6 => vec![PieceKind::Pawn],
            _ => return false,
        };

        for kind in allowed_kinds {
            if self.must_move_king(army) && kind != PieceKind::King {
                continue;
            }
            let moves = self.piece_moves(army, kind);
            if moves != 0 {
                return true;
            }
        }
        false
    }

    fn is_allowed_by_die(&self, kind: PieceKind, die: u8) -> bool {
        match die {
            1 => matches!(kind, PieceKind::King | PieceKind::Pawn),
            2 => kind == PieceKind::Knight,
            3 => kind == PieceKind::Bishop,
            4 => kind == PieceKind::Queen,
            5 => kind == PieceKind::Rook,
            6 => kind == PieceKind::Pawn,
            _ => true,
        }
    }

    fn die_piece_name(&self, die: u8) -> &'static str {
        match die {
            1 => "King or Pawn",
            2 => "Knight",
            3 => "Bishop",
            4 => "Queen",
            5 => "Rook",
            6 => "Pawn",
            _ => "Unknown",
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
        Self::from_array_spec(default_array())
    }
}
