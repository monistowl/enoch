+++
title = "Bishop"
description = "A diagonal slider bound to one of two mystical networks"
template = "page.html"
+++

<div style="text-align: center; font-size: 4rem; margin-bottom: 2rem;">♗</div>

The Bishop moves as in standard chess: **sliding any number of squares diagonally**, stopped only by pieces in its path.

<div class="board-container">
<svg viewBox="0 0 350 350" xmlns="http://www.w3.org/2000/svg">
  <!-- 7x7 board -->
  <defs>
    <pattern id="b-board" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>
  <rect width="350" height="350" fill="url(#b-board)"/>

  <!-- Diagonal rays from center -->
  <!-- NW diagonal -->
  <circle cx="125" cy="125" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="75" cy="75" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="25" cy="25" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <!-- NE diagonal -->
  <circle cx="225" cy="125" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="275" cy="75" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="325" cy="25" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <!-- SW diagonal -->
  <circle cx="125" cy="225" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="75" cy="275" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="25" cy="325" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <!-- SE diagonal -->
  <circle cx="225" cy="225" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="275" cy="275" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="325" cy="325" r="12" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>

  <!-- Direction lines -->
  <line x1="175" y1="175" x2="30" y2="30" stroke="#4a7cb8" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>
  <line x1="175" y1="175" x2="320" y2="30" stroke="#4a7cb8" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>
  <line x1="175" y1="175" x2="30" y2="320" stroke="#4a7cb8" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>
  <line x1="175" y1="175" x2="320" y2="320" stroke="#4a7cb8" stroke-width="1" stroke-dasharray="4,4" opacity="0.5"/>

  <!-- Bishop in center -->
  <text x="175" y="188" text-anchor="middle" fill="#6a9cd8" font-size="36">♝</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -0.5rem;">
  The Bishop slides along diagonals until blocked.
</p>

## Diagonal Networks

What makes the Enochian Bishop unique is its assignment to a **diagonal network**.

The board's diagonals are divided into two intertwined systems:
- **Aries network** (bitmask `0x55AA55AA55AA55AA`)
- **Cancer network** (bitmask `0xAA55AA55AA55AA55`)

Each bishop is permanently assigned to one network at the start of the game. This affects what it can capture.

## Capture Restrictions

| Can Capture | Cannot Capture |
|-------------|----------------|
| Kings | Bishops (any) |
| Rooks | Queens (different network) |
| Knights | |
| Pawns | |
| Queens (same network only) | |

### Bishops Never Capture Bishops

No bishop can ever capture another bishop—regardless of network. They pass through each other's influence like ghosts.

### Network-Limited Queen Captures

A bishop can only capture a queen if both share the **same diagonal network**. A Cancer bishop cannot touch an Aries queen, and vice versa.

## Strategic Implications

The network system creates interesting dynamics:
- Your bishop might completely ignore certain enemy pieces
- Board control is split across two invisible layers
- Knowing which network each piece belongs to is crucial

<div class="callout callout--tip">
    <div class="callout__title">Watch the Networks</div>
    <div class="callout__content">
        When evaluating a position, consider which network your pieces and your opponent's pieces belong to. Some threats simply don't exist across network boundaries.
    </div>
</div>

## Summary

| Property | Value |
|----------|-------|
| Movement | Slide diagonally, any distance |
| Capture | Same as movement, with restrictions |
| Cannot Capture | Bishops (never), Queens (different network) |
| Network | Assigned to Aries or Cancer at game start |
