mod a_move;
mod errors;
mod position;

pub use a_move::*;
pub use errors::*;
pub use position::*;
use std::fmt;

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
