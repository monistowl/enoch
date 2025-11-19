# Enochian Chess Rules

This document outlines the rules of Enochian Chess, as described in the project's `ROADMAP.md`.

## Board & Players

*   **Board:** 8x8
*   **Armies:** 4 armies: Blue, Black, Red, and Yellow.
*   **Teams:**
    *   Team 1: Blue + Black
    *   Team 2: Red + Yellow
*   **Turn Order:** The turn order is clockwise, but can be changed by the selected starting array. A common example is Blue -> Red -> Black -> Yellow.

## Pieces

### King
*   **Movement:** 1 step in any direction.
*   **Notes:** Kings are captured, not checkmated.

### Queen
*   **Movement:** Leaps exactly 2 squares along ranks, files, or diagonals, ignoring intervening pieces.
*   **Notes:** Queens cannot capture other Queens.

### Rook
*   **Movement:** Slides any number of squares along ranks or files.

### Bishop
*   **Movement:** Slides any number of squares diagonally.
*   **Notes:** Bishops and Queens run on distinct but interlocking networks of diagonals. Bishops cannot capture other Bishops, but can capture Queens.

### Knight
*   **Movement:** L-shaped leap (2+1).

### Pawn
*   **Movement:** 1 step forward.
*   **Capture:** 1 step diagonally forward.
*   **Notes:** No double-step or en passant.

## Special Rules

### King Capture & Check Logic
*   There is no checkmate; kings are captured.
*   If a king is in check and has a legal move, it **must** move, even if it stays in check.
*   If the king has no legal moves (e.g. blocked by friendly pieces), another piece may be moved.

### Frozen Pieces
*   When a king is captured, all pieces of that army become **frozen**.
*   Frozen pieces cannot move, threaten, or be captured. They act as blocking terrain.

### Seizing the Throne
*   If a king moves onto an ally's throne square, you gain control of that army.
*   If the seized army was frozen, its pieces are revived.
*   Control persists even if the king later leaves the throne.
*   If the controlling king is captured, control reverts to the original player (if their king is still on the board).

### Exchange of Prisoners
*   If two enemy players have each captured a king, they may agree to an exchange.
*   The kings are returned to their thrones (or the nearest allowed square) and their armies are unfrozen.

### Privileged Pawn & Promotion
*   A pawn is promoted on its normal promotion rank to the piece of its "type" (e.g. a pawn of a queen becomes a queen).
*   A pawn becomes **privileged** if its army is reduced to:
    *   (king + queen + pawn)
    *   (king + bishop + pawn)
    *   (king + pawn)
*   A privileged pawn may be promoted to any major piece.
*   If it promotes to a piece type that is already in play, the existing piece is demoted to a pawn of that type.

### Stalemate & Draws
*   **Stalemate:** If a player's unchecked king has no legal moves except to move into check, that player skips their turns until the stalemate is lifted.
*   **Draw Conditions:**
    *   Both allied kings are bare.
    *   Only the four bare kings remain.

### Divination Mode (Optional)
*   A dice roll determines which piece type must be moved:
    *   1: King or Pawn
    *   2: Knight
    *   3: Bishop
    *   4: Queen
    *   5: Rook
    *   6: Pawn
