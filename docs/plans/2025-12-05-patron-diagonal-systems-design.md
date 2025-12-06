# Patron Piece System & Diagonal Networks Design

**Date:** 2025-12-05
**Status:** Approved

## Overview

Implement two related features for Enochian chess rule fidelity:

1. **Patron Piece System** - Each pawn has a patron piece type; on promotion, the pawn becomes that patron (unless privileged)
2. **Diagonal Networks (Aries/Cancer)** - Queens and bishops are assigned to diagonal systems, restricting which pieces they can capture

## Data Model Changes

### Piece Struct (`types.rs`)

```rust
pub struct Piece {
    pub army: Army,
    pub kind: PieceKind,
    pub pawn_type: Option<PieceKind>,           // patron for pawns
    pub diagonal_system: Option<DiagonalSystem>, // for queens/bishops
}
```

### Board Struct (`board.rs`)

Add piece metadata storage alongside bitboards:

```rust
pub struct Board {
    // ... existing bitboard fields ...

    /// Per-square piece metadata (patron, diagonal system)
    pub piece_map: HashMap<Square, Piece>,
}
```

### ArraySpec (`arrays.rs`)

Placements already use `Piece` struct. Ensure arrays specify:
- `pawn_type: Some(PieceKind::X)` for each pawn
- `diagonal_system: Some(DiagonalSystem::Aries/Cancer)` for queens/bishops

## Logic Changes

### Promotion (`game.rs`)

```
promote_pawn(army, square, target):
  piece = piece_map.get(square)
  patron = piece.pawn_type.unwrap_or(Queen)

  if is_privileged_pawn(army):
    promote to `target` (player's choice)
  else:
    promote to `patron`

  // existing demotion logic unchanged
```

### Capture Restrictions (`moves.rs`)

```
can_capture_piece(attacker, target) -> bool:
  // Queens can't capture queens
  if attacker.kind == Queen && target.kind == Queen:
    return false

  // Bishops can't capture bishops
  if attacker.kind == Bishop && target.kind == Bishop:
    return false

  // Queen ↔ Bishop requires same diagonal system
  if (attacker.kind == Queen && target.kind == Bishop) ||
     (attacker.kind == Bishop && target.kind == Queen):
    return attacker.diagonal_system == target.diagonal_system

  return true
```

Integrate into `compute_queens_moves()` and `compute_bishops_moves()`.

### Move Application (`game.rs`)

When pieces move or are captured, update `piece_map`:
- `move_piece(from, to)`: relocate entry preserving metadata
- `clear_square(sq)`: remove entry
- `place_piece(sq, piece)`: add entry

## FEN Serialization (`fen.rs`)

Extend `PiecePlacement`:

```rust
pub struct PiecePlacement {
    pub square: Square,
    pub army: Army,
    pub kind: PieceKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patron: Option<PieceKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagonal_system: Option<DiagonalSystem>,
}
```

## Implementation Order

1. Add `piece_map` to Board, initialize from placements
2. Update Board methods to maintain piece_map
3. Update ArraySpec with patron/diagonal data for TABLET_OF_FIRE
4. Update promotion logic to use patron
5. Add `can_capture_piece()` helper
6. Integrate capture restrictions into move generation
7. Update FEN serialization
8. Add tests

## Test Cases

**Patron System:**
- Pawn promotes to its patron piece type
- Privileged pawn can choose any major piece
- Promotion demotes existing piece of same type

**Diagonal Networks:**
- Queen cannot capture enemy queen
- Bishop cannot capture enemy bishop
- Queen can capture bishop on same diagonal system
- Queen cannot capture bishop on different diagonal system
- Bishop can capture queen on same diagonal system
- Bishop cannot capture queen on different diagonal system
