use std::fmt;
use std::str;
use crate::base::position::Position;
use tinyvec::TinyVec;
use crate::base::{ChessError, ErrorKind};
use tinyvec::alloc::fmt::Formatter;
use crate::figure::FigureType;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Move {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize((self.from.index<< 6) + self.to.index);
    }
}

impl Move {
    pub fn new(from: Position, to: Position) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Normal,
        }
    }

    pub fn from_code(code: &str) -> Move {
        code.parse::<Move>().unwrap_or_else(|_| panic!("illegal Move code: {}", code))
    }

    pub fn toggle_rows(&self) -> Move {
        Move {
            from: self.from.toggle_row(),
            to: self.to.toggle_row(),
            move_type: self.move_type,
        }
    }
}

impl str::FromStr for Move {
    type Err = ChessError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Move {
            from: code[0..2].parse::<Position>()?,
            move_type: code[2..3].parse::<MoveType>()?,
            to: code[3..5].parse::<Position>()?,
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.from, self.move_type, self.to)
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Default is needed, so that Move can be stored in a TinyVec
impl Default for Move {
    fn default() -> Self {
        Move {
            from: Position::new_unchecked(1, 2),
            to: Position::new_unchecked(6, 5),
            move_type: MoveType::PawnPromotion(PromotionType::Bishop)
        }
    }
}

pub const EXPECTED_MAX_NUMBER_OF_MOVES: usize = 80;

#[derive(Clone)]
pub struct MoveArray {
    array: [Move; EXPECTED_MAX_NUMBER_OF_MOVES]
}

impl tinyvec::Array for MoveArray {
    type Item = Move;
    const CAPACITY: usize = EXPECTED_MAX_NUMBER_OF_MOVES;

    fn as_slice(&self) -> &[Self::Item] {
        &self.array
    }

    fn as_slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.array
    }

    fn default() -> Self {
        MoveArray {
            array: [Move::default(); EXPECTED_MAX_NUMBER_OF_MOVES]
        }
    }
}

pub type Moves = TinyVec<MoveArray>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PromotionType {
    Rook,
    Knight,
    Bishop,
    Queen,
}

impl PromotionType {
    pub fn get_figure_type(&self) -> FigureType {
        match self {
            PromotionType::Rook => {FigureType::Rook}
            PromotionType::Knight => {FigureType::Knight}
            PromotionType::Bishop => {FigureType::Bishop}
            PromotionType::Queen => {FigureType::Queen}
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CastlingType {
    KingSide,
    QueenSide,
}

impl str::FromStr for MoveType {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<MoveType, Self::Err> {
        match s {
            "-" => Ok(MoveType::Normal),
            "Q" => Ok(MoveType::PawnPromotion(PromotionType::Queen)),
            "R" => Ok(MoveType::PawnPromotion(PromotionType::Rook)),
            "K" => Ok(MoveType::PawnPromotion(PromotionType::Knight)),
            "B" => Ok(MoveType::PawnPromotion(PromotionType::Bishop)),
            "e" => Ok(MoveType::EnPassant),
            "c" => Ok(MoveType::Castling(CastlingType::KingSide)),
            "C" => Ok(MoveType::Castling(CastlingType::QueenSide)),
            _ => Err(ChessError{
                msg: format!("unknown pawn promotion type: {}. Only QRKB are allowed.", s),
                kind: ErrorKind::IllegalFormat
            }),
        }
    }
}

impl fmt::Display for MoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let code = match self {
            MoveType::Normal => "-",
            MoveType::PawnPromotion(PromotionType::Queen) => "Q",
            MoveType::PawnPromotion(PromotionType::Rook) => "R",
            MoveType::PawnPromotion(PromotionType::Knight) => "K",
            MoveType::PawnPromotion(PromotionType::Bishop) => "B",
            MoveType::EnPassant => "e",
            MoveType::Castling(CastlingType::KingSide) => "c",
            MoveType::Castling(CastlingType::QueenSide) => "C",
        };
        write!(f, "{}", code)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MoveType {
    Normal,
    PawnPromotion(PromotionType),
    EnPassant,
    Castling(CastlingType)
}
