use std::fmt;
use std::str;
use crate::base::position::Position;

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
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Move {
            from: Position::from_str(&code[0..2]).unwrap(),
            to: Position::from_str(&code[3..5]).unwrap(),
            pawn_promo: PawnPromotion::from_str(&code[2..3]).unwrap()
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
            from: Position{column: 1, row: 2,},
            to: Position{column: 6, row: 5,},
            pawn_promo: PawnPromotion::Yes(PromotionType::Bishop)
        }
    }
}

pub struct MoveArray {
    array: [Move; 80]
}

impl tinyvec::Array for MoveArray {
    type Item = Move;
    const CAPACITY: usize = 80;

    fn as_slice(&self) -> &[Self::Item] {
        &self.array
    }

    fn as_slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.array
    }

    fn default() -> Self {
        MoveArray {
            array: [Move::default(); 80]
        }
    }
}

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
    type Err = ();

    fn from_str(s: &str) -> Result<PawnPromotion, Self::Err> {
        match s {
            "-" => Ok(PawnPromotion::No),
            "Q" => Ok(PawnPromotion::Yes(PromotionType::Queen)),
            "R" => Ok(PawnPromotion::Yes(PromotionType::Rook)),
            "K" => Ok(PawnPromotion::Yes(PromotionType::Knight)),
            "B" => Ok(PawnPromotion::Yes(PromotionType::Bishop)),
            _ => panic!("unknown pawn promotion type"),
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
