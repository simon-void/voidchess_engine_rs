mod position;
mod a_move;

pub use a_move::*;
pub use position::*;
use std::fmt;
use std::ops::Range;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black, White,
}

impl Color {
    pub fn toggle(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}

pub const USIZE_RANGE_07: Range<usize> = 0..8;
