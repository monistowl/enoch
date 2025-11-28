+++
title = "Rook"
description = "The straightforward piece—pure orthogonal power"
template = "page.html"
+++

<div style="text-align: center; font-size: 4rem; margin-bottom: 2rem;">♖</div>

The Rook is the most familiar piece in Enochian Chess. It moves exactly as in standard chess: **sliding any number of squares horizontally or vertically**, stopped by pieces in its path.

<div class="board-container">
<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg">
  <!-- 7x7 board -->
  <defs>
    <pattern id="r-board" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>
  <rect width="350" height="350" fill="url(#r-board)"/>

  <!-- Orthogonal rays from center -->
  <!-- Up -->
  <circle cx="175" cy="125" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="175" cy="75" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="175" cy="25" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <!-- Down -->
  <circle cx="175" cy="225" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="175" cy="275" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="175" cy="325" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <!-- Left -->
  <circle cx="125" cy="175" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="75" cy="175" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="25" cy="175" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <!-- Right -->
  <circle cx="225" cy="175" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="275" cy="175" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>
  <circle cx="325" cy="175" r="12" fill="rgba(196, 162, 74, 0.4)" stroke="#c4a24a" stroke-width="2"/>

  <!-- Direction lines -->
  <line x1="175" y1="175" x2="175" y2="25" stroke="#c4a24a" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>
  <line x1="175" y1="175" x2="175" y2="325" stroke="#c4a24a" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>
  <line x1="175" y1="175" x2="25" y2="175" stroke="#c4a24a" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>
  <line x1="175" y1="175" x2="325" y2="175" stroke="#c4a24a" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>

  <!-- Rook in center -->
  <text x="175" y="188" text-anchor="middle" fill="#c4a24a" font-size="36">♜</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -0.5rem;">
  The Rook slides along ranks and files until blocked.
</p>

## Movement

The Rook can move:
- Any number of squares along a **rank** (horizontal)
- Any number of squares along a **file** (vertical)

Pieces block its path—it cannot jump over anything.

## Captures

The Rook has **no capture restrictions**. It can capture any enemy piece it can reach:
- Kings
- Queens
- Bishops
- Knights
- Pawns
- Other Rooks

<div class="callout callout--note">
    <div class="callout__title">No Special Rules</div>
    <div class="callout__content">
        Unlike the Queen and Bishop, the Rook has no diagonal network restrictions. It's pure, straightforward orthogonal power.
    </div>
</div>

## No Castling

In standard chess, the Rook participates in castling with the King. **There is no castling in Enochian Chess**, so the Rook has no special moves.

## Strategic Value

With four armies on the board, Rooks gain additional importance:
- **Open files** become contested by multiple armies
- **Cross-board threats** can target pieces from any direction
- **Coordination** with your ally's rooks creates powerful batteries

The Rook's ability to capture anything makes it a reliable piece in the chaotic four-way battles of Enochian Chess.

## Summary

| Property | Value |
|----------|-------|
| Movement | Slide orthogonally, any distance |
| Capture | Same as movement |
| Restrictions | None |
| Special Moves | None (no castling) |
