+++
title = "Captures & Threats"
description = "Learn to identify and execute captures"
weight = 3

[extra]
scenario = """
{
  "mode": "tutorial",
  "initialPosition": {
    "d4": "BB",
    "f6": "RB",
    "c3": "YN",
    "e5": "KK"
  },
  "steps": [
    {
      "narrative": "Captures happen when a piece moves to a square occupied by an enemy. Blue Bishop can capture!",
      "highlight": ["d4", "f6"],
      "legalMoves": { "d4": ["c3", "b2", "a1", "e3", "f2", "g1", "c5", "b6", "a7", "e5", "f6", "g7", "h8"] }
    },
    {
      "narrative": "The Blue Bishop threatens Red Bishop on f6. Click d4, then f6 to capture!",
      "highlight": ["d4", "f6"],
      "expectedMove": ["d4", "f6"],
      "legalMoves": { "d4": ["f6", "e5", "c3"] }
    },
    {
      "move": ["d4", "f6"],
      "narrative": "Excellent! The Red Bishop is captured. Blue now threatens Black's King!",
      "highlight": ["f6", "e5"]
    },
    {
      "narrative": "When a King is captured, its army FREEZES. Frozen pieces block movement but can't act.",
      "highlight": ["e5"]
    },
    {
      "narrative": "This is key to Enochian Chess strategy: capture enemy kings to freeze their armies!",
      "highlight": ["f6"]
    }
  ]
}
"""
+++

## Capture Rules

Captures in Enochian Chess follow standard chess rules:
- Move your piece to an enemy-occupied square
- The enemy piece is removed from the board
- You **cannot** capture your teammate's pieces

{{ board() }}

## The Frozen Army Rule

When a **King** is captured:
1. The captured king is removed
2. All remaining pieces of that army **freeze in place**
3. Frozen pieces **block movement** but cannot move or capture
4. The game continues with the remaining armies

This creates strategic obstacles and can dramatically shift the game's dynamics.

## Team Captures

Remember the teams:
- **Team Air** (Blue + Black): Can capture Red and Yellow
- **Team Earth** (Red + Yellow): Can capture Blue and Black

You cannot capture your own teammate's pieces, even if they're in your way!

---

**Previous:** [← How Pieces Move](/learn/movement)
