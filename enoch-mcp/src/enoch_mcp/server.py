"""MCP server for Enochian Chess engine."""

import asyncio
from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import Tool, TextContent

from . import cli


server = Server("enoch-mcp")


@server.list_tools()
async def list_tools() -> list[Tool]:
    """List available tools."""
    return [
        Tool(
            name="enoch_validate_move",
            description="Validate a chess move without applying it",
            inputSchema={
                "type": "object",
                "properties": {
                    "move": {"type": "string", "description": "Move in format 'army: from-to' (e.g., 'blue: e2-e3')"},
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
                "required": ["move"],
            },
        ),
        Tool(
            name="enoch_analyze_square",
            description="Analyze a square to see piece info and legal moves",
            inputSchema={
                "type": "object",
                "properties": {
                    "square": {"type": "string", "description": "Square to analyze (e.g., 'e2')"},
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
                "required": ["square"],
            },
        ),
        Tool(
            name="enoch_query_rules",
            description="Query Enochian chess rules with natural language",
            inputSchema={
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "Rules question (e.g., 'can queen capture queen')"},
                },
                "required": ["query"],
            },
        ),
        Tool(
            name="enoch_generate_position",
            description="Generate a custom chess position",
            inputSchema={
                "type": "object",
                "properties": {
                    "position": {"type": "string", "description": "Position notation (e.g., 'Ke1:blue Ke8:red')"},
                    "state_file": {"type": "string", "description": "Optional path to save position"},
                    "show_board": {"type": "boolean", "description": "Return ASCII board representation"},
                },
                "required": ["position"],
            },
        ),
        Tool(
            name="enoch_make_move",
            description="Make a move in a game",
            inputSchema={
                "type": "object",
                "properties": {
                    "move": {"type": "string", "description": "Move to make (e.g., 'blue: e2-e3')"},
                    "state_file": {"type": "string", "description": "Path to game state JSON file"},
                    "show_board": {"type": "boolean", "description": "Return board after move"},
                },
                "required": ["move", "state_file"],
            },
        ),
        Tool(
            name="enoch_get_status",
            description="Get current game status",
            inputSchema={
                "type": "object",
                "properties": {
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
            },
        ),
        Tool(
            name="enoch_get_legal_moves",
            description="Get all legal moves for an army",
            inputSchema={
                "type": "object",
                "properties": {
                    "army": {"type": "string", "description": "Army name (blue/red/black/yellow)"},
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
                "required": ["army"],
            },
        ),
        Tool(
            name="enoch_convert_format",
            description="Convert game state between formats",
            inputSchema={
                "type": "object",
                "properties": {
                    "format": {"type": "string", "description": "Target format (json/ascii/compact)"},
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
                "required": ["format"],
            },
        ),
        Tool(
            name="enoch_show_board",
            description="Display the current board",
            inputSchema={
                "type": "object",
                "properties": {
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
            },
        ),
        Tool(
            name="enoch_perft",
            description="Run performance test (count positions at depth N)",
            inputSchema={
                "type": "object",
                "properties": {
                    "depth": {"type": "integer", "description": "Search depth (1-6 recommended)"},
                    "state_file": {"type": "string", "description": "Optional path to game state JSON file"},
                },
                "required": ["depth"],
            },
        ),
    ]


@server.call_tool()
async def call_tool(name: str, arguments: dict) -> list[TextContent]:
    """Handle tool calls."""
    import json
    
    try:
        if name == "enoch_validate_move":
            result = cli.validate_move(arguments["move"], arguments.get("state_file"))
        elif name == "enoch_analyze_square":
            result = cli.analyze_square(arguments["square"], arguments.get("state_file"))
        elif name == "enoch_query_rules":
            result = cli.query_rules(arguments["query"])
        elif name == "enoch_generate_position":
            result = cli.generate_position(
                arguments["position"],
                arguments.get("state_file"),
                arguments.get("show_board", False)
            )
        elif name == "enoch_make_move":
            result = cli.make_move(
                arguments["move"],
                arguments["state_file"],
                arguments.get("show_board", False)
            )
        elif name == "enoch_get_status":
            result = cli.get_status(arguments.get("state_file"))
        elif name == "enoch_get_legal_moves":
            result = cli.get_legal_moves(arguments["army"], arguments.get("state_file"))
        elif name == "enoch_convert_format":
            result = cli.convert_format(arguments["format"], arguments.get("state_file"))
        elif name == "enoch_show_board":
            result = cli.show_board(arguments.get("state_file"))
        elif name == "enoch_perft":
            result = cli.run_perft(arguments["depth"], arguments.get("state_file"))
        else:
            raise ValueError(f"Unknown tool: {name}")
        
        return [TextContent(type="text", text=json.dumps(result, indent=2))]
    
    except Exception as e:
        return [TextContent(type="text", text=json.dumps({"error": str(e)}, indent=2))]


async def run_server():
    """Run the MCP server."""
    async with stdio_server() as (read_stream, write_stream):
        await server.run(read_stream, write_stream, server.create_initialization_options())


def main():
    """Entry point for the MCP server."""
    asyncio.run(run_server())


if __name__ == "__main__":
    main()
