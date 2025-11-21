use serde::{Deserialize, Serialize};

pub const ARMY_COUNT: usize = 4;
pub const TEAM_COUNT: usize = 2;
pub const PIECE_KIND_COUNT: usize = 6;

pub type Square = u8;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Army {
    Blue,
    Black,
    Red,
    Yellow,
}

impl Army {
    pub fn from_str(name: &str) -> Option<Army> {
        match name.to_lowercase().as_str() {
            "blue" => Some(Army::Blue),
            "black" => Some(Army::Black),
            "red" => Some(Army::Red),
            "yellow" => Some(Army::Yellow),
            _ => None,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Army::Blue => "Blue",
            Army::Black => "Black",
            Army::Red => "Red",
            Army::Yellow => "Yellow",
        }
    }
}

impl Army {
    pub const ALL: [Army; ARMY_COUNT] = [Army::Blue, Army::Black, Army::Red, Army::Yellow];

    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    pub fn team(self) -> Team {
        match self {
            Army::Blue | Army::Black => Team::Air,
            Army::Red | Army::Yellow => Team::Earth,
        }
    }

    pub fn pawn_direction(self) -> i8 {
        match self {
            Army::Blue => 1,   // moves up
            Army::Red => -1,   // moves down
            Army::Black => -1, // moves left (but in rank terms)
            Army::Yellow => 1, // moves right (but in rank terms)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Team {
    Air,   // Blue + Black
    Earth, // Red + Yellow
}

impl Team {
    pub const ALL: [Team; TEAM_COUNT] = [Team::Air, Team::Earth];

    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    pub fn name(self) -> &'static str {
        match self {
            Team::Air => "Air",
            Team::Earth => "Earth",
        }
    }

    pub const fn armies(self) -> [Army; 2] {
        match self {
            Team::Air => [Army::Blue, Army::Black],
            Team::Earth => [Army::Red, Army::Yellow],
        }
    }

    pub const fn opponent(self) -> Team {
        match self {
            Team::Air => Team::Earth,
            Team::Earth => Team::Air,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum PieceKind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceKind {
    pub const ALL: [PieceKind; PIECE_KIND_COUNT] = [
        PieceKind::King,
        PieceKind::Queen,
        PieceKind::Bishop,
        PieceKind::Knight,
        PieceKind::Rook,
        PieceKind::Pawn,
    ];

    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn name(self) -> &'static str {
        match self {
            PieceKind::King => "King",
            PieceKind::Queen => "Queen",
            PieceKind::Bishop => "Bishop",
            PieceKind::Knight => "Knight",
            PieceKind::Rook => "Rook",
            PieceKind::Pawn => "Pawn",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u8);

impl PlayerId {
    pub const PLAYER_ONE: PlayerId = PlayerId(0);
    pub const PLAYER_TWO: PlayerId = PlayerId(1);

    pub const fn new(id: u8) -> PlayerId {
        PlayerId(id)
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        PlayerId::PLAYER_ONE
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub army: Army,
    pub kind: PieceKind,
    pub pawn_type: Option<PieceKind>, // for “pawn of X” if you want to distinguish
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub kind: PieceKind,
    pub promotion: Option<PieceKind>,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{:?}{}{}",
            file_char(self.from),
            rank_char(self.from),
            file_char(self.to),
            rank_char(self.to)
        )
    }
}

pub fn file_char(square: Square) -> char {
    ((square % 8) + b'a') as char
}

pub fn rank_char(square: Square) -> char {
    ((square / 8) + b'1') as char
}
