# UI Improvements & Testing Strategy Brainstorm

## Current State Analysis

### What Works Well
- ✅ Board scales dynamically (1x1 to 3x3 squares)
- ✅ Responsive layout (side-by-side vs stacked)
- ✅ Clear visual indicators (❄ ⚠ ✓ ⊗ 🏆 ⚖)
- ✅ Color-coded armies with good contrast
- ✅ Help system with scrolling
- ✅ Screenshot/capture for debugging

### Current Pain Points

#### 1. **Move Input is Cryptic**
```
> blue: e2-e4
```
- Users must know army names
- Must type full army name
- No visual feedback until Enter
- No autocomplete or suggestions
- Easy to make typos

#### 2. **No Visual Move Hints**
- Can't see legal moves for a piece
- No indication of which pieces can move
- No highlighting of selected piece
- Hard to learn piece movement rules

#### 3. **Command Discovery**
- Commands hidden in help text
- No inline suggestions
- `/` prefix not obvious
- Many commands rarely used

#### 4. **Status Information Overload**
- Too much text in status panel
- Important info (check, frozen) buried
- History takes up space
- Command help line too long

#### 5. **No Undo/Redo**
- Mistakes are permanent
- Can't explore "what if" scenarios
- No way to review game history visually

## Proposed Improvements

### Phase 1: Better Move Input

#### A. Army Selection Mode
Instead of typing army name, use a selection system:
```
Current: > blue: e2-e4
Proposed: [Blue] [Red] [Black] [Yellow]  > e2-e4
          ^^^^^
          (highlighted, press 1-4 or arrow keys)
```

**Implementation:**
- Add `selected_army: Option<Army>` to App
- Show army selector bar above input
- Keys: `1`=Blue, `2`=Red, `3`=Black, `4`=Yellow
- Or arrow keys + Enter to select
- Auto-select current turn's army

**Benefits:**
- No typing army names
- Visual feedback
- Faster input
- Fewer errors

#### B. Piece Selection Mode
Click-style interface for terminal:
```
1. Press 'e2' to select piece
2. Shows legal moves highlighted on board
3. Press 'e4' to move there
```

**Implementation:**
- Add `selected_square: Option<u8>` to App
- Parse square coordinates (a1-h8)
- Highlight selected piece
- Show legal moves with different background
- Second square input completes move

**Benefits:**
- Visual feedback
- Learn legal moves
- More intuitive
- Less typing

### Phase 2: Visual Enhancements

#### A. Move Highlighting
```rust
// In board rendering
if let Some(sq) = app.selected_square {
    // Highlight selected piece with bright border
    // Show legal moves with green tint
    let legal_moves = app.game.legal_moves_from(sq);
    for mv in legal_moves {
        // Tint destination squares
    }
}
```

#### B. Last Move Indicator
Show the last move made:
```
▶ Blue to move    Last: Red r: d7-d5
```

#### C. Captured Pieces Display
Show captured pieces beside board:
```
┌─ Captured ─┐
│ Blue: ♟♟♗  │
│ Red:  ♙♘   │
└────────────┘
```

### Phase 3: Command Improvements

#### A. Command Palette
Press `:` to open command palette with fuzzy search:
```
┌─ Commands ─────────────────┐
│ > arr                      │
│   /arrays - List arrays    │
│   /array <name> - Load     │
│   /screenshot <file>       │
└────────────────────────────┘
```

#### B. Contextual Help
Show relevant commands based on game state:
```
In check: "Press ? for help | /status to see details"
Frozen:   "Army frozen - use /exchange to revive"
```

#### C. Keyboard Shortcuts
- `s` - Save game (prompts for filename)
- `l` - Load game
- `u` - Undo last move
- `r` - Redo move
- `h` - Show move history
- `?` - Help (already implemented)

### Phase 4: Game State Visualization

#### A. Move History Panel
```
┌─ History ──────┐
│ 1. Blue: e2-e4 │
│ 2. Red:  d7-d5 │
│ 3. Black: Nf6  │
│ 4. Yellow: c5  │
│ > 5. Blue: ?   │
└────────────────┘
```

#### B. Threat Indicators
Show pieces under attack:
```
K (under attack by 2 pieces)
Q (defended by 1 piece)
```

#### C. Board Orientation
Allow rotating board for each player:
```
Press 'o' to rotate board 90°
Useful for 4-player games
```

### Phase 5: Advanced Features

#### A. Analysis Mode
- Show all legal moves for current army
- Highlight pieces that can move
- Show attack/defense coverage

#### B. Divination Mode Enhancements
- Show dice roll animation
- Display piece selection probabilities
- Visual feedback for restricted moves

#### C. Tutorial Mode
- Step-by-step guide for new players
- Highlight valid moves
- Explain special rules as they occur

## Testing Strategy: Resize Integration

### Current Testing Gap
- No tests for UI rendering
- No tests for different terminal sizes
- No tests for responsive layout
- Manual testing only

### Proposed Testing Approach

#### 1. **Snapshot Testing for UI**

Create a test harness that renders UI at different sizes:

```rust
#[cfg(test)]
mod ui_tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_render_at_minimum_size() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        
        terminal.draw(|f| render(f, &mut app)).unwrap();
        
        let buffer = terminal.backend().buffer();
        // Assert board is visible
        // Assert no overflow
        // Assert key elements present
    }

    #[test]
    fn test_render_at_large_size() {
        let backend = TestBackend::new(200, 60);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        
        terminal.draw(|f| render(f, &mut app)).unwrap();
        
        // Assert board uses 3x3 squares
        // Assert side panel visible
        // Assert all info displayed
    }

    #[test]
    fn test_responsive_breakpoints() {
        let sizes = vec![
            (80, 24),   // Minimum
            (100, 30),  // Medium
            (132, 46),  // Large
            (200, 60),  // Extra large
        ];
        
        for (width, height) in sizes {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).unwrap();
            let mut app = App::new();
            
            terminal.draw(|f| render(f, &mut app)).unwrap();
            
            // Verify no panics
            // Verify board fits
            // Verify layout is correct
        }
    }
}
```

