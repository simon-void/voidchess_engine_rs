mod functions;

use std::fmt;
use crate::base::*;
use std::str;
use crate::game::GameState;

#[derive(Debug, Copy, Clone)]
pub struct Figure {
    pub fig_type: FigureType,
    pub color: Color,
}

impl Figure {
    pub fn for_reachable_moves(&self, pos: Position, match_state: &GameState, move_collector: &mut Moves) {
        functions::for_reachable_moves(self.fig_type, pos, match_state, move_collector)
    }
}

impl str::FromStr for Figure {
    type Err = ChessError;

    fn from_str(desc: &str) -> Result<Self, Self::Err> {
        match desc {
            "♙" => Ok(Figure{fig_type: FigureType::Pawn, color: Color::White}),
            "♟" => Ok(Figure{fig_type: FigureType::Pawn, color: Color::Black}),
            "♖" => Ok(Figure{fig_type: FigureType::Rook(RookType::Promoted), color: Color::White}),
            "♜" => Ok(Figure{fig_type: FigureType::Rook(RookType::Promoted), color: Color::Black}),
            "♘" => Ok(Figure { fig_type: FigureType::Knight, color: Color::White }),
            "♞" => Ok(Figure { fig_type: FigureType::Knight, color: Color::Black }),
            "♗" => Ok(Figure { fig_type: FigureType::Bishop, color: Color::White }),
            "♝" => Ok(Figure { fig_type: FigureType::Bishop, color: Color::Black }),
            "♕" => Ok(Figure { fig_type: FigureType::Queen, color: Color::White }),
            "♛" => Ok(Figure { fig_type: FigureType::Queen, color: Color::Black }),
            "♔" => Ok(Figure { fig_type: FigureType::King, color: Color::White }),
            "♚" => Ok(Figure { fig_type: FigureType::King, color: Color::Black }),
            _ => Err(ChessError{
                msg: format!("unexpected character, utf-chess symbol like ♙ expected but got {}", desc),
                kind: ErrorKind::IllegalFormat,
            })
        }
    }
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.fig_type {
            FigureType::Pawn => {if self.color==Color::White {"♙"} else {"♟"}}
            FigureType::Rook(_) => {if self.color==Color::White {"♖"} else {"♜"}}
            FigureType::Knight => {if self.color==Color::White {"♘"} else {"♞"}}
            FigureType::Bishop => {if self.color==Color::White {"♗"} else {"♝"}}
            FigureType::Queen => {if self.color==Color::White {"♕"} else {"♛"}}
            FigureType::King => {if self.color==Color::White {"♔"} else {"♚"}}
        };
        write!(f,"{}", symbol)
    }
}

pub struct FigureAndPosition {
    pub figure: Figure,
    pub pos: Position,
}

impl str::FromStr for FigureAndPosition {
    type Err = ChessError;

    fn from_str(desc: &str) -> Result<Self, Self::Err> {
        if desc.chars().count()!=3 {
            return Err(ChessError{
                msg: format!("FigureAndPosition.from_str: expected three characters like \"♔e1\" but found \"{}\"", desc),
                kind: ErrorKind::IllegalFormat,
            })
        }
        let split_point = desc.len()-2; // splitting is a bit more complicated since utf-8 chars like ♔ take more space than 1 byte
        let figure = desc[..split_point].parse::<Figure>()?;
        let pos = desc[split_point..].parse::<Position>()?;

        Ok(FigureAndPosition{
            figure,
            pos,
        })
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

impl PartialEq for FigureType {
    fn eq(&self, other: &FigureType) -> bool {
        fn rank(this: &FigureType) -> u8 {
            match this {
                FigureType::Pawn => 1,
                FigureType::Rook(_) => 2,
                FigureType::Knight => 3,
                FigureType::Bishop => 4,
                FigureType::Queen => 5,
                FigureType::King => 6,
            }
        }
        rank(self)==rank(other)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RookType {
    QueenSide,
    KingSide,
    Promoted,
}
