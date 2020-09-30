use crate::base::Colour;

#[derive(Debug, Copy, Clone)]
pub struct Figure {
    pub fig_type: FigureType,
    pub color: Colour,
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
