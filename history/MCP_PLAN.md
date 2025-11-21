# Enoch MCP Server Implementation Plan

## Goal
Expose all enoch CLI functionality via MCP (Model Context Protocol) so AI agents can easily learn and use the chess engine with minimal onboarding.

## Architecture

### Option 1: Standalone MCP Server (RECOMMENDED)
Create a separate Python package `enoch-mcp` that wraps the enoch CLI binary.

**Pros:**
- Zero changes to core enoch codebase
- Easy to install via pip (`pip install enoch-mcp`)
- Follows the beads-mcp pattern already in AGENTS.md
- Can be developed/versioned independently
- Works with any MCP-compatible client (Claude, etc.)

**Cons:**
- Requires Python runtime
- Adds one more dependency to install

### Option 2: Native Rust MCP Server
Build MCP server directly into enoch binary with `--mcp-server` flag.

**Pros:**
- Single binary, no Python needed
- Potentially faster

**Cons:**
- Requires significant Rust MCP library work
- Couples MCP concerns with chess engine
- Harder to maintain

**Decision: Go with Option 1** - follows existing patterns, easier to implement and maintain.

## MCP Tools Design

Each CLI operation becomes an MCP tool with structured inputs/outputs:

### 1. `enoch_validate_move`
**Input:**
- `move` (string): Move in format "army: from-to" (e.g., "blue: e2-e3")
- `state_file` (optional string): Path to game state JSON

**Output:**
```json
{
  "valid": true,
  "piece": "Pawn",
  "captures": null,
  "reason": null
}
```

### 2. `enoch_analyze_square`
**Input:**
- `square` (string): Square to analyze (e.g., "e2")
- `state_file` (optional string): Path to game state JSON

**Output:**
```json
{
  "square": "e2",
  "piece": {"army": "Blue", "kind": "Pawn"},
  "status": "Active",
  "legal_moves": ["e3"]
}
```

### 3. `enoch_query_rules`
**Input:**
- `query` (string): Natural language rules question

**Output:**
```json
{
  "query": "can queen capture queen",
  "answer": "❌ No - Queens cannot capture other queens"
}
```

### 4. `enoch_generate_position`
**Input:**
- `position` (string): Position notation (e.g., "Ke1:blue Ke8:red")
- `state_file` (optional string): Where to save the position
- `show_board` (optional bool): Return ASCII board

**Output:**
```json
{
  "pieces_count": 2,
  "board": "8 . . . . K . . .\n...",
  "saved_to": "/path/to/file.json"
}
```

### 5. `enoch_make_move`
**Input:**
- `move` (string): Move to make
- `state_file` (string): Game state file
- `show_board` (optional bool): Return board after move

**Output:**
```json
{
  "success": true,
  "board": "...",
  "current_turn": "Red"
}
```

### 6. `enoch_get_status`
**Input:**
- `state_file` (optional string): Game state file

**Output:**
```json
{
  "current_turn": "Blue",
  "armies": {
    "Blue": {"status": "Active", "in_check": false},
    "Red": {"status": "Active", "in_check": false},
    "Black": {"status": "Frozen", "in_check": false},
    "Yellow": {"status": "Active", "in_check": false}
  },
  "winner": null
}
```

### 7. `enoch_get_legal_moves`
**Input:**
- `army` (string): Army name (blue/red/black/yellow)
- `state_file` (optional string): Game state file

**Output:**
```json
{
  "army": "Blue",
  "moves": [
    {"from": "e2", "to": "e3"},
    {"from": "d2", "to": "d3"}
  ]
}
```

### 8. `enoch_convert_format`
**Input:**
- `format` (string): Target format (json/ascii/compact)
- `state_file` (optional string): Game state file

**Output:**
```json
{
  "format": "compact",
  "output": "blue:Ra1,Nb1,Bc1..."
}
```

### 9. `enoch_perft`
**Input:**
- `depth` (integer): Search depth
- `state_file` (optional string): Game state file

**Output:**
```json
{
  "depth": 4,
  "nodes": 118567,
  "time_seconds": 0.245,
  "nps": 483325
}
```

### 10. `enoch_show_board`
**Input:**
- `state_file` (optional string): Game state file

**Output:**
```json
{
  "board": "8 r N B Q K B N R\n7 n P P P P P P P\n..."
}
```

### 11. `enoch_ai_play`
**Input:**
- `armies` (array of strings): Which armies are AI-controlled
- `auto_play` (bool): Play until game ends
- `state_file` (string): Game state file

**Output:**
```json
{
  "moves_made": 5,
  "game_over": false,
  "winner": null
}
```

## Implementation Steps

### Phase 1: MCP Server Package Structure
```
enoch-mcp/
├── pyproject.toml
├── README.md
├── src/
│   └── enoch_mcp/
│       ├── __init__.py
│       ├── server.py      # Main MCP server
│       ├── tools.py       # Tool definitions
│       └── cli.py         # CLI wrapper utilities
└── tests/
    └── test_tools.py
```

### Phase 2: Core Implementation
1. Create `cli.py` with helper functions to invoke enoch CLI and parse output
2. Create `tools.py` with MCP tool definitions (schemas + handlers)
3. Create `server.py` with MCP server setup using `mcp` Python library
4. Add proper error handling and validation

### Phase 3: Testing
1. Unit tests for CLI parsing
2. Integration tests with actual enoch binary
3. Test with MCP inspector tool

### Phase 4: Documentation
1. Update AGENTS.md with enoch-mcp installation instructions
2. Add examples of using enoch via MCP
3. Document tool schemas

### Phase 5: Publishing
1. Publish to PyPI as `enoch-mcp`
2. Add to enoch README

## AGENTS.md Integration

Add this section to AGENTS.md:

```markdown
## Enochian Chess with enoch-mcp

This project includes an Enochian chess engine. To use it via MCP:

**Install:**
```bash
pip install enoch-mcp
```

**Add to MCP config** (e.g., `~/.config/claude/config.json`):
```json
{
  "enoch": {
    "command": "enoch-mcp",
    "args": []
  }
}
```

**Available tools:**
- `enoch_validate_move` - Check if a move is legal
- `enoch_analyze_square` - Inspect a square and see legal moves
- `enoch_query_rules` - Ask questions about game rules
- `enoch_generate_position` - Create custom positions
- `enoch_make_move` - Make a move in a game
- `enoch_get_status` - Get current game state
- `enoch_get_legal_moves` - List all legal moves for an army
- `enoch_convert_format` - Convert game state between formats
- `enoch_perft` - Performance testing
- `enoch_show_board` - Display the current board
- `enoch_ai_play` - Let AI make moves

All tools work with game state files (JSON) for persistence.
```

## Benefits

1. **Discoverability**: AI agents automatically see all available tools via MCP
2. **Type Safety**: JSON schemas ensure correct inputs
3. **Consistency**: All tools follow same patterns
4. **Extensibility**: Easy to add new tools as CLI grows
5. **Zero Friction**: One pip install + config entry = full chess engine access

## Alternative: Tool Discovery via --help

If we don't want to build MCP server yet, we could add a `--mcp-schema` flag to enoch that outputs JSON schema for all commands. AI agents could then:
1. Run `enoch --mcp-schema` to discover capabilities
2. Use regular CLI with structured parsing

This is simpler but less integrated with MCP ecosystem.

## Recommendation

**Start with standalone enoch-mcp package** following the beads-mcp pattern. This gives us:
- Clean separation of concerns
- Easy installation and configuration
- Full MCP integration
- Minimal changes to core enoch codebase

The entire MCP server can be ~200-300 lines of Python that wraps the existing CLI.
