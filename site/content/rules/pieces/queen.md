+++
title = "Queen"
description = "The most unusual piece—a leaper, not a slider"
template = "page.html"
+++

<div style="text-align: center; font-size: 4rem; margin-bottom: 2rem;">♕</div>

The Queen in Enochian Chess is radically different from standard chess. She is a **leaper**, not a slider.

## Movement

The Queen leaps exactly **two squares** in any direction:
- Orthogonally (horizontal or vertical)
- Diagonally

She **ignores blockers**—pieces between her starting square and destination don't matter. She simply jumps over them.

<div class="board-container">
<svg viewBox="0 0 250 250" xmlns="http://www.w3.org/2000/svg">
  <!-- 5x5 board centered on queen -->
  <defs>
    <pattern id="q-board" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>
  <rect width="250" height="250" fill="url(#q-board)"/>

  <!-- Move destinations (2 squares away) -->
  <!-- Orthogonal -->
  <circle cx="125" cy="25" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="125" cy="225" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="25" cy="125" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="225" cy="125" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <!-- Diagonal -->
  <circle cx="25" cy="25" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="225" cy="25" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="25" cy="225" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>
  <circle cx="225" cy="225" r="18" fill="rgba(74, 124, 184, 0.4)" stroke="#4a7cb8" stroke-width="2"/>

  <!-- Blocked squares (1 square away - can't stop here) -->
  <text x="125" y="82" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="125" y="182" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="75" y="132" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="175" y="132" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="75" y="82" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="175" y="82" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="75" y="182" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>
  <text x="175" y="182" text-anchor="middle" fill="#5a5854" font-size="14">✗</text>

  <!-- Queen in center -->
  <text x="125" y="138" text-anchor="middle" fill="#e47a6a" font-size="36">♛</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -0.5rem;">
  Blue circles: valid destinations (exactly 2 squares). ✗: cannot stop here.
</p>

<div class="callout callout--important">
    <div class="callout__title">Not a Slider!</div>
    <div class="callout__content">
        This is the biggest adjustment for chess players. The Queen cannot sweep across the board. She moves in precise two-square hops, making her both more limited in range and surprisingly tricky to use.
    </div>
</div>

## Capture Restrictions

The Queen has significant capture restrictions:

| Can Capture | Cannot Capture |
|-------------|----------------|
| Kings | Queens (any) |
| Rooks | Bishops (different network) |
| Knights | |
| Pawns | |
| Bishops (same network only) | |

### Queens Never Capture Queens

No queen can ever capture another queen. They are effectively invisible to each other, creating unusual board dynamics.

### Diagonal Networks

Each queen is assigned to either the **Aries** or **Cancer** diagonal network at the start of the game. A queen can only capture bishops that share her network.

This creates asymmetric threats—a queen might threaten some bishops but be completely harmless to others.

## Tactical Implications

The Queen's leap makes her excellent for:
- **Forking** pieces two squares apart
- **Jumping over** pawn chains and blockades
- **Surprise attacks** since she ignores intervening pieces

However, she struggles with:
- **Long-range control** (she can't sweep files or diagonals)
- **Continuous pressure** (must hop, can't slide into position)
- **Capturing queens** (simply impossible)

## Summary

| Property | Value |
|----------|-------|
| Movement | Leap exactly 2 squares, any direction |
| Capture | Same as movement, with restrictions |
| Cannot Capture | Queens (never), Bishops (different network) |
| Network | Assigned to Aries or Cancer at game start |
