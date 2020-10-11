use std::fmt;
use std::iter::{Iterator};
use std::ops::Range;
use std::str;
use crate::base::{Color, ChessError, ErrorKind};
use crate::game::{Board, FieldContent, USIZE_RANGE_063};
use crate::figure::FigureType;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub column: i8,
    pub row: i8,
    pub index: usize,
}

impl Position {
    pub fn new_checked(column: i8, row: i8) -> Option<Position> {
        if !(I8_RANGE_07.contains(&column) && I8_RANGE_07.contains(&row)) {
            return None
        }
        Some(Position::new_unchecked(column, row))
    }

    pub const fn new_unchecked(column: i8, row: i8) -> Position {
        // debug_assert!(
        //     I8_RANGE_07.contains(&column) && I8_RANGE_07.contains(&row),
        //     "column and row were expected to be 0..64 but were column: {} and row: {}",
        //     column, row
        // );
        Position {
            column,
            row,
            index: ((row*8)+column) as usize,
        }
    }

    pub fn from_index_unchecked(index: usize) -> Position {
        debug_assert!(
            USIZE_RANGE_063.contains(&index),
            "index was expected to be 0..64 but was {}",
            index
        );
        let i = index as i8;
        let column = i % 8;
        let row = i/8;
        debug_assert!(
          I8_RANGE_07.contains(&column) && I8_RANGE_07.contains(&row),
          "column and row were expected to be 0..64 but were column: {} and row: {}",
          column, row
        );

        Position {
            column,
            row,
            index: ((row*8)+column) as usize,
        }
    }

    pub fn get_row_distance(&self, other: Position) -> i8 {
        (self.row - other.row).abs()
    }

    pub fn step(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                let new_column = self.column + 1;
                if new_column == 8 { None } else { Some(Position::new_unchecked(new_column, self.row)) }
            },
            Direction::Down => {
                let new_column = self.column - 1;
                if new_column == -1 { None } else { Some(Position::new_unchecked(new_column, self.row)) }
            },
            Direction::Right => {
                let new_row = self.row + 1;
                if new_row == 8 { None } else { Some(Position::new_unchecked(self.column, new_row)) }
            },
            Direction::Left => {
                let new_row = self.row - 1;
                if new_row == -1 { None } else { Some(Position::new_unchecked(self.column, new_row )) }
            },
            Direction::UpRight => Position::new_checked(self.column + 1, self.row + 1),
            Direction::DownRight => Position::new_checked(self.column - 1, self.row + 1),
            Direction::DownLeft => Position::new_checked(self.column - 1, self.row - 1),
            Direction::UpLeft => Position::new_checked(self.column + 1, self.row - 1),
        }
    }

    pub fn step_unchecked(&self, direction: Direction) -> Position {
        self.step(direction).expect("programmer assured this was fine")
    }

    fn jump(
        &self,
        column_delta: i8,
        row_delta: i8,
    ) -> Option<Position> {
        Position::new_checked(self.column + column_delta, self.row + row_delta)
    }

    pub fn reachable_directed_positions<'a, 'b>(
        &'a self,
        fig_color: Color,
        direction: Direction,
        board: &'b Board,
    ) -> DirectedPosIterator<'b > {
        DirectedPosIterator::new(*self, fig_color, direction, board)
    }

    pub fn reachable_knight_positions<'a, 'b>(
        &'a self,
        knight_color: Color,
        board: &'b Board,
    ) -> KnightPosIterator<'b> {
        KnightPosIterator::new(*self, knight_color, board)
    }
}

impl str::FromStr for Position {
    type Err = ChessError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut char_iter = code.chars();
        let column = ((char_iter.next().unwrap() as u8) - 97) as i8;
        let row = ((char_iter.next().unwrap() as u8) - 49) as i8;
        if char_iter.next().is_some()  {
            return Err(ChessError{
                msg: format!("only 2 chars expected for Position: {}", code),
                kind: ErrorKind::IllegalFormat
            });
        }

        if !(I8_RANGE_07.contains(&column) && I8_RANGE_07.contains(&row)) {
            return Err(ChessError{
                msg: format!("illegal value for Position: {}", code),
                kind: ErrorKind::IllegalFormat
            });
        }

        Ok(Position::new_unchecked(column, row))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.index==other.index
    }

    fn ne(&self, other: &Self) -> bool {
        self.index!=other.index
    }
}

impl Eq for Position {

}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", (self.column + 97) as u8 as char, (self.row+49) as u8 as char)
    }
}

pub struct DirectedPosIterator<'a> {
    latest_position: Option<Position>,
    direction: Direction,
    moving_fig_color: Color,
    board: &'a Board,
}

