# Enochian Chess Zola Site Design

**Date**: 2025-05-28
**Status**: Approved

## Overview

A static documentation site for Enochian Chess built with Zola, featuring an elegant occult aesthetic that embraces the game's Golden Dawn origins while remaining accessible to chess variant enthusiasts with no prior esoteric knowledge.

## Audience & Tone

- **Primary audience**: Chess variant enthusiasts (cold-start friendly)
- **Tone**: Elegant and mysterious with stylistic flair
- **Approach**: Progressive disclosure — quick-start for newcomers, deep reference for enthusiasts

## Technology Stack

- **Generator**: Zola (Rust-based static site generator)
- **Theme**: Custom theme called `grimoire`
- **Fonts**: Cormorant Garamond (Google Fonts), JetBrains Mono for code
- **JavaScript**: None required for docs phase (pure CSS)

## Directory Structure

```
site/
├── config.toml
├── content/
│   ├── _index.md            # Home page
│   ├── quick-start.md       # 5-minute intro
│   ├── rules/
│   │   ├── _index.md        # Full rules overview
│   │   └── pieces/
│   │       ├── _index.md    # Pieces overview
│   │       ├── king.md
│   │       ├── queen.md
│   │       ├── bishop.md
│   │       ├── rook.md
│   │       ├── knight.md
│   │       └── pawn.md
│   └── (future: arrays/, history/, engine/)
├── static/
│   ├── css/
│   ├── images/
│   └── fonts/
└── themes/grimoire/
    ├── templates/
    └── sass/
```

## URL Structure

- `/` — Home
- `/quick-start/` — Learn in 5 minutes
- `/rules/` — Full rules reference
- `/rules/pieces/` — Piece catalog
- `/rules/pieces/queen/` — Individual piece pages

## Visual Design System

### Color Palette

**Base (dark neutral):**
- `--bg-deep: #0a0a0f` — Page background
- `--bg-surface: #12121a` — Card/section backgrounds
- `--bg-elevated: #1a1a24` — Hover states, borders
- `--text-primary: #e8e4dc` — Body text (warm cream)
- `--text-muted: #8a8680` — Secondary text

**Elemental accents (four armies):**
- `--air-blue: #4a7cb8` — Blue army / Air
- `--water-black: #2d2d3a` — Black army / Water
- `--fire-red: #c45a4a` — Red army / Fire
- `--earth-yellow: #c4a24a` — Yellow army / Earth

### Typography

- **Display/Body**: Cormorant Garamond (serif)
- **Monospace**: JetBrains Mono (move notation, code)
- **Scale**: 1.25 ratio (major third)
- **Body size**: 18px for comfortable reading

## Layout

- **Max content width**: 720px
- **Navigation**: Fixed sidebar on desktop, hamburger on mobile
- **Header**: Minimal — title with elemental underline
- **Footer**: Light — GitHub link, thematic flourish

## Components

### Navigation Sidebar
- Section headers with elemental color accents
- Current page highlighted with colored left border
- Collapsible on mobile (<768px)

### Piece Cards
- Dark surface with elemental border
- Piece glyph, name, one-line summary
- Used on pieces index page

### Callout Boxes
- Left border in elemental colors
- Types: note (blue), warning (red), tip (yellow), important (gradient)

### Board Diagrams
- SVG-based with army-colored pieces
- Reuse existing `/assets/sprite/` SVGs
- Captioned explanations

### Move Notation
- Monospace, subtle background
- Inline code style: `Qd1-d3`

## Content Plan

### Home Page
- Atmospheric hero with title
- Hook: "A four-army chess variant from the Victorian occult tradition"
- Three navigation cards
- Stylized board preview

### Quick Start
1. The Board & Armies
2. Turn Order
3. How Pieces Move (brief)
4. How to Win
5. One Special Rule (frozen armies)

### Full Rules
- Adapted from `docs/enochian-rules.md`
- Organized sections with anchor links
- Complete coverage: diagonal networks, thrones, privileged pawns, etc.

### Pieces Section
- Index with six piece cards
- Individual pages with movement diagrams and special rules

## Implementation Phases

### Phase 1: Foundation
1. Initialize Zola site in `/site/`
2. Create grimoire theme with base templates
3. Set up Sass variables and typography
4. Build navigation component

### Phase 2: Core Pages
5. Home page with hero
6. Quick Start guide
7. Full Rules (adapted from existing docs)
8. Pieces index and individual pages

### Phase 3: Polish
9. Board diagrams (SVG)
10. Responsive testing
11. Typography/spacing refinement

### Future Phases
- Starting Arrays pages
- History/Origins page
- Developer documentation
- **Web game interface** (Rust→WASM)

## Deployment

- Output: `/site/public/`
- Compatible with GitHub Pages, Netlify, Cloudflare Pages