#### 2. **Layout Calculation Tests**

Test the math for board sizing:

```rust
#[test]
fn test_square_size_calculation() {
    // Test that square size is calculated correctly
    let test_cases = vec![
        ((80, 24), 1),   // Small -> 1x1
        ((100, 30), 2),  // Medium -> 2x2
        ((132, 46), 3),  // Large -> 3x3
    ];
    
    for ((width, height), expected_size) in test_cases {
        let available_height = height.saturating_sub(6);
        let available_width = width.saturating_sub(4);
        
        let max_square_height = available_height / 9;
        let max_square_width = available_width / 10;
        let square_size = max_square_height.min(max_square_width / 2).max(1).min(3);
        
        assert_eq!(square_size, expected_size);
    }
}

#[test]
fn test_board_dimensions() {
    // Test that board dimensions are calculated correctly
    for square_size in 1..=3 {
        let board_width = 2 + (square_size * 2 + 1) * 8 + 2;
        let board_height = 1 + square_size * 8 + 1 + 2;
        
        // Verify dimensions are reasonable
        assert!(board_width > 0);
        assert!(board_height > 0);
        
        // Verify board fits in minimum terminal
        if square_size == 1 {
            assert!(board_width <= 80);
            assert!(board_height <= 24);
        }
    }
}
```

#### 3. **Screenshot-Based Regression Tests**

Use the screenshot feature for visual regression:

```rust
#[test]
fn test_ui_screenshots() {
    let sizes = vec![(80, 24), (100, 30), (132, 46)];
    
    for (width, height) in sizes {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        
        terminal.draw(|f| render(f, &mut app)).unwrap();
        
        // Save screenshot
        let filename = format!("tests/screenshots/ui_{}x{}.txt", width, height);
        if let Some(ref frame) = app.last_frame {
            std::fs::write(&filename, frame).unwrap();
        }
        
        // Compare with golden file
        let golden = format!("tests/golden/ui_{}x{}.txt", width, height);
        if std::path::Path::new(&golden).exists() {
            let expected = std::fs::read_to_string(&golden).unwrap();
            let actual = app.last_frame.as_ref().unwrap();
            assert_eq!(actual, &expected, "UI changed at {}x{}", width, height);
        }
    }
}
```

#### 4. **Property-Based Testing**

Test that UI works at ANY size:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_ui_at_random_sizes(width in 80u16..200, height in 24u16..60) {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new();
        
        // Should not panic
        let result = terminal.draw(|f| render(f, &mut app));
        assert!(result.is_ok());
        
        // Board should be visible
        assert!(app.last_frame.is_some());
    }
}
```

### Integration with CI/CD

Add to `.github/workflows/test.yml`:

```yaml
- name: UI Tests
  run: |
    cargo test --test ui_tests
    
- name: Generate UI Screenshots
  run: |
    cargo test --test ui_screenshots -- --ignored
    
- name: Upload Screenshots
  uses: actions/upload-artifact@v3
  with:
    name: ui-screenshots
    path: tests/screenshots/
```

## Priority Ranking

### High Priority (Do First)
1. **Army selection mode** - Biggest UX improvement
2. **Move highlighting** - Makes game learnable
3. **Layout calculation tests** - Prevent regressions
4. **Undo/redo** - Essential for gameplay

### Medium Priority
5. **Command palette** - Better discoverability
6. **Last move indicator** - Helpful context
7. **Screenshot regression tests** - Catch visual bugs
8. **Captured pieces display** - Nice to have

### Low Priority (Future)
9. **Board rotation** - Niche use case
10. **Analysis mode** - Advanced feature
11. **Tutorial mode** - Requires content creation
12. **Property-based UI tests** - Overkill for now

## Implementation Plan

### Sprint 1: Input Improvements
- [ ] Add army selection bar
- [ ] Implement piece selection mode
- [ ] Add move highlighting
- [ ] Test at multiple terminal sizes

### Sprint 2: Testing Infrastructure
- [ ] Create UI test harness
- [ ] Add layout calculation tests
- [ ] Implement screenshot regression tests
- [ ] Document testing approach

### Sprint 3: Visual Polish
- [ ] Add last move indicator
- [ ] Show captured pieces
- [ ] Improve status panel layout
- [ ] Add keyboard shortcuts

### Sprint 4: Advanced Features
- [ ] Implement undo/redo
- [ ] Add command palette
- [ ] Create move history panel
- [ ] Add contextual help

## Technical Considerations

### Performance
- UI rendering is fast (< 16ms)
- No performance issues observed
- Screenshot capture is negligible overhead

### Accessibility
- Color-blind friendly palette needed
- Screen reader support (future)
- Keyboard-only navigation (already works)

### Compatibility
- Works on macOS, Linux, Windows
- Tested on iTerm2, Terminal.app, Alacritty
- Minimum terminal: 80x24

### Maintainability
- Keep UI code separate from game logic
- Use ratatui best practices
- Document layout calculations
- Add tests for all new features

## Conclusion

The current UI is functional but has room for significant UX improvements. The biggest wins are:

1. **Better move input** - Army selection + piece selection modes
2. **Visual feedback** - Highlighting, last move, captured pieces
3. **Testing** - Automated UI tests prevent regressions

These changes will make the game more intuitive for new players while maintaining the terminal aesthetic.