impl DirectedPosIterator<'_> {
    fn new(
        fig_pos: Position,
        fig_color: Color,
        direction: Direction,
        board: &Board,
    ) -> DirectedPosIterator {
        DirectedPosIterator {
            latest_position: Some(fig_pos),
            direction,
            moving_fig_color: fig_color,
            board,
        }
    }
}

impl Iterator for DirectedPosIterator<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let latest_pos = match self.latest_position {
            Some(pos) => pos,
            None => return None,
        };

        let new_pos = match latest_pos.step(self.direction) {
            Some(pos) => pos,
            None => return None,
        };
        let some_new_pos = Some(new_pos);

        match self.board.get_content_type(new_pos, self.moving_fig_color) {
            FieldContent::OwnFigure => None,
            FieldContent::OpponentFigure => {
                self.latest_position = None;
                some_new_pos
            }
            FieldContent::Empty => {
                self.latest_position = some_new_pos;
                some_new_pos
            }
        }
    }
}

pub struct KnightPosIterator<'a> {
    knight_pos: Position,
    knight_color: Color,
    board: &'a Board,
    index: usize,
}

impl KnightPosIterator<'_> {
    fn new(
        knight_position: Position,
        knight_color: Color,
        board: &Board,
    ) -> KnightPosIterator {
        KnightPosIterator {
            knight_pos: knight_position,
            knight_color,
            board,
            index: 0,
        }
    }
}

impl Iterator for KnightPosIterator<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index==8 {
                break;
            }
            let opt_pos: Option<Position> = match self.index {
                0 =>  self.knight_pos.jump(2, -1),
                1 =>  self.knight_pos.jump(2, 1),
                2 =>  self.knight_pos.jump(-2, -1),
                3 =>  self.knight_pos.jump(-2, 1),
                4 =>  self.knight_pos.jump(1, -2),
                5 =>  self.knight_pos.jump(1, 2),
                6 =>  self.knight_pos.jump(-1, -2),
                7 =>  self.knight_pos.jump(-1, 2),
                _ => panic!("index should lie between [0,7] but is {}", self.index)
            };
            self.index = self.index + 1;
            let opt_pos = opt_pos.map(|pos|{
                let field_content = self.board.get_content_type(pos, self.knight_color);
                match field_content {
                    FieldContent::OwnFigure => None,
                    _ => Some(pos)
                }
            }).flatten();
            if opt_pos.is_some() {
                return opt_pos;
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft
}

impl Direction {
    /**
    * returns a triple: (forwardLeft, forward, forwardRight)
    */
    pub fn forward_directions(color: Color) -> (Direction, Direction, Direction) {
        match color {
            Color::White => (Direction::UpLeft, Direction::Up,Direction::UpRight),
            Color::Black => (Direction::DownLeft, Direction::Down,Direction::DownRight),
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Right => Direction::Left,
            Direction::DownRight => Direction::UpLeft,
            Direction::Down => Direction::Up,
            Direction::DownLeft => Direction::UpRight,
            Direction::Left => Direction::Right,
            Direction::UpLeft => Direction::DownRight,
        }
    }
}

pub static ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Up, Direction::UpRight, Direction::Right, Direction::DownRight,
    Direction::Down, Direction::DownLeft, Direction::Left, Direction::UpLeft
];

pub static STRAIGHT_DIRECTIONS: [Direction; 4] = [
    Direction::Up, Direction::Right, Direction::Down, Direction::Left
];

pub static DIAGONAL_DIRECTIONS: [Direction; 4] = [
    Direction::UpRight, Direction::DownRight, Direction::DownLeft, Direction::UpLeft
];

pub const I8_RANGE_07: Range<i8> = 0..8;


//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(
    column, row, expected_index,
    case(0, 0, 0),
    case(7, 7, 63),
    case(1, 0, 1),
    case(0, 1, 8),
    ::trace //This attribute enable traceing
    )]
    fn test_position_unchecked_new(column: i8, row: i8, expected_index: usize) {
        let pos = Position::new_unchecked(column, row);
        assert_eq!(pos.index, expected_index);
    }

    #[rstest(
    pos_str, expected_column, expected_row, expected_index,
    case("a1", 0, 0, 0),
    case("h8", 7, 7, 63),
    case("b1", 1, 0, 1),
    case("a2", 0, 1, 8),
    ::trace //This attribute enable traceing
    )]
    fn test_position_from_str(pos_str: &str, expected_column: i8, expected_row: i8, expected_index: usize) {
        let pos = pos_str.parse::<Position>().unwrap();
        assert_eq!(pos.column, expected_column);
        assert_eq!(pos.row, expected_row);
        assert_eq!(pos.index, expected_index);
    }
}