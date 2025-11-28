+++
title = "Full Rules"
description = "The complete rulebook for Enochian Chess"
template = "section.html"
+++

This is the comprehensive reference for Enochian Chess. If you're new, start with the [Quick Start](/quick-start/) guide first.

## Board Geometry

Enochian Chess uses a standard **8×8 board** with algebraic notation (`a1` in the lower-left corner). What makes it unique is the orientation: four armies enter from the four cardinal directions.

The board is oriented by compass:
- **South** (ranks 1-2): Blue army's home
- **East** (files g-h): Red army's home
- **North** (ranks 7-8): Black army's home
- **West** (files a-b): Yellow army's home

Play proceeds **clockwise**: Blue → Red → Black → Yellow → Blue...

## The Four Armies

| Army | Element | Team | Pawn Direction | Promotion Zone |
|------|---------|------|----------------|----------------|
| <span class="army-badge army-badge--blue">Blue</span> | Air | Air | North (+8) | Rank 8 |
| <span class="army-badge army-badge--red">Red</span> | Fire | Earth | West (−1) | File a |
| <span class="army-badge army-badge--black">Black</span> | Water | Air | South (−8) | Rank 1 |
| <span class="army-badge army-badge--yellow">Yellow</span> | Earth | Earth | East (+1) | File h |

**Teams:** Blue + Black form **Team Air**. Red + Yellow form **Team Earth**.

In a four-player game, each person controls one army. In a two-player game, each person controls an entire team (two armies).

## Thrones {#thrones}

Each army has two **throne squares**—special squares where kings gain additional powers.

| Army | Throne Squares |
|------|----------------|
| Blue | d1, e1 |
| Red | e8, d8 |
| Black | a5, a4 |
| Yellow | h4, h5 |

**Throne properties:**
- A king on its own throne can **share the square** with one allied piece
- If an enemy captures a double-occupied throne, **both pieces are removed**
- Moving your king to an **ally's throne** lets you seize control of their army

## Diagonal Networks {#diagonal-networks}

One of the most unusual features of Enochian Chess is the division of diagonals into two **networks**: Aries and Cancer.

Every diagonal square belongs to one network:
- **Aries network**: Squares matching the pattern `0x55AA55AA55AA55AA`
- **Cancer network**: Squares matching the pattern `0xAA55AA55AA55AA55`

Queens and bishops are permanently assigned to one network. This affects captures:
- Bishops can only capture queens on the **same** diagonal network
- Queens can only capture bishops on the **same** diagonal network
- Bishops **never** capture other bishops
- Queens **never** capture other queens

The starting array defines which network each piece belongs to.

## King Capture & Frozen Armies {#frozen-armies}

<div class="callout callout--important">
    <div class="callout__title">No Checkmate</div>
    <div class="callout__content">
        There is no checkmate in Enochian Chess. Kings are captured directly, just like any other piece.
    </div>
</div>

When a king is captured:
1. The entire army becomes **frozen**
2. Frozen pieces remain on the board but cannot move, capture, or give check
3. Frozen pieces still block squares—they become obstacles

### Seizing Control

You can rescue a frozen allied army:
1. Move your king onto your ally's **throne square**
2. You immediately gain control of their frozen army
3. The army thaws and can move again on subsequent turns
4. Control persists even if your king leaves the throne

### Exchange of Prisoners

If two opposing players have each captured a king, they may negotiate an **exchange**:
- Both captured kings return to their thrones (or nearest legal square)
- Both frozen armies thaw immediately
- This is a negotiated action, not automatic

## Check & Forced King Moves {#check}

Check works differently than in standard chess:

1. **Check detection**: A king is in check if any unfrozen opposing piece attacks its square

2. **Forced king moves**: If your king is in check **and** has at least one legal king move, you **must** move the king. You cannot block with another piece or capture the attacker with a non-king.

3. **No legal king moves**: Only when the king has no legal moves can other pieces act while the king remains in check.

## Stalemate

If a non-checked king has no legal moves (every move would put it in check), that army is **stalemated**.

A stalemated army skips its turns until the stalemate is broken—perhaps by an ally moving pieces, or an enemy capture that frees up squares.

## Privileged Pawn {#privileged-pawn}

Normally, pawns promote to their assigned **patron piece** (the piece type they represent, like "pawn of queen").

However, if an army is reduced to a minimal force:
- King + Queen + Pawn
- King + Bishop + Pawn
- King + Pawn only

...then that pawn becomes **privileged**. A privileged pawn may promote to **any** major piece type.

**Twist:** If you promote to a piece type already on the board, the existing piece is **demoted** back into a pawn.

## Victory & Draw Conditions

### Victory
A team wins when **both enemy kings** have been captured (and not returned via prisoner exchange).

### Draws
The game is drawn if:
- Both allied kings are bare (no other pieces)
- Only four bare kings remain on the board
- Players mutually agree after an unresolved stalemate cycle

## Divination Mode (Optional) {#divination}

For those who embrace the mystical origins, **Divination Mode** adds a d6 roll that constrains each move:

| Die | Piece(s) You Must Move |
|-----|----------------------|
| 1 | King or Pawn |
| 2 | Knight |
| 3 | Bishop |
| 4 | Queen |
| 5 | Rook |
| 6 | Pawn |

If no piece of the rolled type can legally move, you may re-roll (up to a configured limit).

---

<div style="margin-top: 2rem;">
    <a href="/rules/pieces/" class="btn btn--secondary">Learn About Each Piece →</a>
</div>
