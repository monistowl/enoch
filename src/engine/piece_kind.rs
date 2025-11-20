use crate::engine::types::PieceKind;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidLength,
    InvalidSource,
    InvalidTarget,
    InvalidCastling,
}

#[derive(Debug, PartialEq)]
pub enum SpecialMove {
    Promotion(PieceKind),
}

#[derive(Debug, PartialEq)]
pub struct ParsedMove {
    pub piece: PieceKind,
    /// from file and rank is optional (e.g. Nf3)
    pub from_file: Option<char>,
    pub from_rank: Option<u64>,
    pub to: u64,
    pub is_capture: bool,
    pub special_move: Option<SpecialMove>,
}

/// parses PGN moves, there is no validation of the move. All validations are
/// done on game.rs (this includes promotion logic)
/// It is only responsible to make sure the string is a correct PGN format
pub fn parse_move(cmd: &str) -> Result<ParsedMove, ParseError> {
    if cmd.len() <= 1 {
        // invalid
        return Err(ParseError::InvalidLength);
    }

    let mut chars = cmd.chars();
    let source = chars.next().unwrap();
    let piece = parse_source(source)?;

    match piece {
        PieceKind::Pawn => parse_pawn(source, chars),

        PieceKind::Knight
        | PieceKind::Rook
        | PieceKind::Bishop
        | PieceKind::Queen
        | PieceKind::King => parse_piece(piece, chars),
    }
}

