+++
title = "Board & Armies"
description = "Meet the four armies and learn board orientation"
weight = 1

[extra]
scenario = """
{
  "mode": "tutorial",
  "initialPosition": {
    "b1": "BR", "c1": "BN", "d1": "BB", "e1": "BK", "f1": "BQ", "g1": "BB",
    "b8": "KR", "c8": "KN", "d8": "KB", "e8": "KK", "f8": "KQ", "g8": "KB",
    "h2": "RR", "h3": "RN", "h4": "RB", "h5": "RK", "h6": "RQ", "h7": "RB",
    "a2": "YR", "a3": "YN", "a4": "YB", "a5": "YK", "a6": "YQ", "a7": "YB"
  },
  "steps": [
    {
      "narrative": "This is the Enochian Chess starting position. Four armies occupy the edges of the board.",
      "highlight": ["e1", "e8", "h5", "a5"]
    },
    {
      "narrative": "Blue (bottom) and Black (top) form Team Air. Their kings are highlighted.",
      "highlight": ["e1", "e8"]
    },
    {
      "narrative": "Red (right) and Yellow (left) form Team Earth. Notice how armies face inward.",
      "highlight": ["h5", "a5"]
    },
    {
      "narrative": "Each army has: King, Queen, 2 Bishops, 1 Knight, 1 Rook. No pawns in this variant!",
      "highlight": ["b1", "c1", "d1", "f1", "g1"]
    },
    {
      "narrative": "Click on any piece to see its legal moves. Try clicking the Blue Bishop on d1!",
      "legalMoves": {
        "d1": ["c2", "b3", "e2", "f3", "g4"]
      }
    }
  ]
}
"""
+++

## The Four Armies

Enochian Chess features four armies arranged around the board's edges:

| Army | Position | Element | Team |
|------|----------|---------|------|
| **Blue** | Bottom (rank 1) | Water | Air |
| **Black** | Top (rank 8) | Air | Air |
| **Red** | Right (h-file) | Fire | Earth |
| **Yellow** | Left (a-file) | Earth | Earth |

{{ board() }}

## Team Structure

- **Team Air**: Blue + Black (opposing edges, files b-g)
- **Team Earth**: Red + Yellow (opposing edges, ranks 2-7)

Teams work together to capture enemy kings. When a king falls, its entire army freezes in place—still blocking movement but unable to act.

## What's Different from Chess?

1. **No pawns** — Each army has only 6 pieces
2. **Four players** — Turn order: Blue → Red → Black → Yellow
3. **Team victory** — Capture both enemy kings to win
4. **Frozen armies** — Captured kings leave obstacles behind

---

**Next:** [How Pieces Move →](../movement/)
