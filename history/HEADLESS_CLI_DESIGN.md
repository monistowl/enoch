# Headless CLI Mode Design

## Current State

The game currently requires the TUI (Terminal User Interface) to play. All commands go through the interactive interface.

## Goal

Enable playing Enochian Chess via pure CLI commands without the TUI, suitable for:
- Scripting and automation
- AI vs AI matches
- Remote play over SSH
- Integration with other tools
- Batch game processing

## Required Features

### 1. Non-Interactive Mode

```bash
# Start a new game
enoch --headless --array "Tablet of Fire"

# Make moves via stdin or arguments
enoch --headless --move "blue: e2-e4"
enoch --headless --move "red: d7-d6"

# Or pipe commands
echo "blue: e2-e4" | enoch --headless
```

### 2. Game State Management

```bash
# Save/load game state
enoch --headless --load game.json --move "blue: e2-e4" --save game.json

# Or use state file
enoch --headless --state game.json --move "blue: e2-e4"
```

### 3. Output Formats

```bash
# ASCII board output
enoch --headless --show

# JSON output for parsing
enoch --headless --show --format json

# Minimal output (just success/error)
enoch --headless --quiet --move "blue: e2-e4"
```

### 4. AI Integration

```bash
# Enable AI for specific armies
enoch --headless --ai red,black --state game.json

# AI vs AI match
enoch --headless --ai red,black,yellow --state game.json --auto-play

# Play until game ends
enoch --headless --ai red,black,yellow --auto-play --output match.json
```

### 5. Query Commands

```bash
# Show legal moves
enoch --headless --state game.json --legal-moves blue

# Show game status
enoch --headless --state game.json --status

# Check if move is valid
enoch --headless --state game.json --validate "blue: e2-e4"
```

## Implementation Plan

### Phase 1: Basic Headless Mode (30 min)

Add `--headless` flag that:
- Skips TUI initialization
- Reads commands from stdin or args
- Outputs to stdout
- Exits after command

```rust
// main.rs
fn main() {
    let args = Args::parse();
    
    if args.headless {
        run_headless(args);
    } else {
        run_tui();
    }
}

fn run_headless(args: Args) {
    let mut game = load_or_create_game(&args);
    
    if let Some(move_cmd) = args.move_cmd {
        execute_move(&mut game, &move_cmd);
    }
    
    if args.show {
        print_board(&game, args.format);
    }
    
    if let Some(save_path) = args.save {
        game.to_json_file(&save_path);
    }
}
```

### Phase 2: State Management (15 min)

```rust
fn load_or_create_game(args: &Args) -> Game {
    if let Some(state_file) = &args.state {
        Game::from_json_file(state_file).unwrap()
    } else if let Some(array_name) = &args.array {
        let spec = find_array_by_name(array_name).unwrap();
        Game::from_array_spec(spec)
    } else {
        Game::from_array_spec(default_array())
    }
}
```

### Phase 3: Output Formats (20 min)

```rust
fn print_board(game: &Game, format: OutputFormat) {
    match format {
        OutputFormat::Ascii => {
            for row in game.board.ascii_rows() {
                println!("{}", row);
            }
        }
        OutputFormat::Json => {
            println!("{}", game.to_json().unwrap());
        }
        OutputFormat::Compact => {
            println!("Turn: {} | Status: {}", 
                game.current_army().display_name(),
                game.status_summary());
        }
    }
}
```

### Phase 4: AI Integration (15 min)

```rust
fn run_with_ai(game: &mut Game, ai_armies: Vec<Army>, auto_play: bool) {
    loop {
        let current = game.current_army();
        
        if ai_armies.contains(&current) {
            if let Some(mv) = ai::capture_preferring_move(game, current) {
                game.apply_move(current, mv.from, mv.to, None).unwrap();
                println!("{}: {} -> {}", 
                    current.display_name(),
                    square_name(mv.from),
                    square_name(mv.to));
            }
        } else if !auto_play {
            break; // Wait for human input
        }
        
        if game.winning_team().is_some() || !auto_play {
            break;
        }
    }
}
```

### Phase 5: Query Commands (10 min)

