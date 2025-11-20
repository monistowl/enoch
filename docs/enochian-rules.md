# Enochian Chess Rules

This document captures the rule set that guides the migration of **enoch** from a
FIDE PGN trainer to a faithful implementation of Golden Dawn / Zalewski style
Enochian chess. The intent is to make the rules machine-readable so the engine,
UI, and tests can rely on a single source of truth.

## Board Geometry & Coordinates

- **Board size:** 8×8 (64 squares). All examples below use algebraic
  coordinates with `a1` at the lower-left corner.
- **Indexing:** Bitboards follow the ordinary chess convention where `a1` is
  bit `0`, files increase to the right, and ranks increase upward.
- **Orientation:** The canonical clockwise turn order is
  `Blue → Red → Black → Yellow`. Blue begins on the southern edge, Red on the
  eastern edge, Black on the northern edge, and Yellow on the western edge.
- **Throne squares:** Each army owns two throne squares. Kings can share a
  throne with one allied piece. If an enemy captures a double-occupied throne,
  **both** occupants are removed. Thrones double as the default return squares
  for exchanged kings.

| Army   | Throne squares | Notes |
| ------ | -------------- | ----- |
| Blue   | `d1`, `e1`     | Southern (Air) throne. Allied with Black. |
| Red    | `e8`, `d8`     | Eastern/Fire throne. Allied with Yellow. |
| Black  | `a5`, `a4`     | Northern/Water throne. Allied with Blue. |
| Yellow | `h4`, `h5`     | Western/Earth throne. Allied with Red. |

> ⚠️ The throne coordinates above follow Zalewski’s diagrams. If we import an
> array with a different orientation, the YAML spec must override these values.

## Armies, Teams, and Promotion Zones

| Army   | Element | Team  | Home sector (array default) | Pawn direction | Promotion zone |
| ------ | ------- | ----- | --------------------------- | -------------- | -------------- |
| Blue   | Air     | Air   | South (files `a`–`h`, ranks `1`–`2`) | +8 (north) | Rank 8 |
| Red    | Fire    | Earth | East (files `g`–`h`, ranks `1`–`8`)  | −1 (west)  | File `a` |
| Black  | Water   | Air   | North (files `a`–`h`, ranks `7`–`8`) | −8 (south) | Rank 1 |
| Yellow | Earth   | Earth | West (files `a`–`b`, ranks `1`–`8`)  | +1 (east)  | File `h` |

- **Teams:** Blue + Black (Team *Air*) vs Red + Yellow (Team *Earth*).
- **Controllers:** In two-player games each human controls a team (two armies).
  In four-player games, each human controls a single army, but teams still win
  or lose collectively.
- **Turn order overrides:** Certain Zalewski arrays start with a different
  compass rotation (e.g., Blue → Yellow → Black → Red). When that happens, the
  YAML spec must declare the specific order for that array.

## Piece Catalogue

### King
- Moves exactly one square in any direction.
- Kings are captured rather than checkmated.
- When a king is threatened and has at least one legal king move, that army
  **must** move its king. (It may even remain in check as long as the move is
  legal.)
- If the king has no legal moves, other pieces may act even while the king is
  checked.

### Queen (Alibaba leaper)
- Jumps exactly **two squares** orthogonally or diagonally, ignoring blockers.
- Cannot capture enemy queens.
- Captures enemy bishops only if both pieces share the same diagonal system
  (see *Diagonal Networks* below).

### Bishop
- Slides along diagonals within its assigned network.
- Bishops never capture enemy bishops, but **can** capture queens whose diagonal
  system matches the bishop’s own system.
- Each bishop starts either on the Aries network or the Cancer network. The
  arrays determine this assignment.

### Rook
- Standard orthogonal slider. Blocks halt movement just like FIDE rooks.

### Knight
- 2+1 “L” leaper. Knights ignore blocking pieces.

### Pawn
- Moves one square “forward” in the army’s orientation (see table above).
- Captures one square diagonally forward.
- No double-step, en passant, or initial push variants.
- Each pawn is tied to a **patron** piece type (e.g., “pawn of queen”). Upon
  promotion it becomes that patron piece.
- **Privileged pawn:** If an army is reduced to `king + queen + pawn`,
  `king + bishop + pawn`, or just `king + pawn`, that pawn becomes privileged.
  A privileged pawn may promote to any major piece. If it promotes to a type
  already on the board, the existing piece is demoted back into a pawn of that
  type.

## Diagonal Networks (Aries vs Cancer)

Zalewski describes two intertwined diagonal lattices:

- **Aries network:** Squares matching bitmask `0x55AA55AA55AA55AA`. These are
  the light + dark diamonds that queens of the Aries set leap through.
- **Cancer network:** Squares matching mask `0xAA55AA55AA55AA55`. These are the
  complementary diagonals.

Queens and bishops are permanently attached to one of the two systems. Arrays
define the attachment per-piece. Legal move generation must ensure captures only
occur across compatible systems.

## Frozen Armies, Thrones, and Control

1. **King capture:** When a king is captured, all pieces belonging to that army
   become **frozen**. Frozen pieces occupy squares but may not move, attack, or
   be captured.