fn parse_piece(piece: PieceKind, mut chars: Chars) -> Result<ParsedMove, ParseError> {
    let mut is_capture = false;
    let mut to: u64 = 0;

    #[derive(Debug, PartialEq)]
    enum PieceParserState {
        Initial,
        PotentialTargetFileParsed,
        PotentialTargetRankParsed,
        PotentialTargetParsed,
        SourceParsed,
        TargetFileParsed,
        TargetParsed,
    }

    let mut state = PieceParserState::Initial;

    let mut potential_target_rank: u64 = 0;
    let mut potential_target_file: char = ' ';

    let mut source_file: Option<char> = None;
    let mut source_rank: Option<u64> = None;

    while let Some(c) = chars.next() {
        match state {
            PieceParserState::Initial => match c {
                file @ 'a'..='h' => {
                    potential_target_file = file;
                    state = PieceParserState::PotentialTargetFileParsed;
                }
                rank @ '0'..='8' => {
                    potential_target_rank = rank.to_digit(10).unwrap() as u64;
                    state = PieceParserState::PotentialTargetRankParsed;
                }
                'x' => {
                    state = PieceParserState::SourceParsed;
                    is_capture = true;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },

            PieceParserState::PotentialTargetFileParsed => match c {
                rank @ '0'..='8' => {
                    potential_target_rank = rank.to_digit(10).unwrap() as u64;
                    state = PieceParserState::PotentialTargetParsed;
                }
                'x' if piece != PieceKind::King => {
                    source_file = Some(potential_target_file);
                    potential_target_file = ' ';
                    state = PieceParserState::SourceParsed;
                    is_capture = true;
                }
                // handling ambiguous (exclude king)
                file @ 'a'..='h' if piece != PieceKind::King => {
                    source_file = Some(potential_target_file);
                    potential_target_file = file;
                    state = PieceParserState::TargetFileParsed;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PieceParserState::PotentialTargetRankParsed => match c {
                'x' => {
                    source_rank = Some(potential_target_rank);
                    potential_target_rank = 0;
                    state = PieceParserState::SourceParsed;
                    is_capture = true;
                }
                // handling ambiguous (exclude king)
                file @ 'a'..='h' if piece != PieceKind::King => {
                    source_rank = Some(potential_target_rank);
                    potential_target_file = file;
                    to = 0; // Replace with a valid bitboard value
                    state = PieceParserState::TargetFileParsed;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PieceParserState::PotentialTargetParsed => match c {
                'x' if piece != PieceKind::King => {
                    source_file = Some(potential_target_file);
                    source_rank = Some(potential_target_rank);
                    potential_target_file = ' ';
                    potential_target_rank = 0;
                    state = PieceParserState::SourceParsed;
                    is_capture = true;
                }
                file @ 'a'..='h' if piece != PieceKind::King => {
                    source_file = Some(potential_target_file);
                    source_rank = Some(potential_target_rank);
                    potential_target_file = file;
                    to = 0; // Replace with a valid bitboard value
                    state = PieceParserState::TargetFileParsed;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },

            PieceParserState::SourceParsed => match c {
                file @ 'a'..='h' => {
                    potential_target_file = file;
                    state = PieceParserState::PotentialTargetFileParsed;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PieceParserState::TargetFileParsed => match c {
                rank @ '0'..='8' => {
                    potential_target_rank = rank.to_digit(10).unwrap() as u64;
                    to = 0; // Replace with a valid bitboard value
                    state = PieceParserState::TargetParsed;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PieceParserState::TargetParsed => {
                return match c {
                    _ => Err(ParseError::InvalidTarget),
                }
            }
        }
    }

    // final checks
    if state == PieceParserState::PotentialTargetParsed {
        to = 0; // Replace with a valid bitboard value
        state = PieceParserState::TargetParsed;
    }

    if state != PieceParserState::TargetParsed || to == 0 {
        return Err(ParseError::InvalidTarget);
    }

    Ok(ParsedMove {
        piece,
        from_file: source_file,
        from_rank: source_rank,
        to,
        is_capture,
        special_move: None,
    })
}

fn parse_pawn(source: char, mut chars: Chars) -> Result<ParsedMove, ParseError> {
    let mut is_capture = false;
    let mut to: u64 = 0;
    let mut special_move: Option<SpecialMove> = None;

    #[derive(Debug, PartialEq)]
    enum PawnParserState {
        Initial,
        TargetParsed,
        Capturing,
        PromotionPiece,
    }

    let mut state = PawnParserState::Initial;
    let mut _target_rank: u64 = 0;

    while let Some(c) = chars.next() {
        match state {
            PawnParserState::Initial => match c {
                rank @ '1'..='8' => {
                    _target_rank = rank.to_digit(10).unwrap() as u64;
                    to = 0; // Replace with a valid bitboard value
                    state = PawnParserState::TargetParsed;
                }
                'x' => {
                    state = PawnParserState::Capturing;
                    is_capture = true;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PawnParserState::Capturing => match c {
                _file @ 'a'..='h' => {
                    if let Some(c) = chars.next() {
                        match c {
                            rank @ '1'..='8' => {
                                _target_rank = rank.to_digit(10).unwrap() as u64;
                                to = 0; // Replace with a valid bitboard value
                                state = PawnParserState::TargetParsed;
                            }
                            _ => {
                                return Err(ParseError::InvalidTarget);
                            }
                        }
                    } else {
                        return Err(ParseError::InvalidTarget);
                    }
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PawnParserState::TargetParsed => match c {
                '=' => {
                    state = PawnParserState::PromotionPiece;
                }
                _ => {
                    return Err(ParseError::InvalidTarget);
                }
            },
            PawnParserState::PromotionPiece => {
                let promotion = match c {
                    'N' => PieceKind::Knight,
                    'R' => PieceKind::Rook,
                    'B' => PieceKind::Bishop,
                    'Q' => PieceKind::Queen,
                    _ => {
                        return Err(ParseError::InvalidTarget);
                    }
                };
                special_move = Some(SpecialMove::Promotion(promotion));
            }
        }
    }

    // final checks
    if to == 0 {
        return Err(ParseError::InvalidTarget);
    }
    if state == PawnParserState::PromotionPiece && special_move == None {
        return Err(ParseError::InvalidTarget);
    }

    Ok(ParsedMove {
        piece: PieceKind::Pawn,
        from_file: Some(source),
        from_rank: None,
        to,
        is_capture,
        special_move,
    })
}

fn parse_source(c: char) -> Result<PieceKind, ParseError> {
    match c {
        'a'..='h' => Ok(PieceKind::Pawn),
        'N' => Ok(PieceKind::Knight),
        'R' => Ok(PieceKind::Rook),
        'B' => Ok(PieceKind::Bishop),
        'Q' => Ok(PieceKind::Queen),
        'K' => Ok(PieceKind::King),
        _ => Err(ParseError::InvalidSource),
    }
}
