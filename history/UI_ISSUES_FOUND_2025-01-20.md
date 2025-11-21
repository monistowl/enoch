# UI Issues Found from Playtesting

## Issues Discovered

### 1. **No Visual Feedback for Invalid Moves**
When a move fails (like Black g8-f6), the error message appears but:
- The selected square stays highlighted
- User doesn't know WHY the move failed
- No indication of what moves ARE legal

**Fix:** Clear selection on error and show specific reason

### 2. **No Move History Display**
After several moves, there's no way to see what happened:
- Can't review the game
- Hard to learn from mistakes
- No notation for sharing games

**Fix:** Add move history panel showing last N moves

### 3. **Confusing Square Notation**
The board shows files as A-H but:
- Users might not know chess notation
- No visual guide for coordinates
- Easy to mistype

**Fix:** Add coordinate hints or click-to-select

### 4. **No Undo**
Made a mistake? Too bad!
- Can't explore variations
- Punishing for new players
- No way to fix typos

**Fix:** Implement undo/redo stack

### 5. **Army Selector Takes Vertical Space**
The army selector bar uses a full line:
- Reduces board size on small terminals
- Could be more compact
- Information is redundant with status panel

**Fix:** Make it more compact or integrate into header

### 6. **No Indication of Whose Turn It Is**
The turn indicator is in the board title:
- Easy to miss
- Not prominent enough
- Should be more obvious

**Fix:** Make turn indicator more prominent

### 7. **Legal Moves Not Shown Until Selection**
You have to select a piece to see where it can go:
- Trial and error
- Frustrating for beginners
- Slows down play

**Fix:** Add "show all legal moves" mode

### 8. **No Captured Pieces Display**
Can't see what's been captured:
- Hard to evaluate position
- Missing strategic information
- No sense of material advantage

**Fix:** Add captured pieces panel

### 9. **Status Messages Disappear**
Success/error messages vanish on next action:
- Easy to miss
- No log of what happened
- Can't review errors

**Fix:** Add message history or persistent log

### 10. **No Quick Restart**
To start a new game:
- Must quit and restart
- Or manually load array
- Cumbersome

**Fix:** Add /restart or /new command

## Quick Wins (Implement Now)

### A. Better Error Handling
```rust
// Clear selection on error
if move_result.is_err() {
    self.selected_square = None;
}

// Show more specific errors
"Invalid move: Knight at g8 cannot reach f6 (blocked)"
```

### B. Move History Display
```rust
pub struct App {
    // ...
    pub move_history: Vec<String>,  // Already added!
}

// In UI, show last 5 moves:
┌─ History ──────┐
│ 1. Blue: e2-e3 │
│ 2. Red:  d7-d6 │
│ 3. ...         │
└────────────────┘
```

### C. Compact Army Selector
Instead of full line, make it inline:
```
> [1:Blue] [2:Red] [3:Black] [4:Yellow] | e2
```

### D. Prominent Turn Indicator
```
╔═══════════════════════════════════╗
║  ▶▶▶ BLUE'S TURN ◀◀◀              ║
╚═══════════════════════════════════╝
```

### E. Undo Command
```rust
pub struct App {
    pub undo_stack: Vec<Game>,
}

// /undo or Ctrl-Z
```

## Medium Priority

### F. Show All Legal Moves Mode
Press 'L' to highlight all pieces that can move:
- Pieces with legal moves: bright highlight
- Pieces that can't move: dim
- Toggle on/off

### G. Captured Pieces Panel
```
┌─ Captured ─────┐
│ Blue: ♟♟♗      │
│ Red:  ♙♘       │
│ Black: ♙       │
│ Yellow: ♟♟     │
└────────────────┘
```

### H. Message Log
Keep last 10 messages in a scrollable log:
```
┌─ Log ──────────┐
│ Blue: e2-e3    │
│ Red: d7-d6     │
│ Invalid move   │
│ Black: ...     │
└────────────────┘
```

## Implementation Plan

### Phase 1: Error Handling & History (30 min)
- [x] Add move_history field (already done!)
- [ ] Clear selection on error
- [ ] Show move history in UI
- [ ] Better error messages

### Phase 2: Undo/Redo (45 min)
- [ ] Add undo_stack to App
- [ ] Implement /undo command
- [ ] Add Ctrl-Z keybinding
- [ ] Show undo availability in UI

### Phase 3: Visual Improvements (30 min)
- [ ] Compact army selector
- [ ] Prominent turn indicator
- [ ] Better coordinate labels
- [ ] Highlight current player's pieces

### Phase 4: Advanced Features (60 min)
- [ ] Show all legal moves mode
- [ ] Captured pieces display
- [ ] Message log
- [ ] /restart command

## Testing Strategy

### Manual Playtest Checklist
- [ ] Make a valid move - does it work?
- [ ] Make an invalid move - is error clear?
- [ ] Select wrong army's piece - good error?
- [ ] Try to move when in check - forced to move king?
- [ ] Capture a piece - is it shown?
- [ ] Play 10 moves - can you see history?
- [ ] Make a mistake - can you undo?
- [ ] Resize terminal - does UI adapt?

### Automated Tests
```rust
#[test]
fn test_undo_redo() {
    let mut app = App::new(false);
    let initial = app.game.clone();
    
    // Make a move
    app.game.apply_move(Army::Blue, 12, 20, None).unwrap();
    
    // Undo
    app.undo();
    assert_eq!(app.game.board, initial.board);
    
    // Redo
    app.redo();
    assert_ne!(app.game.board, initial.board);
}
```

## Metrics to Track

- **Time to first move**: How long does it take a new user?
- **Error rate**: How often do users make invalid moves?
- **Undo usage**: How often is undo used?
- **Session length**: How long do people play?

## User Feedback Questions

1. Was the army selection intuitive?
2. Did you understand the move highlighting?
3. Were error messages helpful?
4. Did you miss having undo?
5. Was the board easy to read?

## Conclusion

The new selection system is a huge improvement, but there are still several UX issues:

**Critical:**
- Better error handling
- Move history display
- Undo/redo

**Important:**
- Compact army selector
- Prominent turn indicator
- Captured pieces

**Nice to have:**
- Show all legal moves
- Message log
- Quick restart

Let's tackle the critical items first!
