# enoch-mcp

MCP (Model Context Protocol) server for the Enochian Chess engine.

## Installation

```bash
pip install enoch-mcp
```

**Note:** Requires the `enoch` binary to be in your PATH or in `../target/release/enoch` (development mode).

## Configuration

Add to your MCP client configuration (e.g., `~/.config/claude/config.json`):

```json
{
  "mcpServers": {
    "enoch": {
      "command": "enoch-mcp"
    }
  }
}
```

## Available Tools

### Game Analysis
- **enoch_validate_move** - Check if a move is legal without applying it
- **enoch_analyze_square** - Inspect a square and see all legal moves from it
- **enoch_query_rules** - Ask questions about Enochian chess rules

### Game Management
- **enoch_generate_position** - Create custom positions from notation
- **enoch_make_move** - Execute a move in a game
- **enoch_get_status** - Get current game state (turn, frozen armies, winner)
- **enoch_get_legal_moves** - List all legal moves for an army
- **enoch_show_board** - Display the current board
- **enoch_list_arrays** - List all available starting arrays
- **enoch_undo** - Undo last N moves

### Game I/O
- **enoch_export_pgn** - Export game to PGN format
- **enoch_import_pgn** - Import game from PGN format

### Automation
- **enoch_batch** - Execute commands from batch file
- **enoch_stats** - Get game statistics (moves, captures, status)

### Utilities
- **enoch_convert_format** - Convert between JSON, ASCII, and compact formats
- **enoch_perft** - Performance testing (count positions at depth N)

## Usage Examples

### Start a new game
```python
# Generate starting position
enoch_generate_position(
    position="Ke1,Qd1,Ra1:blue Ke8:red",
    state_file="/tmp/game.json",
    show_board=True
)

# Make a move
enoch_make_move(
    move="blue: e2-e3",
    state_file="/tmp/game.json",
    show_board=True
)
```

### Analyze positions
```python
# Check if a move is legal
enoch_validate_move(move="blue: e2-e3")

# Analyze a square
enoch_analyze_square(square="e2")

# Get all legal moves
enoch_get_legal_moves(army="blue")
```

### Learn the rules
```python
# Query rules
enoch_query_rules(query="can queen capture queen")
enoch_query_rules(query="promotion")
enoch_query_rules(query="frozen armies")
```

## Development

```bash
# Install in development mode
cd enoch-mcp
pip install -e .

# Run the server
enoch-mcp
```

## License

MIT
