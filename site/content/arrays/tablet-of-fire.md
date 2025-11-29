+++
title = "Tablet of Fire"
description = "The primary starting array — Fire of Fire"
template = "page.html"
+++

The **Tablet of Fire** is the default starting array in the Enochian Chess engine. It represents the "Fire of Fire" configuration from Zalewski's documentation.

## Configuration

| Property | Value |
|----------|-------|
| **Element** | Fire of Fire |
| **Turn Order** | Blue → Red → Black → Yellow (clockwise) |
| **Team Air** | Blue (South) + Black (North) |
| **Team Earth** | Red (East) + Yellow (West) |

## Throne Squares

| Army | Thrones |
|------|---------|
| <span class="army-badge army-badge--blue">Blue</span> | d1, e1 |
| <span class="army-badge army-badge--red">Red</span> | h4, h5 |
| <span class="army-badge army-badge--black">Black</span> | d8, e8 |
| <span class="army-badge army-badge--yellow">Yellow</span> | a4, a5 |

## Board Diagram

<div class="board-container">
<svg viewBox="0 0 400 400" xmlns="http://www.w3.org/2000/svg">
  <!-- Board squares -->
  <defs>
    <pattern id="board-pat" width="100" height="100" patternUnits="userSpaceOnUse">
      <rect width="50" height="50" fill="#2a2a35"/>
      <rect x="50" width="50" height="50" fill="#1a1a22"/>
      <rect y="50" width="50" height="50" fill="#1a1a22"/>
      <rect x="50" y="50" width="50" height="50" fill="#2a2a35"/>
    </pattern>
  </defs>

  <rect width="400" height="400" fill="url(#board-pat)"/>

  <!-- Throne highlights -->
  <rect x="150" y="350" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="200" y="350" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="0" y="150" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="0" y="200" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="150" y="0" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="200" y="0" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="350" y="150" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>
  <rect x="350" y="200" width="50" height="50" fill="rgba(196, 162, 74, 0.2)" stroke="#c4a24a" stroke-width="1"/>

  <!-- Blue army (rank 1-2, South) -->
  <text x="25" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♜</text>
  <text x="75" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♞</text>
  <text x="125" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♝</text>
  <text x="175" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♛</text>
  <text x="225" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♚</text>
  <text x="275" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♝</text>
  <text x="325" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♞</text>
  <text x="375" y="378" text-anchor="middle" fill="#6a9cd8" font-size="28">♜</text>
  <!-- Blue pawns -->
  <text x="25" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="75" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="125" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="175" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="225" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="275" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="325" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>
  <text x="375" y="328" text-anchor="middle" fill="#6a9cd8" font-size="24">♟</text>

  <!-- Red army (rank 8-7, North/top) -->
  <text x="25" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♜</text>
  <text x="75" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♞</text>
  <text x="125" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♝</text>
  <text x="175" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♛</text>
  <text x="225" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♚</text>
  <text x="275" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♝</text>
  <text x="325" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♞</text>
  <text x="375" y="28" text-anchor="middle" fill="#e47a6a" font-size="28">♜</text>
  <!-- Red pawns -->
  <text x="25" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="75" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="125" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="175" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="225" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="275" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="325" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>
  <text x="375" y="78" text-anchor="middle" fill="#e47a6a" font-size="24">♟</text>

  <!-- Black army (file a, West/left side - vertical) -->
  <text x="25" y="128" text-anchor="middle" fill="#8a8aaa" font-size="28">♜</text>
  <text x="25" y="178" text-anchor="middle" fill="#8a8aaa" font-size="28">♞</text>
  <text x="25" y="228" text-anchor="middle" fill="#8a8aaa" font-size="28">♝</text>
  <text x="25" y="278" text-anchor="middle" fill="#8a8aaa" font-size="28">♛</text>
  <!-- Black pawns (file b) -->
  <text x="75" y="128" text-anchor="middle" fill="#8a8aaa" font-size="24">♟</text>
  <text x="75" y="178" text-anchor="middle" fill="#8a8aaa" font-size="24">♟</text>
  <text x="75" y="228" text-anchor="middle" fill="#8a8aaa" font-size="24">♟</text>
  <text x="75" y="278" text-anchor="middle" fill="#8a8aaa" font-size="24">♟</text>

  <!-- Yellow army (file h, East/right side - vertical) -->
  <text x="375" y="128" text-anchor="middle" fill="#e4c26a" font-size="28">♜</text>
  <text x="375" y="178" text-anchor="middle" fill="#e4c26a" font-size="28">♞</text>
  <text x="375" y="228" text-anchor="middle" fill="#e4c26a" font-size="28">♝</text>
  <text x="375" y="278" text-anchor="middle" fill="#e4c26a" font-size="28">♛</text>
  <!-- Yellow pawns (file g) -->
  <text x="325" y="128" text-anchor="middle" fill="#e4c26a" font-size="24">♟</text>
  <text x="325" y="178" text-anchor="middle" fill="#e4c26a" font-size="24">♟</text>
  <text x="325" y="228" text-anchor="middle" fill="#e4c26a" font-size="24">♟</text>
  <text x="325" y="278" text-anchor="middle" fill="#e4c26a" font-size="24">♟</text>

  <!-- Board labels -->
  <text x="25" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">a</text>
  <text x="75" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">b</text>
  <text x="125" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">c</text>
  <text x="175" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">d</text>
  <text x="225" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">e</text>
  <text x="275" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">f</text>
  <text x="325" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">g</text>
  <text x="375" y="395" text-anchor="middle" fill="#5a5854" font-size="10" font-family="monospace">h</text>
</svg>
</div>

<p style="text-align: center; color: #8a8680; font-size: 0.9rem; margin-top: -1rem;">
  Throne squares highlighted in gold. Blue (south), Red (north), Black (west), Yellow (east).
</p>

<div class="callout callout--note">
    <div class="callout__title">Prototype Layout</div>
    <div class="callout__content">
        This diagram shows the current <strong>prototype transcription</strong> used by the engine. The exact historical Zalewski diagram may differ slightly in piece arrangement. Black and Yellow armies are shown with reduced pieces for the prototype; the final version will have complete armies on the side edges.
    </div>
</div>

## Piece Placement Details

### Blue Army (South)
Standard chess arrangement on ranks 1-2:
- Rank 1: R-N-B-Q-K-B-N-R
- Rank 2: 8 pawns

### Red Army (North)
Mirror of Blue on ranks 7-8:
- Rank 8: R-N-B-Q-K-B-N-R
- Rank 7: 8 pawns

### Black Army (West)
Arranged vertically on files a-b:
- File a: Rook, Knight, Bishop, Queen (ranks 6-3)
- File b: 4 pawns (moving east toward file h)

### Yellow Army (East)
Arranged vertically on files g-h:
- File h: Rook, Knight, Bishop, Queen (ranks 6-3)
- File g: 4 pawns (moving west toward file a)

## Pawn Movement Directions

| Army | Pawn Direction | Promotion Zone |
|------|----------------|----------------|
| Blue | ↑ North | Rank 8 |
| Red | ↓ South | Rank 1 |
| Black | → East | File h |
| Yellow | ← West | File a |

This creates fascinating dynamics where pawns from different armies advance in perpendicular directions!
