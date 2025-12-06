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

## Concourse Formations (Rules 5.10-5.11)

Zalewski describes special formations when four pieces of the same type form a
2×2 square:

### Concourse of Bishoping (Rule 5.10)
- If three Bishops are on adjacent squares and the fourth Bishop moves to
  complete a 2×2 formation:
  - The player completing the formation **captures the two enemy Bishops**
  - **Takes control of the ally's army** (similar to seizing a throne)
- Allied Bishops move on the same diagonal course
- Enemy Bishops move on different courses
- The original text mentions "5 positions" where concourse can occur, but the
  engine implements detection for any valid 2×2 formation

### Concourse of Queens (Rule 5.11)
- Same rules as Concourse of Bishoping, but with four Queens

### Requirements for a Valid Concourse
- Exactly 4 pieces of the same type (all Bishops or all Queens) in a 2×2 square
- The formation must contain exactly 2 enemy pieces and 1 ally piece
  (plus the moving piece)
- Rooks, Knights, Pawns, and Kings cannot form a concourse

## Withdrawal (Rules 9.1-9.3)

Players may withdraw from the game under certain conditions:

1. **4-player mode (Rule 9.1-9.2):** Any player can withdraw at any time. Their
   pieces transfer to ally control, and they lose their turn in the rotation.
   If withdrawing with only a bare king, the ally takes both turns.

2. **2-player mode (Rule 9.3):** Can only withdraw an army reduced to just its
   king. The withdrawn king becomes frozen and that army loses its turn.

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
3. If you need to make the array selectable from the UI or CLI later, expose it through `arrays::ArraySpec` and call `Game::from_array_spec` (the default is `TABLET_OF_FIRE_FIRE`).

### Band layout

All arrays use a "south-to-north bands" layout where armies are stacked
horizontally across the board:

- **Blue**: Major pieces on rank 1, pawns on rank 2 (marching north)
- **Black**: Major pieces on rank 3, pawns on rank 4 (marching north)
- **Yellow**: Major pieces on rank 5, pawns on rank 6 (marching north)
- **Red**: Major pieces on rank 8, pawns on rank 7 (marching south)

Throne squares are positioned per army:
- Blue: `d1`, `e1`
- Black: `d3`, `e3`
- Yellow: `d5`, `e5`
- Red: `d8`, `e8`

### Available arrays

The engine ships with an [`ArraySpec`](src/engine/arrays.rs) registry containing
all 16 standard Enochian Chess starting positions (4 boards × 4 settings).

#### Board types

Each board has a unique turn order:

| Board | Turn Order | Notes |
| ----- | ---------- | ----- |
| **Fire** | Blue → Red → Black → Yellow | Deosil (clockwise) from South |
| **Earth** | Yellow → Blue → Red → Black | Widdershins from West |
| **Air** | Red → Yellow → Black → Blue | Widdershins from East |
| **Water** | Blue → Black → Yellow → Red | Deosil from South |

#### Settings (piece arrangements)

Fire/Earth boards use **Group 1** settings; Air/Water boards use **Group 2**:

| Setting | Group 1 (Fire/Earth) | Group 2 (Air/Water) |
| ------- | -------------------- | ------------------- |
| **Earth** | KR, B, Q, N | KR, N, Q, B |
| **Air** | KB, R, N, Q | KB, Q, N, R |
| **Water** | KQ, N, R, B | KQ, B, R, N |
| **Fire** | KN, Q, B, R | KN, R, B, Q |

The first piece in each setting is the King's partner (shares the throne).

#### Array constants

All 16 combinations are available:
- `TABLET_OF_FIRE_EARTH`, `TABLET_OF_FIRE_AIR`, `TABLET_OF_FIRE_WATER`, `TABLET_OF_FIRE_FIRE`
- `TABLET_OF_EARTH_EARTH`, `TABLET_OF_EARTH_AIR`, `TABLET_OF_EARTH_WATER`, `TABLET_OF_EARTH_FIRE`
- `TABLET_OF_AIR_EARTH`, `TABLET_OF_AIR_AIR`, `TABLET_OF_AIR_WATER`, `TABLET_OF_AIR_FIRE`
- `TABLET_OF_WATER_EARTH`, `TABLET_OF_WATER_AIR`, `TABLET_OF_WATER_WATER`, `TABLET_OF_WATER_FIRE`

Use `available_arrays()` to enumerate the registry, and `find_array_by_name(name)`
to select a specific table (see `Game::from_array_spec` in `src/engine/game.rs`).
The default array is `TABLET_OF_FIRE_FIRE`.
