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

## Implemented Enochian Features

The following changes have been completed to transform the engine:

### 1. Core Data Model (`src/engine/board.rs`, `src/engine/game.rs`)

*   **Armies and Teams:** Replaced `white`/`black` with `Army` (Blue, Black, Red, Yellow) and `Team` (Air, Earth).
*   **Board Representation:** `Board` uses `by_army_kind: [[u64; 6]; 4]` for four-army bitboards.
*   **Game State:** `Game` handles four-player turn order, frozen armies, and throne control. FIDE state (castling/en passant) was removed.

### 2. Move Generation (`src/engine/moves.rs`)

*   **Queen:** Implemented the two-square Alibaba leap.
*   **Bishop/Queen Interaction:** Implemented "Concourse of Bishoping" (Aries/Cancer diagonal systems).
*   **Pawn:** Removed double-step/en passant. Implemented single-step and diagonal capture per army direction.

### 3. Game Rules (`src/engine/game.rs`)

*   **King Capture:** Kings are captured, freezing the army.
*   **Seizing the Throne:** Kings can unfreeze allies by occupying their throne.
*   **Exchange of Prisoners:** Implemented via `/exchange` command.
*   **Privileged Pawn:** Detection and multi-choice promotion implemented.
*   **Stalemate:** Stalemated armies skip turns.
*   **Divination Mode:** Implemented dice-based move constraints (`Mode::Divination`).

### 4. User Interface (`src/ui/`)

*   **TUI Overhaul:** Redesigned to match web layout (Captures | Board | Captures).
*   **Status Panel:** Added turn indicator, frozen status, check alerts, and die roll display.
*   **Commands:** Implemented `/new`, `/save`, `/load`, `/ai`, `/mode`.

### 5. AI and Persistence (`src/engine/ai.rs`, `src/engine/fen.rs`)

*   **`src/engine/ai.rs`**: Implements a basic AI using material evaluation. In Divination mode, it selects random legal moves respecting the die. In Normal mode, it uses a single-ply lookahead.
*   **`src/engine/fen.rs`**: Implements `EnochFen` for JSON-based serialization/deserialization of the full game state (arrays, turn order, frozen status). Used for save/load functionality.

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
