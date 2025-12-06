+++
title = "Starting Arrays"
description = "The eight elemental tablets that define initial piece placement"
template = "section.html"
+++

In Enochian Chess, the initial arrangement of pieces is determined by **Starting Arrays** — configurations based on the four elemental tablets described by Zalewski.

Each array defines:
- Which army occupies each edge of the board
- The turn order (clockwise or counter-clockwise)
- Throne positions
- Piece placements, including diagonal network assignments

## The Eight Tablets

Historical sources document eight arrays, each associated with an elemental combination:

| Tablet | Element | Turn Order | Status |
|--------|---------|------------|--------|
| [**Fire**](tablet-of-fire/) | 🔥 Fire of Fire | Blue → Red → Black → Yellow | ✅ Implemented |
| [**Water**](tablet-of-water/) | 💧 Water of Water | Blue → Black → Yellow → Red | 📝 Documented |
| [**Air**](tablet-of-air/) | 💨 Air of Air | Red → Yellow → Black → Blue | 📝 Documented |
| [**Earth**](tablet-of-earth/) | 🌍 Earth of Earth | Yellow → Blue → Red → Black | 📝 Documented |

<div class="callout callout--note">
    <div class="callout__title">Work in Progress</div>
    <div class="callout__content">
        The engine currently implements the <strong>Tablet of Fire</strong> in full. The remaining seven tablets are defined as placeholders awaiting transcription from the original Zalewski diagrams.
    </div>
</div>

## Understanding the Layout

Unlike standard chess where two armies face each other across the board, Enochian Chess places **four armies on all four edges**:

- **Blue** (South) — Ranks 1-2
- **Red** (East) — Files g-h (rotated 90°)
- **Black** (North) — Ranks 7-8
- **Yellow** (West) — Files a-b (rotated 90°)

This means Red and Yellow pawns move **sideways** (west and east respectively), not up the board!
