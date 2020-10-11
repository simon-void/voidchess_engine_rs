use std::fmt;
use std::str;
use crate::base::position::Position;
use tinyvec::TinyVec;
use crate::base::{ChessError, ErrorKind};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub pawn_promo: PawnPromotion,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Move {
        Move {
            from,
            to,
            pawn_promo: PawnPromotion::No,
        }
    }
}

impl str::FromStr for Move {
    type Err = ChessError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Move {
            from: code[0..2].parse::<Position>()?,
            pawn_promo: code[2..3].parse::<PawnPromotion>()?,
            to: code[3..5].parse::<Position>()?,
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.from, self.pawn_promo, self.to)
    }
}

// Default is needed, so that Move can be stored in a TinyVec
impl Default for Move {
    fn default() -> Self {
        Move {
            from: Position::new_unchecked(1, 2),
            to: Position::new_unchecked(6, 5),
            pawn_promo: PawnPromotion::Yes(PromotionType::Bishop)
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PawnPromotion {
    Yes(PromotionType),
    No,
}

impl str::FromStr for PawnPromotion {
    type Err = ChessError;

    fn from_str(s: &str) -> Result<PawnPromotion, Self::Err> {
        match s {
            "-" => Ok(PawnPromotion::No),
            "Q" => Ok(PawnPromotion::Yes(PromotionType::Queen)),
            "R" => Ok(PawnPromotion::Yes(PromotionType::Rook)),
            "K" => Ok(PawnPromotion::Yes(PromotionType::Knight)),
            "B" => Ok(PawnPromotion::Yes(PromotionType::Bishop)),
            _ => Err(ChessError{
                msg: format!("unknown pawn promotion type: {}. Only QRKB are allowed.", s),
                kind: ErrorKind::IllegalFormat
            }),
        }
    }
}

impl fmt::Display for PawnPromotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            PawnPromotion::No => "-",
            PawnPromotion::Yes(PromotionType::Queen) => "Q",
            PawnPromotion::Yes(PromotionType::Rook) => "R",
            PawnPromotion::Yes(PromotionType::Knight) => "K",
            PawnPromotion::Yes(PromotionType::Bishop) => "B",
        };
        write!(f, "{}", code)
    }
}
