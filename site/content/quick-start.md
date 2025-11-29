+++
title = "Quick Start"
description = "Learn the essentials of Enochian Chess in just 5 minutes"
template = "page.html"
+++

Welcome to Enochian Chess. This guide will give you everything you need to start playing.

## The Board & The Armies

Enochian Chess uses a standard 8×8 board, but with **four armies** instead of two:

<div class="team-indicator">
    <div class="team team--air">
        <span class="army-badge army-badge--blue">Blue</span>
        <span>+</span>
        <span class="army-badge army-badge--black">Black</span>
        <span>= Team Air</span>
    </div>
    <div class="team team--earth">
        <span class="army-badge army-badge--red">Red</span>
        <span>+</span>
        <span class="army-badge army-badge--yellow">Yellow</span>
        <span>= Team Earth</span>
    </div>
</div>

Each army occupies one edge of the board:
- **Blue** starts on the South edge (rank 1)
- **Red** starts on the East edge (file h)
- **Black** starts on the North edge (rank 8)
- **Yellow** starts on the West edge (file a)

## Turn Order

Play proceeds **clockwise**: Blue → Red → Black → Yellow → Blue...

This means you'll move, then your *opponent* moves, then your *ally* moves, then the other opponent. The rhythm takes getting used to!

## How Pieces Move

Most pieces move as you'd expect from standard chess, with some notable exceptions:

| Piece | Movement |
|-------|----------|
| **King** | One square in any direction (no castling) |
| **Queen** | Leaps exactly 2 squares in any direction (not a slider!) |
| **Bishop** | Slides diagonally (restricted to its diagonal network) |
| **Rook** | Slides horizontally and vertically |
| **Knight** | L-shaped leap (2+1 squares) |
| **Pawn** | One square forward only (no double-step, no en passant) |

<div class="callout callout--important">
    <div class="callout__title">The Queen is Different!</div>
    <div class="callout__content">
        Unlike standard chess, the Queen doesn't slide across the board. She <em>leaps</em>
        exactly two squares—jumping over any piece in her way. This makes her both more
        limited and surprisingly tricky.
    </div>
</div>

## How to Win

**Capture both enemy kings.** That's it.

There's no checkmate in Enochian Chess. Kings can be captured directly, just like any other piece.

When a king is captured, something dramatic happens: that **entire army freezes**. The pieces remain on the board but cannot move, capture, or give check. They simply block squares like statues.

<div class="callout callout--tip">
    <div class="callout__title">Teamwork Matters</div>
    <div class="callout__content">
        If your ally's king is captured, their army freezes—but <em>you</em> can still fight.
        Move your king onto your ally's <strong>throne square</strong> to seize control of their
        frozen army and bring them back to life.
    </div>
</div>

## One Special Rule: Check Still Matters

When your king is in check and has legal king moves available, you **must** move your king. You cannot block with another piece or capture the attacker with a non-king piece if the king itself can move.

Only when the king has no legal moves can other pieces intervene.

---

## You're Ready!

That's enough to play your first game. As you play, you'll encounter more nuances:

- [Diagonal networks](/rules/pieces/bishop/) restrict which bishops can capture which queens
- [Thrones](/rules/#thrones) let you resurrect frozen allies
- [Privileged pawns](/rules/#privileged-pawn) can promote to anything when your army is depleted

But for now, just play. The elegance of Enochian Chess reveals itself at the board.

<div style="margin-top: 2rem;">
    <a href="/rules/" class="btn btn--primary">Read the Full Rules</a>
</div>

---

## Playing in the Terminal (TUI)

If you are playing using the `enoch` terminal application, here are the essential commands:

- **Move**: Type `army: e2-e4` (e.g., `blue: e2-e4` or `red: h2-h1`).
- **New Game**: `/new <array>` (e.g., `/new Tablet of Fire (Air Setting)`).
- **Save/Load**: `/save mygame.json` and `/load mygame.json`.
- **AI**: Type `/ai` to let the computer make a move for the current army.
- **Divination Mode**: `/mode divination` switches to dice-based play (d6 rolls determine valid pieces).
- **Help**: Type `/help` or `/arrays` to see more options.

