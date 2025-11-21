# Enoch Architecture

This document outlines the architecture of the `enoch` chess engine, focusing on the core data structures and modules.

## Overview

The `enoch` engine is a Rust application that implements the rules of Enochian chess and provides a terminal-based user interface (TUI) for playing the game. The architecture is divided into two main components: the `engine` and the `ui`.

*   **Engine:** The `engine` module contains the core logic for the game, including the board representation, move generation, and game state management.
*   **UI:** The `ui` module is responsible for rendering the TUI and handling user input.

The two components are well-decoupled, with the `ui` module acting as a controller that interacts with the `engine` through a well-defined API.

## Core Modules

### `src/engine`

*   **`board.rs`:** Defines the `Board` struct, which represents the state of the chess board using bitboards. It also includes logic for piece placement, movement, and capturing.
*   **`game.rs`:** Defines the `Game` struct, which encapsulates the entire game state and logic. It manages turns, checks for legal moves, and enforces the rules of the game.
*   **`moves.rs`:** Contains the logic for generating pseudo-legal moves for each piece kind.
*   **`types.rs`:** Defines the core data types used throughout the engine, such as `Army`, `Team`, `PieceKind`, and `Move`.
*   **`arrays.rs`:** Defines the starting positions (arrays) for the game.

### `src/ui`

*   **`app.rs`:** Defines the `App` struct, which manages the state of the TUI application. It handles user input, parses commands, and updates the UI accordingly.
*   **`ui.rs`:** Contains the logic for rendering the TUI using the `ratatui` library.

## Key Data Structures

*   **`Board`:** Represents the chess board using bitboards. It has a `by_army_kind` field, which is a `[[u64; 6]; 4]` array that stores the location of each piece.
*   **`Game`:** The main struct for the game engine. It contains the `Board`, `GameConfig`, and `GameState`.
*   **`GameConfig`:** Stores the configuration for a game, such as the turn order and controller map.
*   **`GameState`:** Stores the dynamic state of the game, such as the current turn, frozen armies, and king positions.
*   **`App`:** The main struct for the TUI application. It holds the `Game` instance and manages the UI state.

## Hard-coded FIDE Rules

The current codebase is designed for Enochian chess and does not seem to have any hard-coded FIDE-specific rules like castling or en passant. The move generation and game logic are all tailored to the rules of Enochian chess.

## TUI Rendering

The TUI is rendered using the `ratatui` library. The `ui.rs` file contains the logic for drawing the board, status panels, and input command line. The rendering is functional, but it could be improved by:

*   Using more graphical characters for the pieces instead of just letters.
*   Adding more color to the UI to distinguish between the different armies and teams.
*   Improving the layout and organization of the different UI components.

The roadmap mentions using Kitty's graphics protocol for a more advanced UI. This would be a good next step for improving the TUI.