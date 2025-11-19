Roadmap for enoch -- an Enochian Chess engine and TUI

---

## Phase 0 – Spec & Codebase Orientation

### 0.1 – Extract a machine-readable rules spec

**Goal:** A single source of truth (YAML/JSON + markdown) describing Enochian rules.

**Tasks (agent-sized):**

1. Create `docs/enochian-rules.md` and `docs/enochian-rules.yaml`.

2. From `Enochian Chess.html` and the Golden Dawn / Zalewski texts, encode:

   * **Board & players**

     * 8×8 board; 4 armies: Blue, Black, Red, Yellow.
     * Teams: *(Blue + Black)* vs *(Red + Yellow)*. 
     * Turn order: clockwise around the board (e.g. Blue → Red → Black → Yellow for one common array). 
   * **Setup / arrays**

     * Eight possible starting arrays (piece placements and army orientations) per Zalewski; at least one fully encoded from the article’s diagrams, then leave TODOs for the remaining ones. 
     * Mark throne squares and double-occupancy rules (initially king + another piece; if captured while two pieces remain, both are removed). 
   * **Piece types**

     * King: 1 step in any direction, like FIDE king. 
     * Queen: leaps exactly **two squares** along ranks, files, or diagonals (Alibaba-style), ignoring intervening pieces. 
     * Rook: as FIDE rook (any number of squares orthogonally). 
     * Bishop: as FIDE bishop (sliding diagonals), but with special bishop/queen capture interactions (see below). 
     * Knight: FIDE knight (2+1 leap). 
     * Pawns: one square forward only; capture diagonally; no double-step, no en passant. 
   * **Bishop/Queen interaction**
     Encode from Enochian article + Zalewski:

     * Bishops and queens run on distinct but interlocking networks of diagonals (“Aries” vs “Cancer” systems). 
     * Queens do **not** capture enemy queens, bishops do **not** capture enemy bishops; queens can capture enemy bishops and bishops can capture enemy queens, per “Concourse of Bishoping” and summary text. 
   * **King capture & check logic**

     * There is no mate; kings are **captured**, not checkmated. 
     * If your king is in check and has any legal king move, you **must** move the king, even if it stays in check. Only if the king has no legal move (blocked by friendly pieces, board edge etc.) may you move another piece. 
   * **Frozen pieces & king capture**

     * When a king is captured, all pieces of that color become **frozen**: they cannot move, threaten, or be captured, but they occupy squares as blocking terrain. 
   * **Seizing the throne / control**

     * If your king moves onto an ally’s throne square, you gain control of that army; frozen pieces revive. Control persists even if your king later leaves; if that king is captured, control reverts (if ally still has a king). 
   * **Exchange of prisoners**

     * If two enemy players each captured a king, they may agree to exchange kings back to their thrones (or nearest allowed squares) and unfreeze their armies. 
   * **Privilege pawn & promotion**

     * Standard promotion: a pawn promoted on its normal promotion rank becomes the piece of its “type” (pawn of queen → queen, etc.).
     * Privileged pawn rule: if a side is reduced to (king + queen + pawn) or (king + bishop + pawn) or (king + pawn), pawn becomes **privileged**, may promote to any major piece; if promoting to a type already in play, the existing piece is demoted back to a pawn-of-that-type. 
   * **Stalemate & draws**

     * Stalemate: if a player’s **unchecked** king has no moves except to move into check, player skips turns until stalemate is lifted. 
     * Draw conditions: e.g. both allied kings bare, or four bare kings only. 
   * **Divination mode (dice)** – mark as “Phase 5 optional”

     * Die roll selects which type of piece must be moved (1: king or pawn, 2: knight, 3: bishop, 4: queen, 5: rook, 6: pawn). 

3. Keep the YAML spec as the thing agents can reference programmatically (`docs/enochian-rules.yaml`).

---

### 0.2 – Map current engine structure

**Goal:** Understand what you’re starting from.

**Tasks:**

1. In the `enoch` fork, scan `src` for core modules:

   * Board representation (bitboards, piece enums, side-to-move).
   * Move generator(s).
   * Game state (castling, en passant, halfmove clocks, etc.).
   * TUI: modules that render ASCII/kitty board, parse input commands. ([GitHub][1])
