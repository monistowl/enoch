"""CLI wrapper utilities for enoch binary."""

import json
import subprocess
from pathlib import Path
from typing import Optional


def find_enoch_binary() -> str:
    """Find enoch binary in PATH or relative to this package."""
    # Try PATH first
    result = subprocess.run(["which", "enoch"], capture_output=True, text=True)
    if result.returncode == 0:
        return result.stdout.strip()
    
    # Try relative path (development mode)
    repo_root = Path(__file__).parent.parent.parent.parent
    binary = repo_root / "target" / "release" / "enoch"
    if binary.exists():
        return str(binary)
    
    raise FileNotFoundError("enoch binary not found in PATH or target/release/")


def run_enoch(args: list[str]) -> tuple[str, str, int]:
    """Run enoch CLI and return (stdout, stderr, returncode)."""
    binary = find_enoch_binary()
    result = subprocess.run(
        [binary] + args,
        capture_output=True,
        text=True
    )
    return result.stdout, result.stderr, result.returncode


def validate_move(move: str, state_file: Optional[str] = None) -> dict:
    """Validate a move without applying it."""
    args = ["--headless", "--validate", move]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    
    if code == 0:
        # Parse success output
        lines = stdout.strip().split("\n")
        result = {"valid": True, "piece": None, "captures": None}
        for line in lines:
            if "Piece:" in line:
                result["piece"] = line.split("Piece:")[1].strip()
            elif "Captures:" in line:
                result["captures"] = line.split("Captures:")[1].strip()
        return result
    else:
        # Parse error
        return {"valid": False, "reason": stdout.strip()}


def analyze_square(square: str, state_file: Optional[str] = None) -> dict:
    """Analyze a square and return piece info and legal moves."""
    args = ["--headless", "--analyze", square]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    lines = stdout.strip().split("\n")
    
    result = {"square": square, "piece": None, "status": None, "legal_moves": []}
    
    for i, line in enumerate(lines):
        if "Piece:" in line:
            result["piece"] = line.split("Piece:")[1].strip()
        elif "Status:" in line:
            result["status"] = line.split("Status:")[1].strip()
        elif "Legal moves" in line:
            # Collect moves from following lines
            for move_line in lines[i+1:]:
                move_line = move_line.strip()
                if move_line and not move_line.startswith("Analyzing"):
                    # Extract just the square (e.g., "e3" or "e3 (captures ...)")
                    move = move_line.split()[0]
                    result["legal_moves"].append(move)
        elif "Empty square" in line:
            result["piece"] = None
            result["status"] = "empty"
    
    return result


def query_rules(query: str) -> dict:
    """Query game rules."""
    args = ["--headless", "--query", query]
    stdout, stderr, code = run_enoch(args)
    return {"query": query, "answer": stdout.strip()}


def generate_position(position: str, state_file: Optional[str] = None, show_board: bool = False) -> dict:
    """Generate a custom position."""
    args = ["--headless", "--generate", position]
    if state_file:
        args.extend(["--state", state_file])
    if show_board:
        args.append("--show")
    
    stdout, stderr, code = run_enoch(args)
    
    result = {"success": code == 0}
    lines = stdout.strip().split("\n")
    
    for line in lines:
        if "Generated position with" in line:
            result["pieces_count"] = int(line.split("with")[1].split("pieces")[0].strip())
        elif "Saved to" in line:
            result["saved_to"] = line.split("Saved to")[1].strip()
    
    if show_board:
        # Extract board (everything after first line)
        board_lines = [l for l in lines if l and not l.startswith("✓")]
        result["board"] = "\n".join(board_lines)
    
    return result


def make_move(move: str, state_file: str, show_board: bool = False) -> dict:
    """Make a move in a game."""
    args = ["--headless", "--move", move, "--state", state_file]
    if show_board:
        args.append("--show")
    
    stdout, stderr, code = run_enoch(args)
    
    result = {"success": code == 0}
    if code == 0:
        if show_board:
            lines = stdout.strip().split("\n")
            board_lines = [l for l in lines if l and not l.startswith("✓") and not l.startswith("🤖")]
            result["board"] = "\n".join(board_lines)
    else:
        result["error"] = stderr.strip() or stdout.strip()
    
    return result


def get_status(state_file: Optional[str] = None) -> dict:
    """Get game status."""
    args = ["--headless", "--status"]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    lines = stdout.strip().split("\n")
    
    result = {"current_turn": None, "armies": {}, "winner": None}
    
    for line in lines:
        if "Current turn:" in line:
            result["current_turn"] = line.split("Current turn:")[1].strip()
        elif ":" in line and any(army in line for army in ["Blue", "Red", "Black", "Yellow"]):
            parts = line.split(":")
            army = parts[0].strip()
            status = parts[1].strip()
            result["armies"][army] = status
        elif "Winner:" in line:
            result["winner"] = line.split("Winner:")[1].strip()
    
    return result


def get_legal_moves(army: str, state_file: Optional[str] = None) -> dict:
    """Get legal moves for an army."""
    args = ["--headless", "--legal-moves", army]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    lines = stdout.strip().split("\n")
    
    moves = []
    for line in lines[1:]:  # Skip header
        line = line.strip()
        if "->" in line or "→" in line:
            parts = line.split("→" if "→" in line else "->")
            if len(parts) == 2:
                moves.append({"from": parts[0].strip(), "to": parts[1].strip()})
    
    return {"army": army, "moves": moves}


def convert_format(format: str, state_file: Optional[str] = None) -> dict:
    """Convert game state to different format."""
    args = ["--headless", "--convert", format]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    return {"format": format, "output": stdout.strip()}


def show_board(state_file: Optional[str] = None) -> dict:
    """Show the current board."""
    args = ["--headless", "--show"]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    return {"board": stdout.strip()}


def run_perft(depth: int, state_file: Optional[str] = None) -> dict:
    """Run performance test."""
    args = ["--headless", "--perft", str(depth)]
    if state_file:
        args.extend(["--state", state_file])
    
    stdout, stderr, code = run_enoch(args)
    lines = stdout.strip().split("\n")
    
    result = {"depth": depth}
    for line in lines:
        if "Nodes:" in line:
            result["nodes"] = int(line.split("Nodes:")[1].strip())
        elif "Time:" in line:
            result["time_seconds"] = float(line.split("Time:")[1].strip().rstrip("s"))
        elif "NPS:" in line:
            result["nps"] = int(float(line.split("NPS:")[1].strip()))
    
    return result
