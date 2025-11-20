# Enochian Chess Architecture

This document outlines the current architecture of the chess engine and the proposed changes to adapt it for Enochian chess.

## Current Architecture

The existing codebase is a standard FIDE chess engine with the following key components:

*   **`src/main.rs`**: The entry point of the application, responsible for initializing the game and the TUI.
*   **`src/engine/`**: Contains the core chess logic.
    *   **`board.rs`**: Defines the `Board` struct, which uses bitboards to represent the chess board and the pieces. The representation is hardcoded for two players (white and black) and standard FIDE pieces.
    *   **`game.rs`**: Defines the `Game` struct, which manages the game state, including turns, castling rights, checks, and game status (ongoing, checkmate, draw). The logic is tightly coupled to FIDE rules.
    *   **`moves.rs`**: Implements move generation for each piece type based on FIDE rules. It uses precomputed move tables and rays for performance.
    *   **`parser.rs`**: Parses PGN notation for moves.
*   **`src/ui/`**: Contains the terminal user interface code.
    *   **`app.rs`**: Manages the application state for the UI.
    *   **`ui.rs`**: Renders the TUI, including the board, pieces, and game information.

### Hardcoded FIDE Rules and Two-Player Logic

The current implementation has several parts that are hardcoded for a two-player FIDE chess game:

*   **`src/engine/board.rs`**: The `Board` struct has separate bitboards for `white_pawns`, `black_knights`, etc.
*   **`src/engine/game.rs`**: The `Game` struct assumes two players, with methods like `is_white()` to determine the current player. It implements FIDE-specific rules like castling, en passant, and checkmate.
*   **`src/engine/moves.rs`**: The move generation functions are all based on standard FIDE piece movements.
*   **`src/ui/ui.rs`**: The TUI is designed to render a two-player chess game.

## Refactoring Plan for Enochian Chess

To transform the engine to support Enochian chess, the following changes will be necessary:

### 1. Core Data Model (`src/engine/board.rs`, `src/engine/game.rs`)

*   **Armies and Teams:** Replace the `white`/`black` concept with four armies (Blue, Black, Red, Yellow) and two teams (Air, Earth). This will require introducing new enums, `Army` and `Team`.
*   **Board Representation:** The `Board` struct will be updated to use a four-dimensional array of bitboards `by_army_kind: [[u64; 6]; 4]` to represent the pieces of the four armies.
*   **Game State:** The `Game` struct will be modified to handle a four-player turn order and the unique game states of Enochian chess, such as frozen armies. FIDE-specific state like castling rights and en passant squares will be removed.

### 2. Move Generation (`src/engine/moves.rs`)

*   The move generation logic for each piece will be rewritten to match the Enochian rules:
    *   **Queen:** Implement the two-square leap.
    *   **Bishop/Queen Interaction:** Implement the "Concourse of Bishoping" rules, where Bishops and Queens have special capture restrictions.
    *   **Pawn:** Remove the initial double move and en passant. Implement the "pawn of X" promotion mechanic.

### 3. Game Rules (`src/engine/game.rs`)

*   Implement the following Enochian rules:
    *   **King Capture:** Kings can be captured.
    *   **Frozen Pieces:** Armies with captured kings are frozen.
    *   **Seizing the Throne:** Kings can take control of allied armies.
    *   **Exchange of Prisoners:** Captured kings can be exchanged.
    *   **Privileged Pawn:** Special promotion rules for pawns.
    *   **Stalemate:** The unique stalemate condition where a player skips turns.

### 4. User Interface (`src/ui/`)

*   The TUI will be updated to:
    *   Render a four-colored board representing the four armies.
    *   Display game state information relevant to Enochian chess (e.g., frozen armies, current turn).
    *   Handle the new move notation (e.g., `blue: e2-e4`).

## Module Relationships (current state)

```
┌────────┐      ┌────────┐      ┌──────────┐      ┌───────┐
│  UI    │ ───▶ │ Parser │ ───▶ │  Game    │ ───▶ │ Board │
│ (App + │      │ (PGN)  │      │ (rules & │      │ (data │
│  View) │      └────────┘      │ state)   │      │ model)│
└────────┘                       ▲          └───────┘
     │                            │
     │                            ▼
     └──────────────────────▶ Moves
                              (precomputed move
                               tables + helpers)
```

- **UI (`src/ui/app.rs`, `src/ui/ui.rs`)** owns terminal state, renders the board, and forwards keystrokes to the parser/game.
- **Parser (`src/engine/piece_kind.rs`)** only understands PGN-style commands today.
- **Game (`src/engine/game.rs`)** coordinates move validation, legality (check/checkmate), turn tracking, and end-game detection.
- **Board (`src/engine/board.rs`)** holds bitboards for every piece type and exposes helpers for move generation.
- **Moves (`src/engine/moves.rs`)** builds pseudo-legal moves for the six FIDE piece types using precomputed rays and direction masks.

## FIDE-Specific Behaviors to Replace

| Area | File(s) | Why it must change |
| ---- | ------- | ------------------ |
| Two-color assumption | `src/engine/board.rs`, `src/engine/game.rs`, `src/ui/ui.rs` | Bitboards, turn logic, and render paths all hard-code `white`/`black`. Need four-army enums, occupancy, frozen status, and team concepts. |
| Castling & en passant bookkeeping | `src/engine/game.rs` | Enochian chess removes castling and en passant; these fields should be dropped or repurposed for throne/frozen state. |
| Checkmate-oriented legality filters | `src/engine/game.rs` | The current legality filter forbids leaving the king in check; Enochian rules allow it (king capture instead of mate). Need forced-king-move logic instead. |
| Sliding queen implementation | `src/engine/moves.rs` | Queens leap two squares Alibaba-style rather than sliding. The move tables must be rebuilt. |
| Bishop capture matrix | `src/engine/moves.rs` | Bishops currently capture any opposing piece. They must obey Aries/Cancer networks and restricted capture targets. |
| Pawn direction & double-step | `src/engine/moves.rs` | The code assumes white moves +8 and black moves −8 with optional double moves/en passant. All four armies require unique forward vectors and no double-step. |
| PGN parser | `src/engine/piece_kind.rs` | Input is PGN-only. The Enochian UI will accept commands like `blue: a3-a4` plus custom verbs (`/arrays`, `/exchange`). Parser and command routing must be rebuilt. |
| Sprite rendering | `src/ui/ui.rs` | The board only renders two colors of pieces and standard chess glyphs. We need four color palettes, throne indicators, frozen markers, and per-army legends. |
| Tests | `tests/` | Existing tests reference PGN helpers and fail to compile. The new rule set requires scenario-based fixtures that cover the Enochian mechanics. |

Documenting these hotspots clarifies where agents must focus when replacing the legacy FIDE logic with the new Enochian model described in `docs/enochian-rules.*`.