2. Create `docs/architecture.md` with:

   * Overview diagram of existing types: `Board`, `Piece`, `Move`, `Game`, `TuiApp` (whatever they’re actually called).
   * Which modules hard-code **two colors**, FIDE pieces, or FIDE-only rules (castling/en passant/checkmate).
   * Where TUI draws the board and pieces (this is where you’ll inject Enochian art and four colors).

---

## Phase 1 – Core Data Model for Enochian Chess

### 1.1 – Introduce multi-army + team model

**Goal:** Support four armies and 2 teams in the core types.

**Tasks:**

1. Add enums in a central types module, e.g. `src/model/types.rs`:

   ```rust
   #[derive(Copy, Clone, Eq, PartialEq, Debug)]
   pub enum Army {
       Blue,
       Black,
       Red,
       Yellow,
   }

   #[derive(Copy, Clone, Eq, PartialEq, Debug)]
   pub enum Team {
       Air,   // Blue + Black
       Earth, // Red + Yellow
   }

   impl Army {
       pub fn team(self) -> Team { /* map */ }
   }

   #[derive(Copy, Clone, Eq, PartialEq, Debug)]
   pub enum PieceKind { King, Queen, Bishop, Knight, Rook, Pawn }

   #[derive(Copy, Clone, Eq, PartialEq, Debug)]
   pub struct Piece {
       pub army: Army,
       pub kind: PieceKind,
       pub pawn_type: Option<PieceKind>, // for “pawn of X” if you want to distinguish
   }
   ```

2. Replace `Color`/`Side` usage with `Army` (and `Team` where needed).

3. Introduce a `PlayerId` type decoupled from `Army` so you can handle 2-player (each controls 2 armies) or 4-player configurations.

### 1.2 – Board representation

You can keep the upstream **bitboard** approach (64-bit board, one bit per square) and extend it:

**Tasks:**

1. Define square indexing (0–63) consistent with existing code. Document mapping to Enochian orientation in `docs/enochian-coordinates.md` (e.g. a1 in one corner, etc.).

2. In the board struct, replace the “2 colors × 6 piece types” bitboards with:

   ```rust
   pub struct BitBoards {
       pub by_army_kind: [[u64; 6]; 4], // [Army][PieceKind]
       pub occupancy_by_army: [u64; 4],
       pub occupancy_by_team: [u64; 2],
       pub all_occupancy: u64,
   }
   ```

3. Add tracking for:

   * Which army currently has the move.
   * Frozen armies (a `bool` or status for each army).
   * Each army’s throne square index.
   * Whether a king is currently captured for each army.

4. Remove or mark as unused all FIDE-only fields:

   * Castling rights.
   * En passant square.
   * 50-move rule counters (you can reintroduce variant draw logic later).

### 1.3 – Game / turn model

**Goal:** Base game loop on *armies* not just “white/black”.

**Tasks:**

1. Introduce:

   ```rust
   pub struct GameConfig {
       pub armies: [Army; 4],
       pub turn_order: [Army; 4], // e.g. [Blue, Red, Black, Yellow]
       pub arrays: Vec<ArraySpec>, // eight starting arrays
   }
   ```

2. Store `current_turn_index: usize` (0..4) inside `GameState`, referencing `turn_order`.

3. Implement helper:

   ```rust
   pub fn next_army(&mut self) {
       self.current_turn_index = (self.current_turn_index + 1) % self.turn_order.len();
   }
   ```

4. Ensure move generation and application always refer to `GameState.current_army()`.

---

## Phase 2 – Legal Move Generation for Enochian Rules

### 2.1 – Piece move patterns (pseudo-legal)

**Goal:** Generate *pseudo-legal* moves for each piece, ignoring king-in-check constraints for now.

**Tasks:**

For each piece, create a dedicated module (if not already present) like `moves/king.rs`, `moves/queen.rs`, etc.

1. **King**

   * Generate 8 adjacent squares; filter off-board; forbid stepping onto allied piece. Enemies include both opposing armies; allies are same team. 
   * You may **allow** moves that result in king staying in or moving into check; legality will be filtered at a later stage by the “must move king” rule (not by check safety).

