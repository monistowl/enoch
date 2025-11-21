# Enoch Architecture

This document outlines the architecture of the `enoch` chess engine, focusing on the core data structures and modules.

## Overview

The `enoch` engine is a Rust application that implements the rules of Enochian chess and provides both a terminal-based user interface (TUI) for interactive play and a headless CLI mode for scripting and automation. The architecture is divided into two main components: the `engine` and the `ui`.

*   **Engine:** The `engine` module contains the core logic for the game, including the board representation, move generation, game state management, and AI strategies.
*   **UI:** The `ui` module is responsible for rendering the TUI and handling user input.

The two components are well-decoupled, with the `ui` module acting as a controller that interacts with the `engine` through a well-defined API. The headless CLI mode provides direct access to engine functionality for non-interactive use cases.

## Core Modules

### `src/engine`

*   **`board.rs`:** Defines the `Board` struct, which represents the state of the chess board using bitboards. It also includes logic for piece placement, movement, and capturing.
*   **`game.rs`:** Defines the `Game` struct, which encapsulates the entire game state and logic. It manages turns, checks for legal moves, enforces the rules of the game, and includes move caching for performance optimization.
*   **`moves.rs`:** Contains the logic for generating pseudo-legal moves for each piece kind.
*   **`types.rs`:** Defines the core data types used throughout the engine, such as `Army`, `Team`, `PieceKind`, and `Move`.
*   **`arrays.rs`:** Defines the starting positions (arrays) for the game.
*   **`ai.rs`:** Implements AI strategies including random move selection and capture-preferring move selection.

### `src/ui`

*   **`app.rs`:** Defines the `App` struct, which manages the state of the TUI application. It handles user input, parses commands, updates the UI accordingly, and manages AI opponents.
*   **`ui.rs`:** Contains the logic for rendering the TUI using the `ratatui` library, including move highlighting, captured pieces display, last move indicator, and colorblind mode.

### `src/main.rs`

*   Entry point for both TUI and headless CLI modes
*   Implements CLI argument parsing with `clap`
*   Provides headless commands: `--validate`, `--analyze`, `--query`, `--generate`, `--perft`, `--convert`
*   Manages game state persistence and AI automation

## Key Data Structures

*   **`Board`:** Represents the chess board using bitboards. It has a `by_army_kind` field, which is a `[[u64; 6]; 4]` array that stores the location of each piece.
*   **`Game`:** The main struct for the game engine. It contains the `Board`, `GameConfig`, and `GameState`.
*   **`GameConfig`:** Stores the configuration for a game, such as the turn order and controller map.
*   **`GameState`:** Stores the dynamic state of the game, such as the current turn, frozen armies, and king positions.
*   **`App`:** The main struct for the TUI application. It holds the `Game` instance and manages the UI state.

## Hard-coded FIDE Rules

The current codebase is designed for Enochian chess and does not seem to have any hard-coded FIDE-specific rules like castling or en passant. The move generation and game logic are all tailored to the rules of Enochian chess.

## TUI Rendering

The TUI is rendered using the `ratatui` library. The `ui.rs` file contains the logic for drawing the board, status panels, and input command line. Current features include:

*   **Checkerboard pattern** with wheat and brown colors for better visual clarity
*   **Color-coded armies** with adaptive piece colors for contrast
*   **Move highlighting** showing selected pieces (yellow) and legal moves (green)
*   **Captured pieces display** tracking all captures by army
*   **Last move indicator** showing opponent's previous move
*   **Colorblind mode** with army symbols (▲ Blue, ▼ Red, ◀ Black, ▶ Yellow)
*   **Built-in help system** accessible via `?` or `F1`

## Headless CLI Mode

The engine supports non-interactive operation through command-line flags:

*   **`--validate`:** Check move legality without applying changes
*   **`--analyze`:** Inspect squares and show legal moves
*   **`--query`:** Natural language rules lookup
*   **`--generate`:** Create custom positions from notation
*   **`--perft`:** Performance testing and move generation validation
*   **`--convert`:** Transform between JSON, ASCII, and compact formats

This enables scripting, automation, testing, and integration with external tools.

## Performance Optimizations

*   **Move caching:** Legal moves are cached per army and invalidated on state changes, providing ~4x performance improvement
*   **Bitboard representation:** Efficient board state using 64-bit integers
*   **Perft benchmarking:** ~483k nodes per second at depth 4 on default position