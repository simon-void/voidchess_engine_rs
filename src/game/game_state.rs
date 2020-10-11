use crate::base::{Color, Position, Move, PawnPromotion, Moves, ChessError, ErrorKind, Direction};
use crate::figure::{Figure, FigureType, RookType, FigureAndPosition};
use crate::game::Board;
use tinyvec::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Board,
    pub turn_by: Color,
    white_king_pos: Position,
    black_king_pos: Position,
    pub en_passant_intercept_pos: Option<Position>,
    pub is_white_queen_side_castling_possible: bool,
    pub is_white_king_side_castling_possible: bool,
    pub is_black_queen_side_castling_possible: bool,
    pub is_black_king_side_castling_possible: bool,
}

impl GameState {
    pub fn classic() -> GameState {
        GameState {
            board: Board::classic(),
            turn_by: Color::White,
            white_king_pos: "e1".parse::<Position>().ok().unwrap(),
            black_king_pos: "e8".parse::<Position>().ok().unwrap(),
            en_passant_intercept_pos: None,
            is_white_queen_side_castling_possible: true,
            is_white_king_side_castling_possible: true,
            is_black_queen_side_castling_possible: true,
            is_black_king_side_castling_possible: true,
        }
    }


    pub fn from_manual_config(
        turn_by: Color,
        en_passant_intercept_pos: Option<Position>,
        positioned_figures: Vec<FigureAndPosition>
    ) -> Result<GameState, ChessError> {
        let mut board = Board::empty();
        let mut opt_white_king_pos: Option<Position> = None;
        let mut opt_black_king_pos: Option<Position> = None;

        for figure_and_pos in positioned_figures {
            let field_was_already_in_use = board.set_figure(figure_and_pos.pos, figure_and_pos.figure);
            if field_was_already_in_use {
                return Err(ChessError{
                    msg: format!("multiple figures placed on {}", figure_and_pos.pos),
                    kind: ErrorKind::IllegalConfiguration
                })
            }
            match figure_and_pos.figure.fig_type {
                FigureType::Pawn => {
                    let pawn_pos_row = figure_and_pos.pos.row;
                    if pawn_pos_row==0 || pawn_pos_row==7 {
                        return Err(ChessError{
                            msg: format!("can't place a pawn on {}", figure_and_pos.pos),
                            kind: ErrorKind::IllegalConfiguration
                        })
                    }
                },
                FigureType::King => {
                    match figure_and_pos.figure.color {
                        Color::White => {
                            if opt_white_king_pos.is_some() {
                                return Err(ChessError{
                                    msg: format!("can't place a pawn on {}. That row isn't reachable for a pawn.", figure_and_pos.pos),
                                    kind: ErrorKind::IllegalConfiguration
                                })
                            }
                            opt_white_king_pos = Some(figure_and_pos.pos);
                        },
                        Color::Black => {
                            if opt_black_king_pos.is_some() {
                                return Err(ChessError{
                                    msg: format!("can't place a pawn on {}. That row isn't reachable for a pawn.", figure_and_pos.pos),
                                    kind: ErrorKind::IllegalConfiguration
                                })
                            }
                            opt_black_king_pos = Some(figure_and_pos.pos);
                        },
                    }
                },
                _ => {},
            };
        }

        // check en-passant
        if let Some(en_passant_pos) = en_passant_intercept_pos {
            let (
                expected_row,
                expected_row_in_text,
                forward_dir,
            ) = match turn_by {
                Color::White => {
                    (5 as i8, 6 as i8, Direction::Down)
                }
                Color::Black => {
                    (2 as i8, 3 as i8, Direction::Up)
                }
            };
            if en_passant_pos.row != expected_row {
                return Err(ChessError {
                    msg: format!("it's {}'s turn so the en-passant position has to be on the {}th row but it's {}.", turn_by, expected_row_in_text, en_passant_pos),
                    kind: ErrorKind::IllegalConfiguration,
                })
            }
            let forward_pawn_pos = en_passant_pos.step(forward_dir).unwrap();
            let mut contains_correct_pawn = false;
            if let Some(forward_figure) = board.get_figure(forward_pawn_pos) {
                if forward_figure.fig_type==FigureType::Pawn && forward_figure.color!=turn_by {
                    contains_correct_pawn = true;
                }
            }
            if !contains_correct_pawn {
                return Err(ChessError {
                    msg: format!("since {} is an en-passant pos, there should be a {} pawn on {} but isn't.", en_passant_pos, turn_by.toggle(), forward_pawn_pos),
                    kind: ErrorKind::IllegalConfiguration,
                })
            }

            let backward_empty_pos = en_passant_pos.step(forward_dir.reverse()).unwrap();
            if !board.is_empty(backward_empty_pos) {
                return Err(ChessError {
                    msg: format!("since {} is an en-passant pos, the position behind it ({}) should be empty but isn't.", en_passant_pos, backward_empty_pos),
                    kind: ErrorKind::IllegalConfiguration,
                })
            }
        }

        let white_king_pos = match opt_white_king_pos {
            Some(pos) => pos,
            None => {
                return Err(ChessError{
                    msg: format!("no white king configured"),
                    kind: ErrorKind::IllegalConfiguration
                })
            },
        };
        let black_king_pos = match opt_black_king_pos {
            Some(pos) => pos,
            None => {
                return Err(ChessError{
                    msg: format!("no white king configured"),
                    kind: ErrorKind::IllegalConfiguration
                })
            },
        };

        let game_state = GameState {
            board,
            turn_by,
            white_king_pos,
            black_king_pos,
            en_passant_intercept_pos,
            is_white_queen_side_castling_possible: false,
            is_white_king_side_castling_possible: false,
            is_black_queen_side_castling_possible: false,
            is_black_king_side_castling_possible: false,
        };

        if game_state.can_passive_players_king_be_caught() {
            return Err(ChessError{
                msg: format!("passive king is in check {}", game_state.board),
                kind: ErrorKind::IllegalConfiguration
            })
        }

        Ok(game_state)
    }

