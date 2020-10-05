use std::fmt;
use std::iter::{Iterator};
use std::ops::Range;
use std::str;
use crate::base::Color;
use crate::game::{Board, FieldContent};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub column: i8,
    pub row: i8,
}

impl Position {
    pub fn safe_new(column: i8, row: i8) -> Option<Position> {
        if !(I8_RANGE_07.contains(&column) && I8_RANGE_07.contains(&row)) {
            return None
        }
        Some(Position {
            column,
            row,
        })
    }

    pub fn get_row_distance(&self, other: Position) -> i8 {
        (self.row - other.row).abs()
    }

    pub fn step(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                let new_column = self.column + 1;
                if new_column == 8 { None } else { Some(Position { column: new_column, row: self.row }) }
            },
            Direction::Down => {
                let new_column = self.column - 1;
                if new_column == -1 { None } else { Some(Position { column: new_column, row: self.row }) }
            },
            Direction::Right => {
                let new_row = self.row + 1;
                if new_row == 8 { None } else { Some(Position { column: self.column, row: new_row }) }
            },
            Direction::Left => {
                let new_row = self.row - 1;
                if new_row == -1 { None } else { Some(Position { column: self.column, row: new_row }) }
            },
            Direction::UpRight => Position::safe_new(self.column + 1, self.row + 1),
            Direction::DownRight => Position::safe_new(self.column - 1, self.row + 1),
            Direction::DownLeft => Position::safe_new(self.column - 1, self.row - 1),
            Direction::UpLeft => Position::safe_new(self.column + 1, self.row - 1),
        }
    }

    fn jump(
        &self,
        column_delta: i8,
        row_delta: i8,
    ) -> Option<Position> {
        Position::safe_new(self.column + column_delta, self.row + row_delta)
    }

    pub fn reachable_directed_positions<'a, 'b>(
        &'a self,
        fig_color: Color,
        direction: Direction,
        board: &'b Board,
    ) -> DirectedPosIterator<'b > {
        DirectedPosIterator::new(*self, fig_color, direction, board)
    }

    // pub fn reachable_straight_positions<'a, 'b>(
    //     &'a self,
    //     fig_color: Color,
    //     board: &'b Board,
    // ) -> impl Iterator<Item=&'b Position> {
    //     let pos_iter1= DirectedPosIterator::new(*self, fig_color, Direction::Up, board);
    //     let pos_iter2= DirectedPosIterator::new(*self, fig_color, Direction::Right, board);
    //     let pos_iter3= DirectedPosIterator::new(*self, fig_color, Direction::Down, board);
    //     let pos_iter4= DirectedPosIterator::new(*self, fig_color, Direction::Left, board);
    //
    //     pos_iter1.chain(pos_iter2).into_iter().chain(pos_iter3).into_iter().chain(pos_iter4).into_iter()
    // }
    //
    // pub fn reachable_diagonal_positions<'a, 'b>(
    //     &'a self,
    //     fig_color: Color,
    //     board: &'b Board,
    // ) -> impl Iterator<Item=&'b Position> {
    //     let pos_iter1= DirectedPosIterator::new(*self, fig_color, Direction::UpRight, board);
    //     let pos_iter2= DirectedPosIterator::new(*self, fig_color, Direction::UpLeft, board);
    //     let pos_iter3= DirectedPosIterator::new(*self, fig_color, Direction::DownLeft, board);
    //     let pos_iter4= DirectedPosIterator::new(*self, fig_color, Direction::DownRight, board);
    //
    //     pos_iter1.chain(pos_iter2).into_iter().chain(pos_iter3).into_iter().chain(pos_iter4).into_iter()
    // }

    pub fn reachable_knight_positions<'a, 'b>(
        &'a self,
        knight_color: Color,
        board: &'b Board,
    ) -> KnightPosIterator<'b> {
        KnightPosIterator::new(*self, knight_color, board)
    }
}

impl str::FromStr for Position {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut char_iter = code.chars();
        let column = ((char_iter.next().unwrap() as u8) - 97) as i8;
        let row = ((char_iter.next().unwrap() as u8) - 49) as i8;
        if char_iter.next().is_some()  {
            panic!("only 2 chars expected for Position: {}", code)
        }

        if !(I8_RANGE_07.contains(&column) && I8_RANGE_07.contains(&row)) {
            panic!("illegal value for Position: {}", code);
        }

        Ok(Position {
            column,
            row,
        })
    }
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

const I8_RANGE_07: Range<i8> = 0..8;