```rust
fn handle_query(game: &Game, query: Query) {
    match query {
        Query::LegalMoves(army) => {
            let moves = game.generate_legal_moves(army);
            for mv in moves {
                println!("{}{} -> {}{}", 
                    file_char(mv.from), rank_char(mv.from),
                    file_char(mv.to), rank_char(mv.to));
            }
        }
        Query::Status => {
            println!("Current: {}", game.current_army().display_name());
            println!("Winner: {:?}", game.winning_team());
        }
        Query::Validate(move_str) => {
            // Parse and validate move
            println!("{}", is_valid);
        }
    }
}
```

## CLI Arguments Structure

```rust
#[derive(Parser)]
struct Args {
    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,
    
    /// Game state file
    #[arg(long)]
    state: Option<String>,
    
    /// Starting array name
    #[arg(long)]
    array: Option<String>,
    
    /// Make a move
    #[arg(long)]
    move_cmd: Option<String>,
    
    /// Show board
    #[arg(long)]
    show: bool,
    
    /// Output format (ascii, json, compact)
    #[arg(long, default_value = "ascii")]
    format: OutputFormat,
    
    /// Save game state
    #[arg(long)]
    save: Option<String>,
    
    /// Enable AI for armies (comma-separated)
    #[arg(long)]
    ai: Option<String>,
    
    /// Auto-play until game ends
    #[arg(long)]
    auto_play: bool,
    
    /// Show legal moves for army
    #[arg(long)]
    legal_moves: Option<String>,
    
    /// Show game status
    #[arg(long)]
    status: bool,
    
    /// Validate a move without executing
    #[arg(long)]
    validate: Option<String>,
    
    /// Quiet mode (minimal output)
    #[arg(long)]
    quiet: bool,
}
```

## Example Usage

### Interactive Play

```bash
# Start new game
enoch --headless --state game.json --show

# Make moves
enoch --headless --state game.json --move "blue: e2-e4" --show
enoch --headless --state game.json --move "red: d7-d6" --show
```

### AI vs Human

```bash
# Enable AI for Red
enoch --headless --state game.json --ai red --show

# Make Blue's move, AI responds automatically
enoch --headless --state game.json --move "blue: e2-e4" --show
```

### AI vs AI

```bash
# Watch AI battle
enoch --headless --ai blue,red,black,yellow --auto-play --save final.json
```

### Scripted Play

```bash
#!/bin/bash
# Play a scripted game

enoch --headless --state game.json --show

for move in "blue: e2-e4" "red: d7-d6" "black: g8-f6" "yellow: b1-c3"; do
    echo "Making move: $move"
    enoch --headless --state game.json --move "$move" --show
    sleep 1
done
```

### Query Information

```bash
# Check legal moves
enoch --headless --state game.json --legal-moves blue

# Validate move
enoch --headless --state game.json --validate "blue: e2-e4"

# Get status
enoch --headless --state game.json --status --format json
```

## Benefits

1. **Automation**: Script entire games or tournaments
2. **Testing**: Automated game testing and validation
3. **Remote Play**: Play over SSH without terminal requirements
4. **Integration**: Integrate with other tools and systems
5. **AI Development**: Test AI strategies in batch
6. **Analysis**: Process many games for analysis

## Implementation Effort

- Phase 1 (Basic): 30 minutes
- Phase 2 (State): 15 minutes
- Phase 3 (Output): 20 minutes
- Phase 4 (AI): 15 minutes
- Phase 5 (Query): 10 minutes

**Total: ~90 minutes**

## Testing

```bash
# Test basic move
enoch --headless --move "blue: e2-e4" --show

# Test AI
enoch --headless --ai red,black,yellow --auto-play

# Test state persistence
enoch --headless --state test.json --move "blue: e2-e4"
enoch --headless --state test.json --show

# Test queries
enoch --headless --legal-moves blue
enoch --headless --status
```

## Future Enhancements

- Network play (client/server mode)
- Tournament mode (round-robin, Swiss)
- PGN-style notation export
- Game replay from notation
- Performance benchmarking mode
- Parallel game processing

## Conclusion

Headless CLI mode would make Enochian Chess:
- Scriptable and automatable
- Suitable for AI development
- Accessible for remote play
- Integrable with other tools
- Testable at scale

Implementation is straightforward since all game logic is already separate from UI.