    pub fn get_reachable_moves(&self) -> Moves {
        let mut move_collector: Moves = tiny_vec!();
        let figures_of_color_with_pos: [Option<(Figure, Position)>; 16] =
            self.board.get_all_figures_of_color(self.turn_by);

        for i in 0..16 as usize {
            match figures_of_color_with_pos[i] {
                Some((figure, pos)) => {
                    figure.for_reachable_moves(pos, self, &mut move_collector);
                },
                None => {
                    break;
                }
            }
        }

        move_collector
    }

    pub fn do_move(&self, next_move: Move) -> GameState {
        if next_move.to==self.white_king_pos || next_move.to==self.black_king_pos {
            panic!("move {} would capture a king on game {}", next_move, self.board)
        }
        let mut new_board = self.board.clone();
        let moving_figure: Figure = self.board.get_figure(next_move.from).unwrap();
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
                let is_castling = if let Some(figure_to_be_caught) = self.board.get_figure(next_move.to) {
                    figure_to_be_caught.color == self.turn_by
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
                fn compute_pawn_move_type(this: &GameState, pawn_move: Move) -> PawnMoveType {
                    if pawn_move.from.get_row_distance(pawn_move.to) == 2 {
                        return PawnMoveType::DoubleStep
                    }
                    if let Some(en_passant_pos) = this.en_passant_intercept_pos {
                        if pawn_move.to == en_passant_pos {
                            return PawnMoveType::EnPassantIntercept
                        }
                    }
                    PawnMoveType::SingleStep
                }

                match compute_pawn_move_type(self, next_move) {
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
                            Some(Position::unchecked_new(
                                next_move.to.column,
                                (next_move.from.row + next_move.to.row) / 2,
                            )),
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

        GameState {
            board: new_board,
            turn_by: self.turn_by.toggle(),
            white_king_pos: new_white_king_pos,
            black_king_pos: new_black_king_pos,
            en_passant_intercept_pos: new_en_passant_intercept_pos,
            is_white_queen_side_castling_possible: new_is_white_queen_side_castling_possible,
            is_white_king_side_castling_possible: new_is_white_king_side_castling_possible,
            is_black_queen_side_castling_possible: new_is_black_queen_side_castling_possible,
            is_black_king_side_castling_possible: new_is_black_king_side_castling_possible,
        }
    }

    pub fn get_passive_kings_pos(&self) -> Position {
        match self.turn_by {
            Color::White => self.black_king_pos,
            Color::Black => self.white_king_pos,
        }
    }

    pub fn can_passive_players_king_be_caught(&self) -> bool {
        //TODO
        false
    }
}

fn do_normal_move(
    new_board: &mut Board,
    next_move: Move,
) -> bool {
    let moving_figure: Figure = new_board.get_figure(next_move.from).expect("field the figure moves from is empty");
    new_board.clear_field(next_move.from);
    new_board.set_figure(next_move.to, moving_figure)
}

/**
* returns - the new position of the king
*/
fn do_castling_move(
    new_board: &mut Board,
    next_move: Move,
) -> Position {
    let is_king_side_castling = next_move.to.column > next_move.from.column;
    let castling_row = next_move.from.row;
    let king_to: Position;
    let rook_to: Position;
    if is_king_side_castling {
        king_to = Position::unchecked_new(6, castling_row);
        rook_to = Position::unchecked_new(5, castling_row)
    } else {
        king_to = Position::unchecked_new(2, castling_row);
        rook_to = Position::unchecked_new(3, castling_row)
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
    new_board: &mut Board,
    next_move: Move,
) {
    do_normal_move(new_board, next_move);
    let double_stepped_pawn_pos =
        Position::unchecked_new(next_move.to.column, next_move.from.row);
    new_board.clear_field(double_stepped_pawn_pos)
}

enum PawnMoveType {
    SingleStep, DoubleStep, EnPassantIntercept,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}'s turn", self.turn_by);
        writeln!(f, "{}", self.board)
    }
}