2. **Queen**

   * Precompute a lookup table `QUEEN_LEAPS[64]` of bitmasks: all squares at exactly distance 2 in rook or bishop directions from each square.
   * Moves:

     * For each destination in `QUEEN_LEAPS[sq]`, if empty → quiet move; if occupied by enemy piece → capture (subject to bishop/queen interactions, see 2.3); if occupied by ally → blocked.

3. **Rook**

   * Use existing rook sliding logic from the engine, but adapt to 4-army occupancy and ally/enemy distinction.
   * Keep standard FIDE blocking rules.

4. **Bishop**

   * Use existing bishop sliding logic, but mark which diagonals are “Aries” or “Cancer” per board geometry from the Golden Dawn text. 
   * Precompute two diagonal masks per square:

     * `ARIES_DIAGS[sq]`
     * `CANCER_DIAGS[sq]`
   * For each bishop, associate which system it belongs to (depends on starting array + army). Store that in piece metadata or a parallel table.

5. **Knight**

   * Reuse existing knight move generator (8 L-shaped leaps, unblocked). 

6. **Pawns**

   * Per army, define forward direction (a delta in square index).
   * Generate:

     * Forward move: one square forward if empty.
     * Capture moves: one square diagonally forward if occupied by enemy piece.
   * Never generate:

     * Double-step.
     * En passant.
   * Add metadata for “pawn of X” promotion type so that promotion rules can be enforced later.

### 2.2 – Promotion zones & privileged pawn

**Goal:** Implement promotion semantics from the article.

**Tasks:**

1. For each army, define a bitmask `PROMOTION_ZONE[army]` of the board’s far rank/file per rules (the opposite side from starting pawns). 
2. When generating pawn moves:

   * If destination in `PROMOTION_ZONE[army]`, create **promotion moves**.
   * Normal promotion: to the pawn’s `pawn_type` (e.g. pawn-of-queen → queen).
3. Implement privileged pawn logic (Phase 2.5 if you want to delay):

   * Function `is_privileged_pawn(state, army, pawn_square) -> bool`:

     * Count remaining non-pawn pieces for that army.
     * Check conditions: (king + queen + pawn) or (king + bishop + pawn) or (king + pawn). 
   * If privileged and promoting:

     * Allow promotion to Q/R/B/N.
     * If promoted piece type already on board:

       * Find one instance and **demote** to pawn-of-that-type (per article example). 

### 2.3 – Bishop–Queen interaction rules

**Goal:** Encode the “Concourse of Bishoping” / non-clashing diagonals.

**Tasks:**

1. In `docs/enochian-rules.md`, summarize:

   * Queens move on one set of diagonals; bishops on the complementary set; queens cannot capture queens, bishops cannot capture bishops; bishops and queens of appropriate sets can capture each other.
2. Implementation:

   * For each square, precompute which diagonal “system” it belongs to (Aries or Cancer). 
   * For each bishop/queen, store its system id.
   * When generating captures:

     * A queen may capture enemy bishops whose system matches the queen’s system and whose square is reachable by a queen leap.
     * A bishop may capture enemy queens whose system matches that bishop’s system and whose square lies on its sliding path.
     * For queen vs queen and bishop vs bishop: **filter out** capture moves regardless of reach.

### 2.4 – King capture, frozen pieces, throne logic

**Goal:** Encode all of the weird-but-fun Enochian special rules.

**Tasks:**

1. **Representation:**

   * Add to `GameState`:

     * `king_square[Army] -> Option<Square>` (None if captured).
     * `frozen[Army] -> bool`.
2. **On king capture:**

   * When an engine move lands on a king:

     * Set `king_square[that_army] = None`.
     * Set `frozen[that_army] = true`. 
3. **Frozen behavior:**

   * In move generation: if `frozen[army]` is true, **generate no moves** for that army’s pieces; they are passive blockers.
   * Treat frozen pieces as **non-threatening** and non-capturable, but still blocking for sliding/stepping pieces. 
4. **Seizing the throne:**

   * Each army has a throne square (`throne_square[Army]`).
   * If a king moves onto an *ally’s* throne square:

     * Transfer control: map that ally’s army to the current controlling player.
     * If that ally’s king is captured and their pieces are frozen, set `frozen[ally] = false`. 
   * Keep a mapping `controller[Army] -> PlayerId`.
