use crate::base::{Color, Position, Move, PawnPromotion, Moves, ChessError, ErrorKind, Direction, Deactivatable};
use crate::figure::{Figure, FigureType, FigureAndPosition};
use crate::game::{Board};
use tinyvec::*;
use std::{fmt,str};

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Board,
    pub turn_by: Color,
    white_king_pos: Position,
    black_king_pos: Position,
    pub en_passant_intercept_pos: Option<Position>,
    pub is_white_queen_side_castling_still_possible: Deactivatable,
    pub is_white_king_side_castling_still_possible: Deactivatable,
    pub is_black_queen_side_castling_still_possible: Deactivatable,
    pub is_black_king_side_castling_still_possible: Deactivatable,
}

impl GameState {
    pub fn classic() -> GameState {
        GameState {
            board: Board::classic(),
            turn_by: Color::White,
            white_king_pos: "e1".parse::<Position>().ok().unwrap(),
            black_king_pos: "e8".parse::<Position>().ok().unwrap(),
            en_passant_intercept_pos: None,
            is_white_queen_side_castling_still_possible: Deactivatable::new(true),
            is_white_king_side_castling_still_possible: Deactivatable::new(true),
            is_black_queen_side_castling_still_possible: Deactivatable::new(true),
            is_black_king_side_castling_still_possible: Deactivatable::new(true),
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

        fn board_contains_rook_at(pos: Position, color: Color, board: &Board) -> bool {
            if let Some(figure) = board.get_figure(pos) {
                figure.fig_type==FigureType::Rook && figure.color==color
            } else {
                false
            }
        }

        let is_white_king_on_starting_pos = white_king_pos == WHITE_KING_STARTING_POS;
        let is_black_king_on_starting_pos = black_king_pos == BLACK_KING_STARTING_POS;

        let is_white_queen_side_rook_on_starting_pos = board_contains_rook_at(
            WHITE_QUEEN_SIDE_ROOK_STARTING_POS, Color::White, &board,
        );
        let is_white_king_side_rook_on_starting_pos = board_contains_rook_at(
            WHITE_KING_SIDE_ROOK_STARTING_POS, Color::White, &board,
        );
        let is_black_queen_side_rook_on_starting_pos = board_contains_rook_at(
            BLACK_QUEEN_SIDE_ROOK_STARTING_POS, Color::White, &board,
        );
        let is_black_king_side_rook_on_starting_pos = board_contains_rook_at(
            BLACK_KING_SIDE_ROOK_STARTING_POS, Color::White, &board,
        );
        let mut is_white_queen_side_castling_possible = Deactivatable::new(is_white_king_on_starting_pos && is_white_queen_side_rook_on_starting_pos);
        let mut is_white_king_side_castling_possible = Deactivatable::new(is_white_king_on_starting_pos && is_white_king_side_rook_on_starting_pos);
        let mut is_black_queen_side_castling_possible = Deactivatable::new(is_black_king_on_starting_pos && is_black_queen_side_rook_on_starting_pos);
        let mut is_black_king_side_castling_possible = Deactivatable::new(is_black_king_on_starting_pos && is_black_king_side_rook_on_starting_pos);

        let game_state = GameState {
            board,
            turn_by,
            white_king_pos,
            black_king_pos,
            en_passant_intercept_pos,
            is_white_queen_side_castling_still_possible: is_white_queen_side_castling_possible,
            is_white_king_side_castling_still_possible: is_white_king_side_castling_possible,
            is_black_queen_side_castling_still_possible: is_black_queen_side_castling_possible,
            is_black_king_side_castling_still_possible: is_black_king_side_castling_possible,
        };

        if game_state.can_passive_players_king_be_caught() {
            return Err(ChessError{
                msg: format!("passive king is in check {}", game_state.board),
                kind: ErrorKind::IllegalConfiguration
            })
        }

        Ok(game_state)
    }

    pub fn toggle_colors(&self) -> GameState {
        fn toggle_pos(pos: Position) -> Position {
            return Position::new_unchecked(
                pos.column, 7-pos.row,
            )
        }
        fn toggle_figures_on_board_to(color: Color, figure_array: [Option<(FigureType, Position)>; 16], board: &mut Board) {
            for opt_figure_type_and_pos in figure_array.iter() {
                if let Some((figure_type, pos)) = opt_figure_type_and_pos {
                    board.set_figure(toggle_pos(*pos), Figure{ fig_type: *figure_type, color });
                } else {
                    break;
                }
            }
        }
        let mut toggled_board = Board::empty();
        let (array_of_opt_white_figures, array_of_opt_black_figures) = self.board.get_white_and_black_figures();
        toggle_figures_on_board_to(Color::Black, array_of_opt_white_figures, &mut toggled_board);
        toggle_figures_on_board_to(Color::White, array_of_opt_black_figures, &mut toggled_board);

        return GameState {
            board: toggled_board,
            turn_by: self.turn_by.toggle(),
            white_king_pos: toggle_pos(self.white_king_pos),
            black_king_pos: toggle_pos(self.black_king_pos),
            en_passant_intercept_pos: self.en_passant_intercept_pos.map(|pos|{toggle_pos(pos)}),
            is_white_queen_side_castling_still_possible: self.is_black_queen_side_castling_still_possible,
            is_white_king_side_castling_still_possible: self.is_black_king_side_castling_still_possible,
            is_black_queen_side_castling_still_possible: self.is_white_queen_side_castling_still_possible,
            is_black_king_side_castling_still_possible: self.is_white_king_side_castling_still_possible,
        }
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
        debug_assert!(
            next_move.to != self.white_king_pos && next_move.to != self.black_king_pos,
            "move {} would capture a king on game {}", next_move, self.board
        );
        debug_assert!(
            self.board.contains_figure(self.white_king_pos, FigureType::King, Color::White),
            "couldn't find white kind at white_king_pos {} on board {}", self.white_king_pos, self.board
        );
        debug_assert!(
            self.board.contains_figure(self.black_king_pos, FigureType::King, Color::Black),
            "couldn't find black kind at black_king_pos {} on board {}", self.black_king_pos, self.board
        );

        let mut new_board = self.board.clone();
        let moving_figure: Figure = self.board.get_figure(next_move.from).unwrap();

        let mut new_is_white_queen_side_castling_possible = self.is_white_queen_side_castling_still_possible;
        let mut new_is_white_king_side_castling_possible = self.is_white_king_side_castling_still_possible;
        let mut new_is_black_queen_side_castling_possible = self.is_black_queen_side_castling_still_possible;
        let mut new_is_black_king_side_castling_possible = self.is_black_king_side_castling_still_possible;

        if next_move.from == WHITE_QUEEN_SIDE_ROOK_STARTING_POS || next_move.to == WHITE_QUEEN_SIDE_ROOK_STARTING_POS {
            new_is_white_queen_side_castling_possible.deactivate()
        }
        if next_move.from == WHITE_KING_SIDE_ROOK_STARTING_POS || next_move.to == WHITE_KING_SIDE_ROOK_STARTING_POS {
            new_is_white_king_side_castling_possible.deactivate()
        }
        if next_move.from == BLACK_QUEEN_SIDE_ROOK_STARTING_POS || next_move.to == BLACK_QUEEN_SIDE_ROOK_STARTING_POS {
            new_is_black_queen_side_castling_possible.deactivate()
        }
        if next_move.from == BLACK_KING_SIDE_ROOK_STARTING_POS || next_move.to == BLACK_KING_SIDE_ROOK_STARTING_POS {
            new_is_black_king_side_castling_possible.deactivate()
        }

        let (
            new_white_king_pos,
            new_black_king_pos,
            new_en_passant_intercept_pos,
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
                    Color::White => {
                        new_is_white_queen_side_castling_possible.deactivate();
                        new_is_white_king_side_castling_possible.deactivate();
                        (
                            new_king_pos,
                            self.black_king_pos,
                            None,
                        )
                    }
                    Color::Black => {
                        new_is_black_queen_side_castling_possible.deactivate();
                        new_is_black_king_side_castling_possible.deactivate();
                        (
                            self.white_king_pos,
                            new_king_pos,
                            None,
                        )
                    }
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
                        )
                    },
                    PawnMoveType::DoubleStep => {
                        do_normal_move(&mut new_board, next_move);
                        (
                            self.white_king_pos, self.black_king_pos,
                            Some(Position::new_unchecked(
                                next_move.to.column,
                                (next_move.from.row + next_move.to.row) / 2,
                            )),
                        )
                    },
                    PawnMoveType::EnPassantIntercept => {
                        do_en_passant_move(&mut new_board, next_move);
                        (
                            self.white_king_pos, self.black_king_pos,
                            None,
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
                )
            },
        };

