+++
title = "Play"
description = "Play a game of Enochian Chess with the interactive WASM-powered game board."
template = "page.html"
+++

# Play Enochian Chess

This interactive game uses the Rust game engine compiled to WebAssembly. All move validation and game logic runs directly in your browser.

**How to play:**
1. Click a piece to select it (must be your turn and not frozen)
2. Legal moves appear as blue dots, captures as red circles
3. Click a legal square to move
4. Turn order: Blue, Red, Black, Yellow (Tablet of Fire)

{{ live_game() }}

## Rules Reminder

- **Teams**: Air (Blue + Black) vs Earth (Red + Yellow)
- **Victory**: Capture the opposing team's kings
- **Frozen Army**: When a king is captured, that army freezes in place
- **Turn Order**: Blue, Red, Black, Yellow (varies by starting array)
