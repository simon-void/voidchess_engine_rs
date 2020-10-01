use crate::base::{Color, Position, Move, PawnPromotion};
use crate::figure::{Figure, FigureType, RookType};
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

#[derive(Debug)]
pub struct MatchState {
    board: [[Option<Figure>; 8]; 8],
    next_turn_by: Color,
    white_king_pos: Position,
    black_king_pos: Position,
    en_passant_intercept_pos: Option<Position>,
    is_white_queen_side_castling_possible: bool,
    is_white_king_side_castling_possible: bool,
    is_black_queen_side_castling_possible: bool,
    is_black_king_side_castling_possible: bool,
}

impl MatchState {
    pub fn new() -> MatchState {
        MatchState {
            board: [
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
            next_turn_by: Color::White,
            white_king_pos: "e1".parse::<Position>().unwrap(),
            black_king_pos: "e8".parse::<Position>().unwrap(),
            en_passant_intercept_pos: None,
            is_white_queen_side_castling_possible: true,
            is_white_king_side_castling_possible: true,
            is_black_queen_side_castling_possible: true,
            is_black_king_side_castling_possible: true,
        }
    }

    pub fn do_move(&self, next_move: Move) -> MatchState {
        if next_move.to==self.white_king_pos || next_move.to==self.black_king_pos {
            panic!("move {} would capture a king on game {}", next_move, self)
        }
        let mut new_board = self.board.clone();
        let moving_figure: Figure = get_figure(&self.board, next_move.from).unwrap();
        let (
            new_white_king_pos,
            new_black_king_pos,
            new_en_passant_intercept_pos,
            new_is_white_queen_side_castling_possible,
            new_is_white_king_side_castling_possible,
            new_is_black_queen_side_castling_possible,
            new_is_black_king_side_castling_possible,
        ) = match moving_figure.fig_type {
            FigureType::King => {
                let is_castling = if let Some(figure_to_be_caught) = get_figure(&self.board, next_move.to) {
                    figure_to_be_caught.color == self.next_turn_by
                } else {
                    false
                };
                let new_king_pos: Position;
                if is_castling {
                    new_king_pos = do_castling_move(&mut new_board, next_move);
                } else {
                    do_normal_move(&mut new_board, next_move);
                    new_king_pos = next_move.to;
                };

                match moving_figure.color {
                    Color::White => (
                        new_king_pos,
                        self.black_king_pos,
                        None, false, false,
                        self.is_black_queen_side_castling_possible,
                        self.is_black_king_side_castling_possible,
                    ),
                    Color::Black => (
                        self.white_king_pos,
                        new_king_pos,
                        None,
                        self.is_white_queen_side_castling_possible,
                        self.is_white_king_side_castling_possible,
                        false, false,
                    ),
                }
            },
            FigureType::Rook(rook_type) => {
                do_normal_move(&mut new_board, next_move);
                match rook_type {
                    RookType::QueenSide => match moving_figure.color {
                        Color::White => (
                            self.white_king_pos, self.black_king_pos, None,
                            false,
                            self.is_white_king_side_castling_possible,
                            self.is_black_queen_side_castling_possible,
                            self.is_black_king_side_castling_possible,
                        ),
                        Color::Black => (
                            self.white_king_pos, self.black_king_pos, None,
                            self.is_white_queen_side_castling_possible,
                            self.is_white_king_side_castling_possible,
                            false,
                            self.is_black_king_side_castling_possible,
                        ),
                    },
                    RookType::KingSide => match moving_figure.color {
                        Color::White => (
                            self.white_king_pos, self.black_king_pos, None,
                            self.is_white_queen_side_castling_possible,
                            false,
                            self.is_black_queen_side_castling_possible,
                            self.is_black_king_side_castling_possible,
                        ),
                        Color::Black => (
                            self.white_king_pos, self.black_king_pos, None,
                            self.is_white_queen_side_castling_possible,
                            self.is_white_king_side_castling_possible,
                            self.is_black_queen_side_castling_possible,
                            false,
                        ),
                    },
                    RookType::Promoted => (
                        self.white_king_pos, self.black_king_pos, None,
                        self.is_white_queen_side_castling_possible,
                        self.is_white_king_side_castling_possible,
                        self.is_black_queen_side_castling_possible,
                        self.is_black_king_side_castling_possible,
                    ),
                }
            },
            FigureType::Pawn => {
                match self.compute_pawn_move_type(next_move) {
                    PawnMoveType::SingleStep => {
                        do_normal_move(&mut new_board, next_move);
                        (
                            self.white_king_pos, self.black_king_pos,
                            None,
                            self.is_white_queen_side_castling_possible,
                            self.is_white_king_side_castling_possible,
                            self.is_black_queen_side_castling_possible,
                            self.is_black_king_side_castling_possible,
                        )
                    },
                    PawnMoveType::DoubleStep => {
                        do_normal_move(&mut new_board, next_move);
                        (
                            self.white_king_pos, self.black_king_pos,
                            Some(Position { column: next_move.to.column, row: (next_move.from.row + next_move.to.row) / 2 }),
                            self.is_white_queen_side_castling_possible,
                            self.is_white_king_side_castling_possible,
                            self.is_black_queen_side_castling_possible,
                            self.is_black_king_side_castling_possible,
                        )
                    },
                    PawnMoveType::EnPassantIntercept => {
                        do_en_passant_move(&mut new_board, next_move);
                        (
                            self.white_king_pos, self.black_king_pos,
                            None,
                            self.is_white_queen_side_castling_possible,
                            self.is_white_king_side_castling_possible,
                            self.is_black_queen_side_castling_possible,
                            self.is_black_king_side_castling_possible,
                        )
                    },
                }
            },
            _ => {
                do_normal_move(&mut new_board, next_move);
                (
                    self.white_king_pos,
                    self.black_king_pos,
                    None,
                    self.is_white_queen_side_castling_possible,
                    self.is_white_king_side_castling_possible,
                    self.is_black_queen_side_castling_possible,
                    self.is_black_king_side_castling_possible,
                )
            },
        };

        MatchState {
            board: new_board,
            next_turn_by: self.next_turn_by.toggle(),
            white_king_pos: new_white_king_pos,
            black_king_pos: new_black_king_pos,
            en_passant_intercept_pos: new_en_passant_intercept_pos,
            is_white_queen_side_castling_possible: new_is_white_queen_side_castling_possible,
            is_white_king_side_castling_possible: new_is_white_king_side_castling_possible,
            is_black_queen_side_castling_possible: new_is_black_queen_side_castling_possible,
            is_black_king_side_castling_possible: new_is_black_king_side_castling_possible,
        }
    }

    fn compute_pawn_move_type(&self, pawn_move: Move) -> PawnMoveType {
        if pawn_move.from.get_row_distance(pawn_move.to) == 2 {
            return PawnMoveType::DoubleStep
        }
        if let Some(en_passant_pos) = self.en_passant_intercept_pos {
            if pawn_move.to == en_passant_pos {
                return PawnMoveType::EnPassantIntercept
            }
        }
        PawnMoveType::SingleStep
    }
}

fn do_normal_move(
    new_board: &mut [[Option<Figure>; 8]; 8],
    next_move: Move,
) {
    let moving_figure: Option<Figure> = get_figure(new_board, next_move.from);
    set_figure(new_board, next_move.from, None);
    set_figure(new_board, next_move.to, moving_figure);
}

/**
* returns - the new position of the king
*/
fn do_castling_move(
    new_board: &mut [[Option<Figure>; 8]; 8],
    next_move: Move,
) -> Position {
    let is_king_side_castling = next_move.to.column > next_move.from.column;
    let castling_row = next_move.from.row;
    let king_to: Position;
    let rook_to: Position;
    if is_king_side_castling {
        king_to = Position {
            column: 6,
            row: castling_row,
        };
        rook_to = Position {
            column: 5,
            row: castling_row,
        }
    } else {
        king_to = Position {
            column: 2,
            row: castling_row,
        };
        rook_to = Position {
            column: 3,
            row: castling_row,
        }
    }
    // move the king
    // (this simplified approach only works in classical chess, not in all chess960 positions)
    do_normal_move(new_board, Move {
        from: next_move.from,
        to: king_to,
        pawn_promo: PawnPromotion::No,
    });
    // move the rook
    do_normal_move(new_board, Move {
        from: next_move.to,
        to: rook_to,
        pawn_promo: PawnPromotion::No,
    });

    king_to
}

fn do_en_passant_move(
    new_board: &mut [[Option<Figure>; 8]; 8],
    next_move: Move,
) {
    do_normal_move(new_board, next_move);
    let double_stepped_pawn_pos = Position {
        column: next_move.to.column,
        row: next_move.from.row,
    };
    set_figure(new_board, double_stepped_pawn_pos, None)
}

fn get_figure(board: &[[Option<Figure>; 8]; 8], pos: Position) -> Option<Figure> {
    board[pos.row as usize][pos.column as usize]
}

fn set_figure(board: &mut [[Option<Figure>; 8]; 8], pos: Position, opt_figure: Option<Figure>) {
    // match opt_figure {
    //     None => println!("clear figure on {}", pos),
    //     Some(figure) => println!("set figure {} on {}", figure, pos),
    // }
    board[pos.row as usize][pos.column as usize] = opt_figure
}

enum PawnMoveType {
    SingleStep, DoubleStep, EnPassantIntercept,
}

impl Display for MatchState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "");
        for row_index in (0..8).rev() {
            let row: [Option<Figure>; 8] = self.board[row_index];
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