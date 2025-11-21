# Enochian Coordinate System

The board is represented by a 64-bit integer (`u64`), where each bit corresponds to a square. The indexing follows the standard for bitboard representations in chess engines.

## Square Indexing

- Square `0` corresponds to `a1`.
- Square `63` corresponds to `h8`.

The files are mapped from `a` to `h`, and ranks from `1` to `8`.

The formula to calculate the square index from a file (`0-7`) and rank (`0-7`) is:

```
square_index = rank * 8 + file
```

## Orientation

The board is oriented with the `a1` square at the bottom-left for the **Blue** army. The turn order and army orientation for the default array are as follows, proceeding clockwise:

- **Blue**: Sits at the south edge, pawns move north. `a1` is on the left.
- **Red**: Sits at the east edge, pawns move west. `h1` is on the left.
- **Black**: Sits at the north edge, pawns move south. `h8` is on the left.
- **Yellow**: Sits at the west edge, pawns move east. `a8` is on the left.

This means "forward" for a pawn depends on its army:
- **Blue**: increases rank (`+8` to square index).
- **Red**: decreases file (`-1` to square index).
- **Black**: decreases rank (`-8` to square index).
- **Yellow**: increases file (`+1` to square index).

This mapping is based on the "Air of Water" array, which is one of the eight starting positions described by Zalewski.