5. **Exchange of prisoners (optional for Phase 2, can be Phase 4):**

   * Define an operation `try_exchange_kings(player_a, player_b)` implementing the article’s constraints (both have captured kings, both still have their own kings alive, etc.).
   * Put this behind a command `/exchange <enemy>` in the TUI rather than as a “move”.

### 2.5 – Move legality + check / stalemate behavior

**Goal:** Turn pseudo-legal moves into legal moves under Enochian constraints.

**Tasks:**

1. Implement an attack tester:

   ```rust
   pub fn is_square_attacked(state: &GameState, sq: Square, by_team: Team) -> bool { ... }
   ```

   respecting Enochian movement rules (especially queen/bishop).

2. **Check detection:**

   * A king is *in check* if any enemy team piece (non-frozen) attacks its square. 

3. **Legal move filtering:**

   * Generate all pseudo-legal moves for `current_army`.
   * If that army’s king is **not** in check:

     * All pseudo-legal moves are allowed (including moves that put king into check; this is permitted).
   * If that army’s king **is** in check:

     * Compute the subset of moves where the moving piece is the king.
     * If non-empty, allowed moves = that subset only (even if they keep the king in check). 
     * If empty (king surrounded by allies etc.), allowed moves = all pseudo-legal moves for other pieces.

4. **Stalemate:**

   * For a player controlling a given army, if:

     * King is **not** in check, and
     * That army has no legal moves except moves that would put an unchecked king into check,
     * Then mark that army as **stalemated**; its turns are skipped until a change in board state lifts the condition. 

5. **Victory / draw condition detection:**

   * Win for a team when both opposing kings have been captured (and not later returned by exchange). 
   * Draw rules as per Zalewski (bare kings etc.). 

---

## Phase 3 – Starting Arrays & FEN-like Notation

### 3.1 – Encode the eight arrays

**Goal:** Data-driven starting positions.

**Tasks:**

1. In `docs/enochian-arrays.md`, list the eight Zalewski arrays with diagrams from the book (just descriptive; no images, but coordinates).

2. Define a serialization format, e.g.:

   ```rust
   pub struct ArraySpec {
       pub name: String,
       pub description: String,
       pub piece_placements: Vec<(Square, Piece)>,
       pub throne_squares: [(Army, Square); 4],
       pub ptah_default_square: Option<Square>, // for divination mode later
       pub turn_order: [Army; 4],
   }
   ```

3. Implement `arrays.rs` that constructs these `ArraySpec`s from hard-coded tables.

### 3.2 – Enochian FEN or JSON

**Goal:** Allow saving/loading game states.

**Tasks:**

1. Define an `EnochFEN` string format, something like:

   ```
   board: 8/8/8/8/8/8/8/8
   armies: B:bkqrnbppppp/... etc
   turn: Blue
   turn_order: Blue,Red,Black,Yellow
   frozen: -
   kings: B:e1,R:a8,...
   array: AirOfWater
   ```

   or make it JSON.

2. Implement `GameState::to_enoch_fen()` and `GameState::from_enoch_fen()`.

3. Wire this into CLI commands:

   * `/save <path>`
   * `/load <path>`

---

## Phase 4 – TUI Adaptation for Enochian Chess

### 4.1 – Board rendering

**Goal:** Show a four-army, colored, Enochian themed board in the terminal.

**Tasks:**

1. Identify current board drawing code (probably uses Unicode pieces and colored squares). ([GitHub][2])

2. Introduce:

   * A per-army color palette (terminal colors) matching the Enochian boards (e.g. Blue = Air, Red = Fire, Black = Water, Yellow = Earth, or however you like, consistent with your board textures). 
   * Glyphs for pieces (e.g. use uppercase letter for piece type plus subscript/colored background for army).

3. Render:

   * Board from a fixed perspective (e.g. corners labelled A1..H8).
   * A sidebar showing whose turn it is, which armies are frozen, king locations, and team status.
   * Throne squares with a special background/frame.
   * Optional Ptah square marker for divination.

