+++
title = "How Pieces Move"
description = "Learn how each piece moves in Enochian Chess"
weight = 2

[extra]
scenario = """
{
  "mode": "tutorial",
  "initialPosition": {
    "d4": "BB",
    "e4": "BN",
    "f4": "BR",
    "d5": "BK",
    "e5": "BQ"
  },
  "steps": [
    {
      "narrative": "Let's learn how each piece moves. We'll start with the Bishop on d4.",
      "highlight": ["d4"],
      "legalMoves": { "d4": ["c3", "b2", "a1", "e3", "f2", "g1", "c5", "b6", "a7", "e5", "f6", "g7", "h8"] }
    },
    {
      "narrative": "The Bishop moves diagonally any number of squares. Click the Bishop to see its moves!",
      "highlight": ["d4"],
      "legalMoves": { "d4": ["c3", "b2", "a1", "e3", "f2", "g1", "c5", "b6", "a7", "e5", "f6", "g7", "h8"] }
    },
    {
      "narrative": "The Knight on e4 moves in an L-shape: 2 squares in one direction, then 1 square perpendicular.",
      "highlight": ["e4"],
      "legalMoves": { "e4": ["d2", "f2", "c3", "g3", "c5", "g5", "d6", "f6"] }
    },
    {
      "narrative": "Knights can jump over other pieces—the only piece that can do this!",
      "highlight": ["e4"],
      "legalMoves": { "e4": ["d2", "f2", "c3", "g3", "c5", "g5", "d6", "f6"] }
    },
    {
      "narrative": "The Rook on f4 moves horizontally or vertically any number of squares.",
      "highlight": ["f4"],
      "legalMoves": { "f4": ["f1", "f2", "f3", "f5", "f6", "f7", "f8", "a4", "b4", "c4", "g4", "h4"] }
    },
    {
      "narrative": "The King on d5 moves one square in any direction—but cannot move into check.",
      "highlight": ["d5"],
      "legalMoves": { "d5": ["c4", "c5", "c6", "d6", "e6"] }
    },
    {
      "narrative": "The Queen combines Bishop and Rook movement—diagonals AND straight lines!",
      "highlight": ["e5"],
      "legalMoves": { "e5": ["e1", "e2", "e3", "e6", "e7", "e8", "a5", "b5", "c5", "f5", "g5", "h5", "d6", "c7", "b8", "f6", "g7", "h8", "f4", "g3", "h2"] }
    },
    {
      "narrative": "Practice: Click each piece to explore its movement. The Queen is the most powerful!",
      "legalMoves": {
        "d4": ["c3", "b2", "a1", "e3", "f2", "g1", "c5", "b6", "a7"],
        "e4": ["d2", "f2", "c3", "g3", "c5", "g5", "d6", "f6"],
        "f4": ["f1", "f2", "f3", "f5", "f6", "f7", "f8", "a4", "b4", "c4", "g4", "h4"],
        "d5": ["c4", "c5", "c6", "d6", "e6"],
        "e5": ["e1", "e2", "e3", "e6", "e7", "e8", "a5", "b5", "c5", "f5", "g5", "h5", "d6", "c7", "b8", "f6", "g7", "h8"]
      }
    }
  ]
}
"""
+++

## Piece Movement Summary

| Piece | Movement | Special Notes |
|-------|----------|---------------|
| **King** | 1 square any direction | Cannot move into check |
| **Queen** | Any distance, any direction | Most powerful piece |
| **Bishop** | Any distance diagonally | Stays on one color |
| **Knight** | L-shape (2+1) | Can jump over pieces |
| **Rook** | Any distance orthogonally | Horizontal or vertical |

{{ board() }}

## Key Differences from Standard Chess

In Enochian Chess:
- **No pawns** — Only the 6 major pieces per army
- **Same movement rules** — Pieces move identically to standard chess
- **Four directions** — Armies attack from all four sides

---

**Previous:** [← Board & Armies](../board-setup/) | **Next:** [Captures & Threats →](../captures/)
