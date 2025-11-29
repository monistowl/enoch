# Modular Widget Architecture Design

## Overview

Design for nestable, modular Alpine.js widgets that support documentation embeds, interactive tutorials, and live game viewing.

## Component Architecture

**Core Pattern: Parent Game State + Child Widgets**

```
enochGame(scenario)           <- Parent: owns all game state
  ├── boardDisplay()          <- Renders board, pieces, handles clicks
  ├── moveHistory()           <- Shows move list with notation
  ├── capturedPieces(team)    <- Shows captured pieces for one team
  ├── turnIndicator()         <- Shows current turn + game status
  └── gameControls()          <- Navigation buttons (prev/next/play)
```

State flows down, events flow up. Child widgets access parent state via Alpine's `$data`.

**File structure:**
```
site/static/js/
  enoch-game.js      <- Parent component + game logic
  enoch-board.js     <- Board rendering (refactored from board.js)
  enoch-widgets.js   <- Small widgets: history, captures, turn, controls
```

**Shortcode mapping:**
- `{{ game() }}` - Full widget with all children
- `{{ board() }}` - Board only (lightweight, current behavior)
- `{{ board_with_controls() }}` - Board + nav controls

## Game State Model

**Parent state (enochGame):**
- `position[]` - 64-element array of pieces
- `captures` - `{ Air: [], Earth: [] }` captured pieces by team
- `frozen[]` - which armies are frozen
- `turn` - current army
- `step` - current step index
- `status` - 'playing', 'check', 'checkmate', 'stalemate', 'draw'

## Extended Scenario Format

```json
{
  "mode": "tutorial",
  "initialPosition": { "d4": "BB", "f6": "RB", "e5": "KK" },
  "initialState": {
    "frozen": ["Red"],
    "captured": { "Air": ["RN"], "Earth": [] }
  },
  "steps": [
    {
      "move": ["d4", "f6"],
      "narrative": "Blue captures Red Bishop!",
      "highlight": ["d4", "f6"]
    }
  ]
}
```

**Computed state logic (JS, pre-WASM):**

When a move is applied:
1. Capture detection: If target square has enemy piece -> add to `captures[team]`
2. King capture: If captured piece is King -> set `frozen[army] = true`
3. Turn advance: Cycle through unfrozen armies (Blue -> Black -> Red -> Yellow)

## Widget Specifications

### boardDisplay()
- Renders 8x8 grid with pieces
- Shows frozen pieces with reduced opacity + desaturated color
- Handles click events, emits to parent
- Displays legal move indicators, last move highlight
- Standalone-capable: works without parent enochGame

### capturedPieces(team)
- Takes team parameter ('Air' or 'Earth')
- Renders piece glyphs in army colors
- Compact horizontal layout

### turnIndicator()
- Shows army name with color badge
- Status overlay: "Check!", "Stalemate", "Blue + Black win!"

### gameControls()
- Prev/Next/Play buttons
- Step counter: "3 / 12"
- Keyboard support: arrows, space

### moveHistory() (deferred)
- Scrollable move list
- Click to jump to position

## Implementation Phases

### Phase 1: Refactor to modular structure
- Extract enochGame() parent component
- Keep boardDisplay() as current rendering
- Extract gameControls()
- Maintain backward compatibility

### Phase 2: Frozen army visuals
- Track frozen[] state from king captures
- CSS: opacity 0.4, grayscale filter
- Update applyMove() for detection

### Phase 3: Captured pieces widget
- Create capturedPieces(team) component
- Track captures as moves applied
- Horizontal glyph display

### Phase 4: Turn indicator widget
- Create turnIndicator() component
- Army color badge
- Game status display

### Phase 5: Full game shortcode
- New {{ game() }} shortcode
- Composed layout: board center, captures sides, controls below
