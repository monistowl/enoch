# Interactive Board Widget Design

**Date:** 2025-11-28
**Status:** Approved

## Overview

Add interactive chess board functionality to the Enochian Chess documentation site, progressing through phases:

1. **GIF demo** for README (portable, works everywhere)
2. **Linear tutorials** with click-through walkthroughs
3. **Scenario challenges** with win/fail validation (stretch)
4. **WASM sandbox** with full engine integration (future)

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Zola Site                                              │
│  ┌──────────────────┐  ┌─────────────────────────────┐  │
│  │  Static Pages    │  │  Interactive Board Widget   │  │
│  │  (rules, pieces) │  │  (Alpine.js + SVG)          │  │
│  │                  │  │                             │  │
│  │  {% include     │◄─┤  - Renders board state      │  │
│  │    board.html %} │  │  - Handles click events    │  │
│  └──────────────────┘  │  - Animates moves          │  │
│                        │  - Shows legal moves       │  │
│  ┌──────────────────┐  │                             │  │
│  │  /learn section  │◄─┤  Scenarios defined as JSON │  │
│  │  (tutorials,     │  │  arrays in page frontmatter│  │
│  │   puzzles)       │  └─────────────────────────────┘  │
│  └──────────────────┘                                   │
└─────────────────────────────────────────────────────────┘
```

**Key decisions:**
- Single reusable `board.html` partial with Alpine component
- Game states/moves defined as JSON in Zola frontmatter
- No build step — Alpine loaded via CDN
- Board styling inherits from existing `_board.scss` + theme system

## Board Widget Component

**Alpine component structure:**

```html
<div x-data="enochBoard(scenario)" class="board-widget">
  <svg viewBox="0 0 400 400" class="board-widget__svg">
    <!-- 64 clickable squares -->
    <!-- Pieces rendered from state -->
    <!-- Legal move indicators -->
  </svg>

  <div class="board-widget__controls">
    <button @click="prevStep()">← Back</button>
    <span x-text="stepLabel"></span>
    <button @click="nextStep()">Next →</button>
  </div>

  <p class="board-widget__narrative" x-text="currentNarrative"></p>
</div>
```

**State managed by Alpine:**
- `position` — 64-square array of piece codes
- `selected` — currently selected square (or null)
- `legalMoves` — squares the selected piece can move to
- `step` — current tutorial step index
- `scenario` — full scenario data (steps, narratives, expected moves)

## Scenario Data Format

```json
{
  "title": "The King's Escape",
  "description": "Blue King must flee check from Red Queen",
  "initialPosition": {
    "e1": "BK", "d8": "RQ", "a1": "BR", "h8": "YK"
  },
  "steps": [
    {
      "narrative": "Blue is in check. The King must move.",
      "turn": "Blue",
      "highlight": ["e1"],
      "legalMoves": {"e1": ["e2", "f1", "f2"]}
    },
    {
      "narrative": "Good! The King escapes to f2.",
      "expectedMove": ["e1", "f2"],
      "autoAdvance": true
    }
  ],
  "mode": "tutorial"
}
```

**Three scenario modes:**
- `demo` — Auto-plays through moves (homepage, embedded examples)
- `tutorial` — User clicks through with guidance and legal move hints
- `puzzle` — User must find correct move(s), validates win/fail

**Position encoding:**
- Two-char codes: `BK` (Blue King), `RP` (Red Pawn), `YQ` (Yellow Queen)
- Square names: algebraic notation (`a1`–`h8`)
- Empty squares omitted from position object

## File Structure

```
site/
├── static/
│   └── js/
│       └── board.js          # Alpine component logic
├── themes/grimoire/
│   ├── templates/
│   │   ├── partials/
│   │   │   └── board.html    # Reusable board widget
│   │   └── learn/
│   │       ├── single.html   # Tutorial page template
│   │       └── list.html     # Learn section index
│   └── sass/
│       └── _board.scss       # (extend existing)
└── content/
    └── learn/
        ├── _index.md         # Learn section landing
        ├── basics.md         # Tutorial: basic moves
        ├── check.md          # Tutorial: check & escape
        └── puzzles/
            └── puzzle-1.md   # Challenge scenario
```

## GIF Generation

**Automated workflow using Playwright:**

1. Create standalone demo page with auto-playing scenario
2. Capture frames with Playwright screenshots
3. Assemble with `gifski` for high-quality output

```bash
# Capture frames
playwright screenshot http://localhost:1111/demo --wait-for-timeout=100

# Assemble GIF
gifski -o demo.gif --fps 4 frame-*.png
```

**Output locations:**
- `static/images/demo.gif` — embedded in README
- `content/demo.md` — hidden page for recording

## Implementation Phases

### Phase 1: Foundation
- Add Alpine.js to base template
- Create `board.js` with core rendering logic
- Create `board.html` partial
- Extend `_board.scss` for widget styling

### Phase 2: Demo Mode
- Implement auto-play functionality
- Create demo scenario for homepage
- Set up GIF capture workflow
- Generate README demo GIF

### Phase 3: Tutorial Mode
- Add step navigation controls
- Implement click-to-select squares
- Show legal move highlights
- Create `/learn` section with initial tutorials

### Phase 4: Puzzle Mode (Stretch)
- Add move validation
- Implement win/fail detection
- Create puzzle scenarios

### Phase 5: WASM Integration (Future)
- Compile Rust engine to WASM
- Replace hardcoded legal moves with engine calls
- Enable full sandbox play
