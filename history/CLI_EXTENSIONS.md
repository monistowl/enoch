# CLI Extensions & Ergonomics Ideas

## Current State Analysis

**Existing CLI Operations:**
- ✅ validate, analyze, query, generate, perft, convert
- ✅ move, show, status, legal-moves
- ✅ ai, auto-play
- ✅ State management via --state flag

**Gaps & Opportunities:**

## 1. Array/Position Management

### `--list-arrays`
List all available starting arrays with descriptions.
```bash
enoch --headless --list-arrays
# Output:
# 1. Air (default) - Air team moves first
# 2. Water - Water team moves first
# 3. Fire - Fire team moves first
# ...
```

### `--array <name>`
Start a game with a specific array.
```bash
enoch --headless --array water --state game.json
```

**Use case:** Testing different starting positions, learning array variations.

## 2. Move History & Undo

### `--history`
Show move history from a game state.
```bash
enoch --headless --state game.json --history
# Output:
# 1. Blue: e2-e3
# 2. Red: e7-e6
# 3. Black: a3-a4
```

### `--undo [N]`
Undo last N moves (default 1).
```bash
enoch --headless --state game.json --undo 2
```

**Use case:** Exploring variations, correcting mistakes, game analysis.

## 3. Position Evaluation

### `--evaluate`
Evaluate current position (material count, piece activity, control).
```bash
enoch --headless --state game.json --evaluate
# Output:
# Material:
#   Blue: K+Q+2R+2N+2B+8P = 39
#   Red: K+Q+2R+2N+2B+6P = 37
# Mobility:
#   Blue: 15 legal moves
#   Red: 12 legal moves
# Status:
#   Blue: Active, not in check
#   Red: Active, in check
```

**Use case:** Position analysis, learning, AI training data.

## 4. Game Replay & Export

### `--replay`
Replay a game move-by-move with optional delay.
```bash
enoch --headless --state game.json --replay --delay 1000
```

### `--export-pgn`
Export game to PGN-like format (adapted for 4-player).
```bash
enoch --headless --state game.json --export-pgn > game.pgn
```

### `--import-pgn`
Import game from PGN format.
```bash
enoch --headless --import-pgn game.pgn --state game.json
```

**Use case:** Sharing games, analysis, archiving.

## 5. Interactive Analysis Mode

### `--interactive`
Enter interactive analysis mode (REPL-style).
```bash
enoch --headless --interactive --state game.json
> analyze e2
> validate blue: e2-e3
> make blue: e2-e3
> status
> quit
```

**Use case:** Exploratory analysis, learning, debugging.

## 6. Batch Operations

### `--batch <file>`
Execute multiple commands from a file.
```bash
# commands.txt:
# generate "Ke1:blue Ke8:red"
# move "blue: e1-e2"
# move "red: e8-e7"
# status

enoch --headless --batch commands.txt --state game.json
```

**Use case:** Scripting, testing, automation.

## 7. Position Search & Filtering

### `--find-positions`
Find positions matching criteria.
```bash
enoch --headless --find-positions "blue in check" --state game.json
enoch --headless --find-positions "material equal"
```

**Use case:** Position databases, pattern recognition.

## 8. Notation Improvements

### `--notation <style>`
Choose notation style (algebraic, descriptive, coordinate).
```bash
enoch --headless --notation algebraic --show
# vs
enoch --headless --notation coordinate --show
```

**Use case:** User preference, compatibility with other tools.

## 9. Validation & Linting

### `--lint`
Check game state for inconsistencies or rule violations.
```bash
enoch --headless --state game.json --lint
# Output:
# ✓ All piece positions valid
# ✓ Turn order correct
# ✓ No illegal frozen piece positions
# ⚠ Warning: Unusual material imbalance
```

**Use case:** Debugging, ensuring state integrity.

## 10. Statistics & Analytics

### `--stats`
Show game statistics.
```bash
enoch --headless --state game.json --stats
# Output:
# Moves played: 42
# Captures: 8 (Blue: 3, Red: 2, Black: 2, Yellow: 1)
# Checks: 5
# Average move time: 2.3s
# Longest sequence without capture: 12 moves
```

**Use case:** Game analysis, learning patterns.

## 11. Puzzle Mode

### `--puzzle`
Generate tactical puzzles from positions.
```bash
enoch --headless --state game.json --puzzle
# Output:
# Blue to move. Find the winning sequence.
# Hint: Look for a fork with the knight.
```

**Use case:** Training, learning tactics.

## 12. Opening Book

### `--opening-book`
Query opening book for recommended moves.
```bash
enoch --headless --state game.json --opening-book
# Output:
# Recommended: blue: e2-e3 (played in 67% of games)
# Alternative: blue: d2-d3 (played in 23% of games)
```

**Use case:** Learning openings, AI improvement.

## Priority Ranking

### High Priority (Most Useful)
1. **--list-arrays** - Essential for discovering starting positions
2. **--array <name>** - Start games with different arrays
3. **--history** - View move history
4. **--evaluate** - Position evaluation for learning
5. **--interactive** - REPL mode for exploration

### Medium Priority (Nice to Have)
6. **--undo** - Undo moves for analysis
7. **--batch** - Scripting support
8. **--stats** - Game statistics
9. **--export-pgn / --import-pgn** - Game sharing

### Low Priority (Future)
10. **--replay** - Animated replay
11. **--lint** - State validation
12. **--puzzle** - Tactical training
13. **--opening-book** - Opening theory

## Implementation Complexity

**Easy (< 50 lines):**
- --list-arrays
- --array
- --history
- --stats (basic)

**Medium (50-150 lines):**
- --evaluate
- --undo
- --batch
- --export-pgn

**Hard (150+ lines):**
- --interactive (REPL)
- --import-pgn
- --puzzle
- --opening-book

## Recommended Next Steps

**Phase 1: Essential Discovery**
1. Implement `--list-arrays` and `--array` for array discovery
2. Add `--history` for move history viewing

**Phase 2: Analysis Tools**
3. Implement `--evaluate` for position evaluation
4. Add `--undo` for move takeback

**Phase 3: Advanced Features**
5. Build `--interactive` REPL mode
6. Add `--batch` for scripting

These would significantly improve CLI ergonomics for:
- Learning the game (arrays, history, evaluate)
- Analysis (undo, interactive)
- Automation (batch)
- Integration (export/import)
