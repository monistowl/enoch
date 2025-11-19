
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Army {
    Blue,
    Black,
    Red,
    Yellow,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Team {
    Air,   // Blue + Black
    Earth, // Red + Yellow
}

impl Army {
    pub fn team(self) -> Team {
        match self {
            Army::Blue | Army::Black => Team::Air,
            Army::Red | Army::Yellow => Team::Earth,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Piece {
    pub army: Army,
    pub kind: PieceKind,
    pub pawn_type: Option<PieceKind>, // for “pawn of X” if you want to distinguish
}
