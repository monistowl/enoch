+++
title = "Knight"
description = "The leaping piece—unchanged from standard chess"
template = "page.html"
+++

<div style="text-align: center; font-size: 4rem; margin-bottom: 2rem;">♘</div>

The Knight moves exactly as in standard chess: an **L-shaped leap** of two squares in one direction and one square perpendicular (or vice versa).

<div class="board-container">
<svg viewBox="0 0 250 250" xmlns="http://www.w3.org/2000/svg">
  <!-- 5x5 board -->
  <defs>
    <pattern id="n-board" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>
  <rect width="250" height="250" fill="url(#n-board)"/>

  <!-- Knight moves (8 L-shaped destinations) -->
  <!-- 2 up, 1 left/right -->
  <circle cx="75" cy="25" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <circle cx="175" cy="25" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <!-- 2 down, 1 left/right -->
  <circle cx="75" cy="225" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <circle cx="175" cy="225" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <!-- 2 left, 1 up/down -->
  <circle cx="25" cy="75" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <circle cx="25" cy="175" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <!-- 2 right, 1 up/down -->
  <circle cx="225" cy="75" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>
  <circle cx="225" cy="175" r="15" fill="rgba(196, 90, 74, 0.4)" stroke="#c45a4a" stroke-width="2"/>

  <!-- Knight in center -->
  <text x="125" y="138" text-anchor="middle" fill="#c45a4a" font-size="36">♞</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -0.5rem;">
  The Knight leaps in an L-shape: 2+1 squares (8 possible destinations).
</p>

## Movement

The Knight:
- Moves in an "L" pattern: 2+1 squares
- **Leaps** directly to its destination
- Ignores all pieces between start and end squares
- Is the only piece (besides the Queen) that can jump over blockers

## Captures

The Knight has **no capture restrictions**. It can capture any enemy piece on its destination square:
- Kings
- Queens
- Bishops
- Rooks
- Pawns
- Other Knights

## Strategic Value

In the crowded four-army battlefield of Enochian Chess, the Knight's leaping ability is especially valuable:
- **Forking potential** increases with more pieces on the board
- **Jumping over blockades** lets it reach positions others can't
- **Unpredictable threats** from its unique movement pattern

<div class="callout callout--tip">
    <div class="callout__title">Four-Army Forks</div>
    <div class="callout__content">
        With four armies on the board, fork opportunities multiply. A well-placed knight might threaten pieces from three different armies simultaneously.
    </div>
</div>

## Summary

| Property | Value |
|----------|-------|
| Movement | L-shaped leap (2+1 squares) |
| Capture | Same as movement |
| Restrictions | None |
| Special | Jumps over all pieces |
