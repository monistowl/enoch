+++
title = "The Pieces"
description = "Movement and capture rules for each piece in Enochian Chess"
template = "section.html"
+++

Enochian Chess uses the same six piece types as standard chess, but several behave quite differently. The most dramatic change is the **Queen**, which leaps rather than slides.

<div class="card-grid">
    <a href="king/" class="piece-card piece-card--king">
        <div class="piece-card__glyph">♔</div>
        <div class="piece-card__content">
            <div class="piece-card__name">King</div>
            <div class="piece-card__summary">One square any direction. Can be captured.</div>
        </div>
    </a>
    <a href="queen/" class="piece-card piece-card--queen">
        <div class="piece-card__glyph">♕</div>
        <div class="piece-card__content">
            <div class="piece-card__name">Queen</div>
            <div class="piece-card__summary">Leaps exactly 2 squares. Cannot capture queens.</div>
        </div>
    </a>
    <a href="bishop/" class="piece-card piece-card--bishop">
        <div class="piece-card__glyph">♗</div>
        <div class="piece-card__content">
            <div class="piece-card__name">Bishop</div>
            <div class="piece-card__summary">Diagonal slider. Restricted to one network.</div>
        </div>
    </a>
    <a href="rook/" class="piece-card piece-card--rook">
        <div class="piece-card__glyph">♖</div>
        <div class="piece-card__content">
            <div class="piece-card__name">Rook</div>
            <div class="piece-card__summary">Orthogonal slider. Standard chess movement.</div>
        </div>
    </a>
    <a href="knight/" class="piece-card piece-card--knight">
        <div class="piece-card__glyph">♘</div>
        <div class="piece-card__content">
            <div class="piece-card__name">Knight</div>
            <div class="piece-card__summary">L-shaped leap. Standard chess movement.</div>
        </div>
    </a>
    <a href="pawn/" class="piece-card piece-card--pawn">
        <div class="piece-card__glyph">♙</div>
        <div class="piece-card__content">
            <div class="piece-card__name">Pawn</div>
            <div class="piece-card__summary">One square forward only. No double-step.</div>
        </div>
    </a>
</div>

## Capture Restrictions

The most unusual aspect of Enochian Chess pieces is the **capture restrictions** between Queens and Bishops:

| Attacker | Can Capture |
|----------|-------------|
| Queen | Kings, Rooks, Knights, Pawns, Bishops (same network only) |
| Bishop | Kings, Rooks, Knights, Pawns, Queens (same network only) |

Queens **never** capture other Queens. Bishops **never** capture other Bishops.

This creates a fascinating dynamic where certain pieces are effectively invisible to each other, and network alignment becomes a strategic consideration.
