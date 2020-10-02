use crate::figure::{Figure, FigureType, RookType};
use crate::base::Color;
use crate::Position;
use std::fmt::{Display, Formatter, Result};

static WHITE_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Color::White,};
static WHITE_QUEEN_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::QueenSide), color: Color::White,};
static WHITE_KING_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::KingSide), color: Color::White,};
static WHITE_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Color::White,};
static WHITE_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Color::White,};
static WHITE_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Color::White,};
static WHITE_KING: Figure = Figure {fig_type:FigureType::King, color: Color::White,};

static BLACK_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Color::Black,};
static BLACK_QUEEN_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::QueenSide), color: Color::Black,};
static BLACK_KING_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::KingSide), color: Color::Black,};
static BLACK_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Color::Black,};
static BLACK_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Color::Black,};
static BLACK_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Color::Black,};
static BLACK_KING: Figure = Figure {fig_type:FigureType::King, color: Color::Black,};

#[derive(Clone, Debug)]
pub struct Board {
    state: [[Option<Figure>; 8]; 8],
}

impl Board {
    pub fn classic() -> Board {
        Board {
            state: [
                [
                    Some(WHITE_QUEEN_SIDE_ROOK),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_BISHOP),
                    Some(WHITE_QUEEN),
                    Some(WHITE_KING),
                    Some(WHITE_BISHOP),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_KING_SIDE_ROOK),
                ],
                [Some(WHITE_PAWN); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(BLACK_PAWN); 8],
                [
                    Some(BLACK_QUEEN_SIDE_ROOK),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_BISHOP),
                    Some(BLACK_QUEEN),
                    Some(BLACK_KING),
                    Some(BLACK_BISHOP),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_KING_SIDE_ROOK),
                ],
            ],
        }
    }

    pub fn empty() -> Board {
        Board {
            state: [
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
        }
    }

    pub fn get_figure(&self, pos: Position) -> Option<Figure> {
        self.state[pos.row as usize][pos.column as usize]
    }

    pub fn set_figure(&mut self, pos: Position, opt_figure: Option<Figure>) {
        // match opt_figure {
        //     None => println!("clear figure on {}", pos),
        //     Some(figure) => println!("set figure {} on {}", figure, pos),
        // }
        self.state[pos.row as usize][pos.column as usize] = opt_figure
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "");
        for row_index in (0..8).rev() {
            let row: [Option<Figure>; 8] = self.state[row_index];
            row.iter().for_each(|&fig_option| {
                let symbol = match fig_option {
                    None => "_",
                    Some(figure) => {
                        match figure.fig_type {
                            FigureType::Pawn => match figure.color {
                                Color::White => "♙",
                                Color::Black => "♟",
                            },
                            FigureType::Rook(RookType) => match figure.color {
                                Color::White => "♖",
                                Color::Black => "♜",
                            },
                            FigureType::Knight => match figure.color {
                                Color::White => "♘",
                                Color::Black => "♞",
                            },
                            FigureType::Bishop => match figure.color {
                                Color::White => "♗",
                                Color::Black => "♝",
                            },
                            FigureType::Queen => match figure.color {
                                Color::White => "♕",
                                Color::Black => "♛",
                            },
                            FigureType::King => match figure.color {
                                Color::White => "♔",
                                Color::Black => "♚",
                            },
                        }
                    }
                };
                write!(f, "{}", symbol);
            });
            writeln!(f, " {}", row_index + 1);
        }
        writeln!(f, "abcdefgh")
    }
}