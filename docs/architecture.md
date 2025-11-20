# Enochian Chess Engine Architecture

This document outlines the high-level architecture of the `enoch` project, focusing on the core modules, data structures, and their interactions.

## 1. Overview

The `enoch` project is structured into two main top-level modules: `engine` and `ui`. The `main.rs` file orchestrates the interaction between these two modules, setting up the terminal user interface (TUI) and managing the main application loop and event handling.

```
.
├── src/
│   ├── engine/                 # Core game logic and rules
│   │   ├── arrays.rs
│   │   ├── board.rs
│   │   ├── game.rs
│   │   ├── macros.rs
│   │   ├── moves.rs
│   │   ├── piece_kind.rs
│   │   └── types.rs
│   ├── ui/                     # Terminal User Interface (TUI)
│   │   ├── app.rs
│   │   └── ui.rs
│   ├── lib.rs                  # Module re-exports
│   └── main.rs                 # Main application entry point
└── docs/
    ├── architecture.md         # This document
    ├── enochian-rules.md       # Detailed Enochian rules in markdown
    └── enochian-rules.yaml     # Machine-readable Enochian rules
```

## 2. Core Modules

### 2.1. `engine` Module

The `engine` module encapsulates all the core game logic, rules, and data structures related to Enochian Chess.

*   **`types.rs`**: Defines fundamental enumerations and structures that are used throughout the engine:
    *   `Army`: Represents the four armies (Blue, Black, Red, Yellow).
    *   `Team`: Represents the two teams (Air: Blue+Black, Earth: Red+Yellow).
    *   `PieceKind`: Defines the types of pieces (King, Queen, Rook, Bishop, Knight, Pawn).
    *   `Square`: Represents a square on the 8x8 board (0-63).
    *   `Move`: Represents a chess move, including `from`, `to`, `kind`, and optional `promotion`.
    *   `PlayerId`: Identifies the players controlling armies.

*   **`board.rs`**: Manages the state of the chessboard and the pieces on it.
    *   `Board` struct: Uses bitboards (`u64`) for efficient representation of piece positions.
        *   `by_army_kind`: `[[u64; PIECE_KIND_COUNT]; ARMY_COUNT]` - Stores bitboards for each piece kind for each army.
        *   `occupancy_by_army`: `[u64; ARMY_COUNT]` - Bitboard of all pieces for each army.
        *   `occupancy_by_team`: `[u64; TEAM_COUNT]` - Bitboard of all pieces for each team.
        *   `all_occupancy`: `u64` - Bitboard of all occupied squares.
        *   `free`: `u64` - Bitboard of all empty squares.
    *   Provides methods for placing, moving, and removing pieces, and querying the board state (e.g., `piece_at`, `king_square`).
    *   Includes `refresh_occupancy()` to keep bitboards consistent after modifications.

*   **`piece_kind.rs`**: Primarily defines the `PieceKind` enum and associated helper methods, possibly for parsing and formatting piece information.

*   **`moves.rs`**: Responsible for generating pseudo-legal moves for each piece type.
    *   Contains functions like `compute_king_moves`, `compute_knights_moves`, `compute_pawns_moves`, etc.
    *   Utilizes precomputed lookup tables (e.g., `KING_MOVES`, `KNIGHT_MOVES`) for efficiency.
    *   Handles sliding piece logic (Rook, Bishop, Queen) with ray-based attacks and blocker detection.
    *   Integrates Enochian specific movement rules (e.g., Queen leaps).

*   **`game.rs`**: Implements the main game logic and rules enforcement. This is the central orchestrator of the game state and rules.
    *   `Game` struct: Holds the current `Board`, `GameConfig`, `GameState`, and `Status`.
    *   `GameConfig`: Stores game configuration like `armies`, `turn_order`, and `controller_map`.
    *   `GameState`: Manages dynamic game state elements such as `current_turn_index`, `army_frozen` status, `king_positions`, and `stalemated_armies`.
    *   Key functions include:
        *   `apply_move`: Attempts to apply a move, enforcing rules like turn order, piece ownership, and check constraints.
        *   `generate_legal_moves`: Generates all legal moves for a given army, filtering pseudo-legal moves based on check safety.
        *   `generate_legal_king_moves`: Generates legal moves specifically for the king.
        *   `generate_legal_non_king_moves`: Generates legal moves for non-king pieces.
        *   `king_in_check`: Determines if an army's king is currently under attack.
        *   `must_move_king`: Determines if the king is in check and no other piece can legally resolve the check.
        *   `update_stalemate_status`: Updates the stalemate status of an army.
        *   `capture_king`, `freeze_army`, `seize_throne_at`, `exchange_prisoners`: Implement Enochian-specific game mechanics.
        *   `is_privileged_pawn`, `promotion_targets`, `promote_pawn`: Handle pawn promotion logic.

*   **`arrays.rs`**: Expected to contain definitions and logic for different starting board arrays (e.g., Zalewski's eight arrays).

*   **`macros.rs`**: Contains Rust macros to simplify code generation, possibly for precomputed move tables or repetitive bitboard operations.

### 2.2. `ui` Module

The `ui` module is responsible for the Terminal User Interface (TUI) components using the `ratatui` and `crossterm` libraries.

*   **`app.rs`**: Defines the main application state for the TUI (`App` struct). It manages:
    *   The current screen/mode of the application (e.g., Main, Exiting).
    *   User input processing (e.g., adding characters, deleting, submitting commands).
    *   Interaction with the game engine based on user commands.

*   **`ui.rs`**: Contains the rendering logic.
    *   `render` function: Takes the current `App` state and draws the various UI elements to the terminal frame (e.g., game board, status messages, input field).
    *   `render_size_error`: Handles drawing a message if the terminal window is too small.

## 3. Key Concepts & Implementations

### Bitboards

The engine heavily relies on bitboards (`u64`) for efficient representation and manipulation of the board state. This allows for fast calculation of piece occupancies and move generation.

### Multi-Army and Teams

Enochian Chess introduces four armies (Blue, Black, Red, Yellow) grouped into two teams (Air and Earth). The `engine/types.rs` and `engine/board.rs` are designed to manage this multi-army structure, replacing traditional two-color chess logic.

### Enochian-Specific Rules

The `engine/game.rs` module is central to enforcing the unique rules of Enochian Chess, including:
*   **King Capture**: Kings are captured, not checkmated, leading to armies becoming "frozen".
*   **Frozen Armies**: Captured kings lead to their armies being unable to move, attack, or be captured, acting as blocking terrain.
*   **Throne Seizure**: Kings moving onto ally throne squares can transfer control.
*   **Privileged Pawns**: Special promotion rules based on remaining pieces.
*   **Queen Leaps**: Queens move by leaping exactly two squares.
*   **Bishop/Queen Diagonal Interaction**: Specific capture rules based on diagonal systems (Aries/Cancer).

## 4. TUI Interaction

The `main.rs` and `ui` module set up a `ratatui`-based TUI. The `App` struct in `ui/app.rs` acts as an intermediary, processing user commands (e.g., moves, system commands) and invoking the appropriate functions within the `engine` module to update the game state. The `ui/ui.rs` then renders the current game state, including the board, piece positions, and game messages, to the terminal.

## 5. FIDE vs. Enochian Adaptations

The codebase has been adapted from a more traditional chess engine to accommodate Enochian rules. While the underlying bitboard mechanics might share similarities with FIDE engines, explicit FIDE-only fields (like castling rights, en passant squares, 50-move rule counters) are either removed or unused, reflecting the Enochian rule set. The core logic now revolves around `Army` and `Team` instead of `Color` or `Side`.