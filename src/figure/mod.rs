mod functions;

use crate::base::Color;
use std::fmt;
use crate::{Move, Position, MatchState};

#[derive(Debug, Copy, Clone)]
pub struct Figure {
    pub fig_type: FigureType,
    pub color: Color,
}

impl Figure {
    pub fn for_reachable_moves<F>(&self, pos: Position, match_state: &MatchState, inform_of: F) where F: Fn(Move) {
        functions::for_reachable_moves(self.fig_type, pos, match_state, inform_of)
    }
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.fig_type {
            FigureType::Pawn => write!(f, "{}-Pawn", self.color),
            FigureType::Rook(_) => write!(f, "{}-Rook", self.color),
            FigureType::Knight => write!(f, "{}-Knight", self.color),
            FigureType::Bishop => write!(f, "{}-Bishop", self.color),
            FigureType::Queen => write!(f, "{}-Queen", self.color),
            FigureType::King => write!(f, "{}-King", self.color),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FigureType {
    Pawn,
    Rook(RookType),
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub enum RookType {
    QueenSide,
    KingSide,
    Promoted,
}
