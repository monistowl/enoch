+++
title = "King"
description = "The most important piece—and unlike standard chess, it can be captured"
template = "page.html"
+++

<div style="text-align: center; font-size: 4rem; margin-bottom: 2rem;">♔</div>

The King moves exactly as in standard chess: **one square in any direction** (horizontally, vertically, or diagonally).

<div class="board-container">
<svg viewBox="0 0 150 150" xmlns="http://www.w3.org/2000/svg">
  <!-- 3x3 board centered on king -->
  <defs>
    <pattern id="k-board" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>
  <rect width="150" height="150" fill="url(#k-board)"/>

  <!-- Move destinations (all 8 adjacent squares) -->
  <circle cx="25" cy="25" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="75" cy="25" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="125" cy="25" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="25" cy="75" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="125" cy="75" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="25" cy="125" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="75" cy="125" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="125" cy="125" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>

  <!-- King in center -->
  <text x="75" y="88" text-anchor="middle" fill="#e4c26a" font-size="36">♚</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -0.5rem;">
  The King can move to any of the 8 adjacent squares.
</p>

## Key Differences from Standard Chess

<div class="callout callout--warning">
    <div class="callout__title">Kings Can Be Captured</div>
    <div class="callout__content">
        There is no checkmate in Enochian Chess. If your king is attacked and cannot escape, it is captured and removed from the board.
    </div>
</div>

### Forced King Moves

When your king is in check and has at least one legal king move:
- You **must** move your king
- You cannot block with another piece
- You cannot capture the attacker with a non-king piece

Only when the king has no legal moves can other pieces act.

### Frozen Armies

When a king is captured:
1. The entire army **freezes** in place
2. Frozen pieces cannot move, capture, or give check
3. Frozen pieces block squares like statues
4. The army remains frozen until rescued by an ally

### Throne Mechanics

Each army has two throne squares. When a king stands on its own throne:
- It can **share the square** with one allied piece
- If captured while sharing, both pieces are removed

When you move your king to an **ally's throne**:
- You seize control of their army
- If they were frozen, they thaw
- Control persists even after leaving the throne

## No Castling

There is no castling in Enochian Chess. The king simply moves one square at a time throughout the game.

## Summary

| Property | Value |
|----------|-------|
| Movement | One square, any direction |
| Capture | Same as movement |
| Special | Can be captured; freezes army when lost |
| Throne | Can share with ally; seizes allied armies |
