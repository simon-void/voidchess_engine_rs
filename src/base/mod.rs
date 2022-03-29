mod a_move;
pub(crate) mod direction;
mod errors;
pub(crate) mod rc_list;
mod position;

pub use a_move::*;
pub use errors::*;
pub use position::*;
pub use direction::*;
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

    pub fn get_fen_char(&self) -> char {
        match self {
            Color::Black => {'b'}
            Color::White => {'w'}
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Deactivatable {
    value: bool,
}

impl Deactivatable {
    pub fn new(value: bool) ->Deactivatable {
        Deactivatable {
            value,
        }
    }

    pub fn deactivate(&mut self) {
        self.value = false;
    }

    pub fn get_value(&self) -> bool {
        self.value
    }
}

impl fmt::Display for Deactivatable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}