        GameState {
            board: new_board,
            turn_by: self.turn_by.toggle(),
            white_king_pos: new_white_king_pos,
            black_king_pos: new_black_king_pos,
            en_passant_intercept_pos: new_en_passant_intercept_pos,
            is_white_queen_side_castling_still_possible: new_is_white_queen_side_castling_possible,
            is_white_king_side_castling_still_possible: new_is_white_king_side_castling_possible,
            is_black_queen_side_castling_still_possible: new_is_black_queen_side_castling_possible,
            is_black_king_side_castling_still_possible: new_is_black_king_side_castling_possible,
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

impl str::FromStr for GameState {
    type Err = ChessError;

    fn from_str(desc: &str) -> Result<Self, Self::Err> {
        let trimmed_desc = desc.trim();
        if trimmed_desc.is_empty() {
            return Ok(GameState::classic())
        }
        let mut token_iter = trimmed_desc.split(" ").into_iter();

        // let desc_contains_figures: bool = "♔♕♗♘♖♙♚♛♝♞♜♟".chars().any(|symbol|{desc.contains(symbol)});
        let desc_contains_moves: bool = trimmed_desc.is_empty() || trimmed_desc.contains("-");
        if desc_contains_moves {
            game_by_moves_from_start(token_iter)
        } else {
            game_by_figures_on_board(token_iter)
        }
    }
}

fn game_by_moves_from_start(token_iter: str::Split<&str>) -> Result<GameState, ChessError> {
    let mut game_state = GameState::classic();
    for token in token_iter {
        let a_move = token.parse::<Move>()?;
        game_state = game_state.do_move(a_move);
    }
    Ok(game_state)
}

fn game_by_figures_on_board(mut token_iter: str::Split<&str>) -> Result<GameState, ChessError> {
    let first_token = token_iter.next().unwrap();
    let turn_by = match first_token {
        "white" => Color::White,
        "black" => Color::Black,
        _ => {
            return Err(ChessError {
                msg: format!("the first token has to be either 'white' or 'black' but was {}", first_token),
                kind: ErrorKind::IllegalConfiguration,
            })
        },
    };

    let mut positioned_figures: Vec<FigureAndPosition> = vec![];
    let mut opt_en_passant_pos: Option<Position> = None;

    for token in token_iter {
        // tokens should either start with a figure char (from "♔♕♗♘♖♙♚♛♝♞♜♟") or E (for en-passant)
        // followed by a position between "a1" and "h8"
        if token.starts_with("E") {
            let en_passant_pos = token[1..].parse::<Position>()?;
            if let Some(old_en_passant_pos) = opt_en_passant_pos {
                return Err(ChessError {
                    msg: format!("there are two en-passant tokens present (on {} and {}) but only one is allowed.", old_en_passant_pos, en_passant_pos),
                    kind: ErrorKind::IllegalConfiguration,
                })
            }
            opt_en_passant_pos = Some(en_passant_pos);
        } else {
            let figure_and_pos = token.parse::<FigureAndPosition>()?;
            positioned_figures.push(figure_and_pos);
        }
    }

    let game_state = GameState::from_manual_config(turn_by, opt_en_passant_pos, positioned_figures)?;
    Ok(game_state)
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
        king_to = Position::new_unchecked(6, castling_row);
        rook_to = Position::new_unchecked(5, castling_row)
    } else {
        king_to = Position::new_unchecked(2, castling_row);
        rook_to = Position::new_unchecked(3, castling_row)
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
        Position::new_unchecked(next_move.to.column, next_move.from.row);
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

pub static WHITE_KING_STARTING_POS: Position = Position::new_unchecked(4, 0);
static WHITE_KING_SIDE_ROOK_STARTING_POS: Position = Position::new_unchecked(7, 0);
static WHITE_QUEEN_SIDE_ROOK_STARTING_POS: Position = Position::new_unchecked(0, 0);
pub static BLACK_KING_STARTING_POS: Position = Position::new_unchecked(4, 7);
static BLACK_KING_SIDE_ROOK_STARTING_POS: Position = Position::new_unchecked(7, 7);
static BLACK_QUEEN_SIDE_ROOK_STARTING_POS: Position = Position::new_unchecked(0, 7);

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::game::{GameState};

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing, expected_nr_of_reachable_moves,
    case("", 20),
    case("e2-e4 e7-e5", 29),
    case("e2-e4 a7-a6", 30),
    case("e2-e4 b7-b5", 29),
    case("a2-a4 a7-a6 a4-a5 b7-b5", 22), // en-passant
    case("white ♔a1 ♙b5 ♟a6 Ec6 ♟c5 ♚e8", 6), // en-passant
    case("white ♖a2 ♔e2 ♖h2 ♚e8", 27), // no castling
    case("white ♖a1 ♔e1 ♖h1 ♚e8", 26), // castling
    case("white ♖a1 ♔e1 ♖h1 ♙a2 ♜h2 ♚e8", 15), // castling
    case("white ♔a1 ♚c1", 3), // king can be caught
    case("white ♔a1 ♚b1", 3), // king can be caught
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_reachable_moves(
        game_config_testing: &str,
        expected_nr_of_reachable_moves: usize,
    ) {
        let game_state = game_config_testing.parse::<GameState>().unwrap();
        let white_nr_of_reachable_moves = game_state.get_reachable_moves().len();
        assert_eq!(white_nr_of_reachable_moves, expected_nr_of_reachable_moves, "nr of reachable moves");

        let black_nr_of_reachable_moves = game_state.toggle_colors().get_reachable_moves().len();
        assert_eq!(black_nr_of_reachable_moves, expected_nr_of_reachable_moves, "nr of reachable moves");
    }
}