4. Expose a config option for rendering style:

   * Simple ASCII.
   * Unicode + ANSI colors.
   * Kitty-image mode later if you want to keep some of `enoch`’s flair.

### 4.2 – Input syntax

**Goal:** Provide a friendly move language that agents and humans can both use.

**Tasks:**

1. Define a move syntax, e.g.:

   * `blue: a3-a4`
   * `red: c1xb4` (capture)
   * `yellow: e7-e8=Q` (promotion)

2. Implement parser:

   * Extract army name before colon.
   * Parse from/to squares and optional promotion suffix.
   * Ensure the parsed army matches `current_army` (or allowed “operate both” rules for 2-player mode).

3. Extend command set:

   * `/new <array_name> [players=2|4]`
   * `/arrays` (list arrays)
   * `/status`
   * `/exchange <opponent_army>` (for prisoner exchanges)
   * `/withdraw` (for player withdrawal as per article). 

---

## Phase 5 – Engine Extras & Divination Mode

This is optional but natural given the sources you provided.

### 5.1 – Simple evaluation & AI

**Goal:** Let `enoch` play itself or play vs one human controlling two armies.

**Tasks:**

1. Collapse team states into a “side to evaluate”: treat `(Blue + Black)` as one side, `(Red + Yellow)` as the other when evaluating positions.
2. Implement a simple evaluation heuristic:

   * Material values tuned to Zalewski’s observed piece strengths (rook heavy in endgame; queen comparatively weak etc.). 
3. Implement a very shallow minimax or Monte-Carlo playout engine that chooses moves for both armies on a team.

### 5.2 – Dice-driven divination mode

**Goal:** Implement the die-selected move rules from Golden Dawn for a “Y paper mode”. 

**Tasks:**

1. Add a `Mode` enum: `Mode::Normal`, `Mode::Divination`.
2. In `Mode::Divination` on each army’s turn:

   * Roll d6 (RNG).
   * Map die to piece type (1 → King or Pawn; 2 → Knight; 3 → Bishop; 4 → Queen; 5 → Rook; 6 → Pawn). 
   * Filter legal moves to only moves whose *moving piece kind* matches the die selection (including king+pawn on 1).
   * If no such move exists, re-roll a limited number of times or fall back to “skip” (document decision).
3. Display the die result in the TUI.

---

## Phase 6 – Testing, Fixtures, and Agent-Friendly Tasks

### 6.1 – Unit tests for movement

**Goal:** Bulletproof the weird rules before you chase UI polish.

**Tasks:**

1. Create `tests/enoch_moves.rs` with table-driven tests:

   * Queen leap patterns from center, edge, and corner squares.
   * Bishop diagonals limited to proper Aries/Cancer systems.
   * Pawn forward/capture moves for each army orientation.
   * King capture → frozen pieces.

2. Create `tests/enoch_rules.rs`:

   * Check situations where a king is in check and:

     * Has multiple king moves → only king moves are legal.
     * Has no king moves → other moves are allowed.
   * Frozen army cannot move and does not give check.
   * Exchange of prisoners restores mobility.

### 6.2 – Golden Dawn position fixtures

**Goal:** Use Zalewski / Regardie examples as regression tests.

**Tasks:**

1. Encode a few sample positions from Zalewski’s annotated games into `EnochFEN`, store in `tests/data/*.fen`. 
2. For each:

   * Assert expected legal moves, promotions, or victory conditions.

### 6.3 – Make it pleasant for small agents

**Goal:** Make your repo “agent-friendly”.

**Tasks:**

1. Add a top-level `ROADMAP.md` (you can paste/trim this plan) with:

   * Current phase.
   * Open TODOs as checkboxes (GitHub formatted).
2. Add `CONTRIBUTING.md` that:

   * Points agents/humans to `docs/enochian-rules.yaml`, `docs/architecture.md`, and `docs/enochian-arrays.md`.
   * Lists typical “good first tasks” (e.g. “implement Bishop–Queen capture filtering”, “add TUI command `/arrays`”).

---

[1]: https://github.com/monistowl/enoch "GitHub - monistowl/enoch: Enochian Chess"
[2]: https://github.com/ronaldsuwandi/enoch "GitHub - ronaldsuwandi/enoch: A Rust-powered  chess engine in a terminal"

