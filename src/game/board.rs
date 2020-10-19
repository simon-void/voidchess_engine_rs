use crate::figure::{Figure, FigureType};
use crate::base::{Color, Position};
use crate::base::I8_RANGE_07;
use std::fmt::{Display, Formatter, Result};
use std::ops::Range;

static WHITE_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Color::White,};
static WHITE_QUEEN_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook, color: Color::White,};
static WHITE_KING_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook, color: Color::White,};
static WHITE_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Color::White,};
static WHITE_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Color::White,};
static WHITE_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Color::White,};
static WHITE_KING: Figure = Figure {fig_type:FigureType::King, color: Color::White,};

static BLACK_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Color::Black,};
static BLACK_QUEEN_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook, color: Color::Black,};
static BLACK_KING_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook, color: Color::Black,};
static BLACK_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Color::Black,};
static BLACK_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Color::Black,};
static BLACK_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Color::Black,};
static BLACK_KING: Figure = Figure {fig_type:FigureType::King, color: Color::Black,};

#[derive(Clone, Debug)]
pub struct Board {
    state: [Option<Figure>; 64],
    number_of_figures: isize,
}

impl Board {
    pub fn classic() -> Board {
        Board {
            number_of_figures: 16,
            state: [
                Some(WHITE_QUEEN_SIDE_ROOK),
                Some(WHITE_KNIGHT),
                Some(WHITE_BISHOP),
                Some(WHITE_QUEEN),
                Some(WHITE_KING),
                Some(WHITE_BISHOP),
                Some(WHITE_KNIGHT),
                Some(WHITE_KING_SIDE_ROOK),
                Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN),
                Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN),
                Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN),
                Some(BLACK_QUEEN_SIDE_ROOK),
                Some(BLACK_KNIGHT),
                Some(BLACK_BISHOP),
                Some(BLACK_QUEEN),
                Some(BLACK_KING),
                Some(BLACK_BISHOP),
                Some(BLACK_KNIGHT),
                Some(BLACK_KING_SIDE_ROOK),
            ],
        }
    }

    pub fn empty() -> Board {
        Board {
            number_of_figures: 0,
            state: [None; 64],
        }
    }

    pub fn get_all_figures_of_color(&self, color: Color) -> [Option<(Figure, Position)>; 16] {
        let mut figures: [Option<(Figure, Position)>; 16] = [None; 16];
        let mut next_index: usize = 0;
        for state_index in USIZE_RANGE_063 {
            if let Some(figure) = self.state[state_index] {
                if figure.color == color {
                    figures[next_index] = Some(
                        (figure, Position::from_index_unchecked(state_index))
                    );
                    next_index += 1;
                }
            }
        }
        figures
    }

    pub fn get_white_and_black_figures(&self) -> ([Option<(FigureType, Position)>; 16],[Option<(FigureType, Position)>; 16]) {
        let mut white_figures: [Option<(FigureType, Position)>; 16] = [None; 16];
        let mut black_figures: [Option<(FigureType, Position)>; 16] = [None; 16];
        let mut next_white_index: usize = 0;
        let mut next_black_index: usize = 0;

        for state_index in USIZE_RANGE_063 {
            if let Some(figure) = self.state[state_index] {
                if figure.color == Color::White {
                    white_figures[next_white_index] = Some(
                        (figure.fig_type, Position::from_index_unchecked(state_index))
                    );
                    next_white_index += 1;
                } else {
                    black_figures[next_black_index] = Some(
                        (figure.fig_type, Position::from_index_unchecked(state_index))
                    );
                    next_black_index += 1;
                }
            }
        }
        (white_figures, black_figures)
    }

    pub fn get_figure(&self, pos: Position) -> Option<Figure> {
        self.state[pos.index]
    }

    /**
    * returns if a figure was caught/replaced on that position
    */
    pub fn set_figure(&mut self, pos: Position, figure: Figure) -> bool {
        // match opt_figure {
        //     None => println!("clear figure on {}", pos),
        //     Some(figure) => println!("set figure {} on {}", figure, pos),
        // }
        let old_content = self.state[pos.index];
        self.state[pos.index] = Some(figure);

        if old_content.is_some() {
            true
        } else {
            self.number_of_figures += 1;
            false
        }
    }

    pub fn clear_field(&mut self, pos: Position) {
        self.number_of_figures = self.number_of_figures - 1;
        self.state[pos.index] = None;
    }

    pub fn contains_sufficient_material_to_continue(&self) -> bool {
        if self.number_of_figures > 6 {
            return true;
        }

        let mut white_knight_nr = 0;
        let mut found_white_bishop = false;
        let mut black_knight_nr = 0;
        let mut found_black_bishop = false;

        for state_index in USIZE_RANGE_063 {
            if let Some(figure) = self.state[state_index] {
                match figure.fig_type {
                    FigureType::Pawn | FigureType::Rook | FigureType::Queen => {return true;}
                    FigureType::Knight => {
                        match figure.color {
                            Color::Black => { black_knight_nr += 1; }
                            Color::White => { white_knight_nr += 1; }
                        }
                    }
                    FigureType::Bishop => {
                        match figure.color {
                            Color::Black => {
                                // this is basically a black_bishop_nr == 2 check
                                if found_black_bishop {
                                    return true;
                                }
                                found_black_bishop = true;
                            }
                            Color::White => {
                                // this is basically a black_bishop_nr == 2 check
                                if found_white_bishop {
                                    return true;
                                }
                                found_white_bishop = true;
                            }
                        }
                    }
                    FigureType::King => {}
                }
            }
        }

        (found_white_bishop && white_knight_nr != 0) ||
            (found_black_bishop && black_knight_nr != 0) ||
            (white_knight_nr>2) || (black_knight_nr>2)
    }

    pub fn is_empty(&self, pos: Position) -> bool {
        self.get_figure(pos).is_none()
    }

    pub fn contains_figure(&self, pos: Position, fig_type: FigureType, color: Color) -> bool {
        match self.state[pos.index] {
            None => false,
            Some(figure) => {
                figure.fig_type == fig_type && figure.color == color
            }
        }
    }

    pub fn get_content_type(&self, pos: Position, color: Color) -> FieldContent {
        match self.get_figure(pos) {
            Some(figure) => if figure.color==color {
                FieldContent::OwnFigure
            } else {
                FieldContent::OpponentFigure
            },
            None => FieldContent::Empty,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f);
        for row_index in I8_RANGE_07.rev() {
            for column_index in I8_RANGE_07 {
                let figure_index = Position::new_unchecked(column_index, row_index).index;
                let fig_option = self.state[figure_index];
                match fig_option {
                    None => write!(f, "_"),
                    Some(figure) => write!(f, "{}", figure),
                }?;
            }
            writeln!(f, " {}", row_index + 1);
        }
        writeln!(f, "abcdefgh")
    }
}

pub const USIZE_RANGE_063: Range<usize> = 0..64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldContent {
    Empty, OwnFigure, OpponentFigure,
}
