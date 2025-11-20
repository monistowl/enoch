# Enochian Chess Rules Specification

This document summarizes the rules of Enochian Chess as derived from the Golden Dawn / Zalewski texts, serving as a single source of truth for the game's implementation.

## 0.1 – Rules Spec

### Board & Players

*   8x8 board; 4 armies: Blue, Black, Red, Yellow.
*   Teams: (Blue + Black) vs (Red + Yellow).
*   Turn order: clockwise around the board (e.g. Blue → Red → Black → Yellow for one common array).

### Setup / Arrays

*   Eight possible starting arrays (piece placements and army orientations) per Zalewski; at least one fully encoded from the article’s diagrams, then leave TODOs for the remaining ones.
*   Mark throne squares and double-occupancy rules (initially king + another piece; if captured while two pieces remain, both are removed).

### Piece Types

*   **King:** 1 step in any direction, like FIDE king.
*   **Queen:** leaps exactly **two squares** along ranks, files, or diagonals (Alibaba-style), ignoring intervening pieces.
*   **Rook:** as FIDE rook (any number of squares orthogonally).
*   **Bishop:** as FIDE bishop (sliding diagonals), but with special bishop/queen capture interactions (see below).
*   **Knight:** FIDE knight (2+1 leap).
*   **Pawns:** one square forward only; capture diagonally; no double-step, no en passant.

### Bishop/Queen Interaction

*   Bishops and queens run on distinct but interlocking networks of diagonals (“Aries” vs “Cancer” systems).
*   Queens do **not** capture enemy queens, bishops do **not** capture enemy bishops; queens can capture enemy bishops and bishops can capture enemy queens, per “Concourse of Bishoping” and summary text.

### King Capture & Check Logic

*   There is no mate; kings are **captured**, not checkmated.
*   If your king is in check and has any legal king move, you **must** move the king, even if it stays in check. Only if the king has no legal move (blocked by friendly pieces, board edge etc.) may you move another piece.

### Frozen Pieces & King Capture

*   When a king is captured, all pieces of that color become **frozen**: they cannot move, threaten, or be captured, but they occupy squares as blocking terrain.

### Seizing the Throne / Control

*   If your king moves onto an ally’s throne square, you gain control of that army; frozen pieces revive. Control persists even if your king later leaves; if that king is captured, control reverts (if ally still has a king).

### Exchange of Prisoners

*   If two enemy players each captured a king, they may agree to exchange kings back to their thrones (or nearest allowed squares) and unfreeze their armies.

### Privilege Pawn & Promotion

*   **Standard promotion:** a pawn promoted on its normal promotion rank becomes the piece of its “type” (pawn of queen → queen, etc.).
*   **Privileged pawn rule:** if a side is reduced to (king + queen + pawn) or (king + bishop + pawn) or (king + pawn), pawn becomes **privileged**, may promote to any major piece; if promoting to a type already in play, the existing piece is demoted back to a pawn-of-that-type.

### Stalemate & Draws

*   **Stalemate:** if a player’s **unchecked** king has no moves except to move into check, player skips turns until stalemate is lifted.
*   **Draw conditions:** e.g. both allied kings bare, or four bare kings only.

### Divination Mode (Dice)

*   Die roll selects which type of piece must be moved (1: king or pawn, 2: knight, 3: bishop, 4: queen, 5: rook, 6: pawn). This is marked as “Phase 5 optional” in the roadmap.