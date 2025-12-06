use serde::{Deserialize, Serialize};
use crate::engine::types::{Army, PieceKind, PlayerId, Square, Piece, ARMY_COUNT};
use crate::engine::game::{Game, GameConfig, Mode};
use crate::engine::board::{Board, ArmyState, OverlayPiece, DEFAULT_PROMOTION_ZONES};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnochFen {
    pub turn_order: Vec<Army>,
    pub current_turn_index: usize,
    pub army_states: Vec<ArmyStateFen>,
    pub pieces: Vec<PiecePlacement>,
    pub promotion_zones: Vec<u64>,
    pub mode: Mode,
    pub divination_die: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArmyStateFen {
    pub army: Army,
    pub controller: PlayerId,
    pub throne_squares: [Square; 2],
    pub is_frozen: bool,
    pub is_stalemated: bool,
    pub king_square: Option<Square>,
    /// Throne overlay pieces (one per throne square, indexed 0 and 1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throne_overlay: Option<[Option<OverlayPieceFen>; 2]>,
}

/// Serializable representation of an overlay piece
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OverlayPieceFen {
    pub army: Army,
    pub kind: PieceKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PiecePlacement {
    pub square: Square,
    pub army: Army,
    pub kind: PieceKind,
}

impl EnochFen {
    pub fn from_game(game: &Game) -> Self {
        let mut pieces = Vec::new();
        for sq in 0..64 {
            if let Some((army, kind)) = game.board.piece_at(sq) {
                pieces.push(PiecePlacement {
                    square: sq,
                    army,
                    kind,
                });
            }
        }

        let mut army_states = Vec::new();
        for army in Army::ALL {
            let idx = army.index();

            // Serialize throne overlay if any pieces exist
            let overlay = &game.board.throne_overlay[idx];
            let throne_overlay = if overlay[0].is_some() || overlay[1].is_some() {
                Some([
                    overlay[0].map(|o| OverlayPieceFen { army: o.army, kind: o.kind }),
                    overlay[1].map(|o| OverlayPieceFen { army: o.army, kind: o.kind }),
                ])
            } else {
                None
            };

            army_states.push(ArmyStateFen {
                army,
                controller: game.board.controller_for(army),
                throne_squares: game.board.armies[idx].throne_squares,
                is_frozen: game.state.army_frozen[idx],
                is_stalemated: game.state.stalemated_armies[idx],
                king_square: game.state.king_square(army),
                throne_overlay,
            });
        }

        EnochFen {
            turn_order: game.config.turn_order.to_vec(),
            current_turn_index: game.state.current_turn_index,
            army_states,
            pieces,
            promotion_zones: game.board.promotion_zones.to_vec(),
            mode: game.config.mode,
            divination_die: game.state.divination_die,
        }
    }

    pub fn into_game(self) -> Result<Game, String> {
        if self.turn_order.len() != 4 {
            return Err("Invalid turn_order length".to_string());
        }
        let mut turn_order = [Army::Blue; 4];
        for (i, a) in self.turn_order.iter().enumerate() {
            turn_order[i] = *a;
        }

        let mut placements = Vec::new();
        for p in self.pieces {
            placements.push((
                p.army,
                Piece {
                    army: p.army,
                    kind: p.kind,
                    pawn_type: None,
                },
                1u64 << p.square,
            ));
        }

        let mut army_states_array = [ArmyState::new(Army::Blue, [0,0], PlayerId::PLAYER_ONE); 4];
        let mut promotion_zones = [0u64; 4];
        if self.promotion_zones.len() == 4 {
             promotion_zones.copy_from_slice(&self.promotion_zones);
        } else {
             promotion_zones = DEFAULT_PROMOTION_ZONES;
        }

        let mut controller_map = [PlayerId::PLAYER_ONE; 4];

        for state in &self.army_states {
            let idx = state.army.index();
            army_states_array[idx] = ArmyState {
                army: state.army,
                throne_squares: state.throne_squares,
                controller: state.controller,
                is_frozen: state.is_frozen,
            };
            controller_map[idx] = state.controller;
        }

        let board = Board::with_state(&placements, army_states_array, promotion_zones);
        
        let config = GameConfig {
            armies: Army::ALL,
            turn_order,
            controller_map,
            mode: self.mode,
        };

        let mut game = Game::with_config(board, config);

        game.state.current_turn_index = self.current_turn_index;
        game.state.divination_die = self.divination_die;
        for state in &self.army_states {
            game.state.set_stalemate(state.army, state.is_stalemated);
            game.state.set_king_square(state.army, state.king_square);

            // Restore throne overlay pieces
            if let Some(overlay) = &state.throne_overlay {
                let idx = state.army.index();
                game.board.throne_overlay[idx] = [
                    overlay[0].as_ref().map(|o| OverlayPiece { army: o.army, kind: o.kind }),
                    overlay[1].as_ref().map(|o| OverlayPiece { army: o.army, kind: o.kind }),
                ];
            }
        }
        
        Ok(game)
    }
}