2. **Seizing a throne:** Moving your king onto an allied throne transfers
   control of that allied army to you. If that army was frozen, it becomes
   active again.
3. **Control persistence:** Once gained, control stays with the seizing king
   even if the king later leaves the throne. If the controlling king is
   captured, control reverts to the ally (provided their king is still alive).
4. **Exchange of prisoners:** If two opposing players each captured a king, they
   may mutually agree to exchange prisoners. The returned kings are placed on
   their throne or the nearest legal square and their armies thaw. This is a
   negotiated action, not an automatic move.

## Checks, Forced King Moves, and Stalemate

- **Check detection:** A king is *in check* if any unfrozen opposing piece
  attacks its square.
- **Forced king moves:** While checked, only king moves are legal **if** at
  least one such move exists. If there is no legal king move, the army may move
  other pieces even though the king remains in check.
- **Stalemate:** If a non-checked king has no legal moves that keep it unchecked,
  that army skips turns until the stalemate is broken (e.g., by an ally moving
  or an enemy capture that frees squares).

## Victory and Draw Conditions

- A team wins when both opposing kings have been captured and not returned via
  prisoner exchange.
- **Draws:**
  - Both allied kings are bare.
  - Only four bare kings remain on the board.
  - Players mutually agree to halt after an unresolved stalemate cycle.

## Divination (Optional Mode)

Divination mode introduces a d6 roll that constrains each move:

| Die | Pieces forced to move |
| --- | --------------------- |
| 1   | King **or** pawn |
| 2   | Knight |
| 3   | Bishop |
| 4   | Queen |
| 5   | Rook |
| 6   | Pawn |

Re-rolls are allowed if no piece of the rolled type has a legal move, up to a
configurable retry limit.

## Starting Arrays

Historical sources provide eight “Tablet” arrays (e.g., *Air of Fire*, *Water of
Earth*). Each array defines:

- Which army occupies each compass direction.
- The turn order (clockwise or counter-clockwise) and starting player.
- Throne positions (sometimes rotated relative to the base table).
- Piece placements, including which diagonals each bishop/queen belongs to.

### Encoding format

The YAML companion file captures arrays via:

- `turn_order`: ordered list of armies.
- `controller_map`: mapping from player slots to armies they control.
- `throne_squares`: overrides for each army if they differ from defaults.
- `piece_placements`: list of `(square, army, kind, diagonal_system)` tuples.

### Adding a new starting array

1. Extend `src/engine/arrays.rs` with a new `ArraySpec` entry:
   * Reference `Army::ALL` order when you supply `throne_squares` and `controller_map`.
   * Provide a `turn_order` array that matches the desired clockwise/counterclockwise order.
   * Supply `promotion_zones` (you can reuse `DEFAULT_PROMOTION_ZONES` from `board.rs`).
   * List the piece placements as bitboards (`Square` indices converted to `1u64 << square`).
2. Update the YAML spec (`docs/enochian-rules.yaml`) so agents can parse the new array data programmatically (name, description, placements, throne overrides).
3. If you need to make the array selectable from the UI or CLI later, expose it through `arrays::ArraySpec` and call `Game::from_array_spec` (the default entry is `TABLET_OF_FIRE_PROTOTYPE`).

### Example: Tablet of Fire (prototype transcription)

The repository currently includes a **prototype transcription** for the Tablet of
Fire array so the engine has concrete data to parse. It preserves the canonical
turn order `Blue → Red → Black → Yellow`, but to keep the layout conflict-free
until we import the historical diagrams, armies are stacked in south-to-north
bands:

- Blue major pieces occupy rank 1 and blue pawns occupy rank 2.
- Black major pieces occupy rank 3 and black pawns occupy rank 4 (they still
  march “north” toward rank 8 even though they start in the south-central band).
- Yellow major pieces live on rank 5 with pawns on rank 6.
- Red major pieces occupy rank 8 and pawns occupy rank 7.
- Thrones are still respected (e.g., `d1/e1` for Blue, `e8/d8` for Red), so the
  sample array keeps the bookkeeping hooks we need for sequestration and
  prisoner exchanges.

> 🔎 **TODO:** Replace the prototype layout with the exact Zalewski diagrams once
> the project imports `Enochian Chess.html`. See `enoch-3py`.

The YAML file mirrors this information so code can load the array without
scraping the Markdown.

### Available arrays

The engine ships with an [`ArraySpec`](src/engine/arrays.rs) registry so you can
instantiate any documented layout.

- **Tablet of Fire (prototype)** – full placements derived from the prototype transcription in this repo. Used by `Game::default()`.
- **Tablet of Water (placeholder)** – clockwise turn order `Blue → Black → Yellow → Red`; layout data still needs transcription.
- **Tablet of Air (placeholder)** – counter-clockwise order `Red → Yellow → Black → Blue`; waiting on diagram details.
- **Tablet of Earth (placeholder)** – counter-clockwise order `Yellow → Blue → Red → Black`; layout TBD.

Use the `available_arrays()` helper in `src/engine/arrays.rs` to enumerate the
registry, and `find_array_by_name(name)` to select a specific table (see `Game::from_array_spec` in `src/engine/game.rs`).
