+++
title = "Pawn"
description = "The foot soldier—simpler movement, unique promotion rules"
template = "page.html"
+++

<div style="text-align: center; font-size: 4rem; margin-bottom: 2rem;">♙</div>

The Pawn in Enochian Chess is simpler than its standard chess counterpart, but has unique promotion mechanics.

## Movement

Each pawn moves **one square forward** in its army's direction:

<div class="board-container">
<svg viewBox="0 0 150 150" xmlns="http://www.w3.org/2000/svg">
  <!-- 3x3 board showing pawn movement (Blue army - moves north) -->
  <defs>
    <pattern id="p-board" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>
  <rect width="150" height="150" fill="url(#p-board)"/>

  <!-- Move destination (straight ahead - blue) -->
  <circle cx="75" cy="25" r="15" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>

  <!-- Capture destinations (diagonal - red) -->
  <circle cx="25" cy="25" r="12" fill="rgba(196, 90, 74, 0.3)" stroke="#c45a4a" stroke-width="2"/>
  <circle cx="125" cy="25" r="12" fill="rgba(196, 90, 74, 0.3)" stroke="#c45a4a" stroke-width="2"/>

  <!-- Pawn in center -->
  <text x="75" y="88" text-anchor="middle" fill="#6a9cd8" font-size="36">♟</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -0.5rem;">
  Pawns move forward (blue circles) and capture diagonally (red circles). Direction depends on army.
</p>

| Army | Direction | Promotion Zone |
|------|-----------|----------------|
| <span class="army-badge army-badge--blue">Blue</span> | North (toward rank 8) | Rank 8 |
| <span class="army-badge army-badge--red">Red</span> | West (toward file a) | File a |
| <span class="army-badge army-badge--black">Black</span> | South (toward rank 1) | Rank 1 |
| <span class="army-badge army-badge--yellow">Yellow</span> | East (toward file h) | File h |

<div class="callout callout--note">
    <div class="callout__title">No Double-Step or En Passant</div>
    <div class="callout__content">
        Pawns move one square only. There is no initial two-square advance and no en passant capture.
    </div>
</div>

## Captures

Pawns capture **one square diagonally forward** (in the direction they move), just like standard chess.

## Promotion

When a pawn reaches its promotion zone (see table above), it promotes. But promotion works differently:

### Patron Piece

Each pawn is assigned a **patron piece type** at the start of the game (e.g., "pawn of the queen" or "pawn of the bishop"). Upon promotion, it becomes that piece type.

### Privileged Pawn {#privileged}

If an army is reduced to a minimal force:
- King + Queen + Pawn
- King + Bishop + Pawn
- King + Pawn only

...the pawn becomes **privileged**. A privileged pawn may promote to **any** major piece type.

<div class="callout callout--warning">
    <div class="callout__title">The Demotion Twist</div>
    <div class="callout__content">
        If you use a privileged pawn to promote to a piece type already on the board, the existing piece is <strong>demoted</strong> back into a pawn of that type.
    </div>
</div>

This creates fascinating endgame decisions: do you create a second queen at the cost of demoting your current one?

## Summary

| Property | Value |
|----------|-------|
| Movement | One square forward (army direction) |
| Capture | One square diagonally forward |
| Double-Step | No |
| En Passant | No |
| Promotion | To patron piece (or any, if privileged) |
