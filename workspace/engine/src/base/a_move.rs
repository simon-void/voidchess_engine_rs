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
    state: usize,
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Move {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.state>>3);
    }
}

impl Move {
    fn combine_state(from_index: usize, to_index: usize, move_type_state: usize) -> Move {
        Move {
            state: (((move_type_state << 3) | to_index) << 6) | from_index
        }
    }

    pub fn new(from: Position, to: Position, move_type: Option<MoveType>) -> Move {
        Self::combine_state(from.index, to.index, move_type.map(|mt|mt.as_state()).unwrap_or(0))
    }

    pub fn from_code(code: &str) -> Move {
        code.parse::<Move>().unwrap_or_else(|_| panic!("illegal Move code: {}", code))
    }

    pub fn from(&self) -> Position {
        Position {index: self.state & 63}
    }

    pub fn to(&self) -> Position {
        Position {index: (self.state >> 6) & 63}
    }

    pub fn move_type(&self) -> MoveType {
        MoveType::from_state(self.state >> 12)
    }

    #[cfg(test)]
    pub fn toggle_rows(&self) -> Move {
        Move::combine_state(
            self.from().toggle_row().index,
            self.to().toggle_row().index,
            self.move_type().as_state(),
        )
    }
}

impl str::FromStr for Move {
    type Err = ChessError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Move::new (
            code[0..2].parse::<Position>()?,
            code[3..5].parse::<Position>()?,
            Some(code[2..3].parse::<MoveType>()?),
        ))
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.from(), self.move_type(), self.to())
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
        Move::new(
            Position::new_unchecked(1, 2),
            Position::new_unchecked(6, 5),
            Some(MoveType::PawnPromotion(PromotionType::Bishop)),
        )
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

impl MoveType {
    pub fn as_state(&self) -> usize {
        match self {
            MoveType::Normal => { 0 }
            MoveType::PawnPromotion(promostion_type) => {
                match promostion_type {
                    PromotionType::Rook => { 1 }
                    PromotionType::Knight => { 2 }
                    PromotionType::Bishop => { 3 }
                    PromotionType::Queen => { 4 }
                }
            }
            MoveType::EnPassant => { 5 }
            MoveType::Castling(castling_type) => {
                match castling_type {
                    CastlingType::KingSide => { 6 }
                    CastlingType::QueenSide => { 7 }
                }
            }
        }
    }

    pub fn from_state(state: usize) -> MoveType {
        match state {
            0 => { MoveType::Normal }
            1 => { MoveType::PawnPromotion(PromotionType::Rook) }
            2 => { MoveType::PawnPromotion(PromotionType::Knight) }
            3 => { MoveType::PawnPromotion(PromotionType::Bishop) }
            4 => { MoveType::PawnPromotion(PromotionType::Queen) }
            5 => { MoveType::EnPassant }
            6 => { MoveType::Castling(CastlingType::KingSide) }
            7 => { MoveType::Castling(CastlingType::QueenSide) }
            _ => { unreachable!() }
        }
    }
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::base::CastlingType::QueenSide;
    use crate::base::Direction;

    #[rstest(
        from_str, to_str, expected_direction,
        case("e4", "e6", Some(Direction::Up)),
        case("e4", "g6", Some(Direction::UpRight)),
        case("e4", "g4", Some(Direction::Right)),
        case("e4", "g2", Some(Direction::DownRight)),
        case("e4", "e2", Some(Direction::Down)),
        case("e4", "c2", Some(Direction::DownLeft)),
        case("e4", "c4", Some(Direction::Left)),
        case("e4", "c6", Some(Direction::UpLeft)),
        case("e4", "e4", None),
        case("e4", "a5", None),
        ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_direction(from_str: &str, to_str: &str, expected_direction: Option<Direction>) {
        let from = from_str.parse::<Position>().unwrap();
        let to = to_str.parse::<Position>().unwrap();

        let actual_opt_direction = from.get_direction(to);
        assert_eq!(actual_opt_direction, expected_direction);
    }

    #[test]
    fn test_move_combine_state() {
        let a_move = Move::combine_state(
            Position::from_code("a2").index,
            Position::from_code("c4").index,
            Some(MoveType::PawnPromotion(PromotionType::Bishop)).unwrap().as_state(),
        );
        assert_eq!(a_move.from(), Position::from_code("a2"), "from");
        assert_eq!(a_move.to(), Position::from_code("c4"), "to");
        assert_eq!(a_move.move_type(), MoveType::PawnPromotion(PromotionType::Bishop), "move_type");
    }

    #[test]
    fn test_move_new_and_getter() {
        let a_move = Move::new(
            Position::from_code("a2"),
            Position::from_code("c4"),
            Some(MoveType::PawnPromotion(PromotionType::Knight)),
        );
        assert_eq!(a_move.from(), Position::from_code("a2"), "from");
        assert_eq!(a_move.to(), Position::from_code("c4"), "to");
        assert_eq!(a_move.move_type(), MoveType::PawnPromotion(PromotionType::Knight), "move_type");

        let b_move = Move::new(
            Position::from_code("h8"),
            Position::from_code("h1"),
            None,
        );
        assert_eq!(b_move.from(), Position::from_code("h8"), "from");
        assert_eq!(b_move.to(), Position::from_code("h1"), "to");
        assert_eq!(b_move.move_type(), MoveType::Normal, "move_type");
    }


    #[test]
    fn test_move_toggle_row() {

        let a_move = Move::new(
            Position::from_code("a2"),
            Position::from_code("d5"),
            Some(MoveType::Castling(QueenSide)),
        );
        let toggled_move = a_move.toggle_rows();

        assert_eq!(toggled_move.from(), Position::from_code("a7"), "from");
        assert_eq!(toggled_move.to(), Position::from_code("d4"), "to");
        assert_eq!(toggled_move.move_type(), MoveType::Castling(QueenSide), "move_type");

        assert_eq!(toggled_move.toggle_rows(), a_move, "double toggle");
    }
}
