use crate::base::{Position, Color, Direction, MoveType, Move, CastlingType};
use crate::game::Board;
use crate::figure::FigureType;
use std::cmp::max;

pub fn is_king_in_check(king_pos: Position, color: Color, board: &Board) -> bool {
    let (forward_left, forward, forward_right) = Direction::forward_directions(color);

    is_king_straight_attackable(king_pos, color, forward, board) ||
        is_king_straight_attackable(king_pos, color, Direction::Left, board) ||
        is_king_straight_attackable(king_pos, color, Direction::Right, board) ||
        is_king_forward_diagonal_attackable(king_pos, color, forward_left, board) ||
        is_king_forward_diagonal_attackable(king_pos, color, forward_right, board) ||
        is_king_attackable_by_knight(king_pos, color, board) ||
        (
            (!king_pos.is_on_ground_row(color)) && (
                is_king_straight_attackable(king_pos, color, forward.reverse(), board) ||
                    is_king_backward_diagonal_attackable(king_pos, color, forward_left.reverse(), board) ||
                    is_king_backward_diagonal_attackable(king_pos, color, forward_right.reverse(), board)
            )
        )
}

pub fn is_king_straight_attackable(king_pos: Position, color: Color, dir: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, dir, board) {
        return match opponent_fig_type {
            FigureType::Rook | FigureType::Queen => true,
            FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

pub fn is_king_forward_diagonal_attackable(king_pos: Position, color: Color, diagonal: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, diagonal, board) {
        return match opponent_fig_type {
            FigureType::Bishop | FigureType::Queen => true,
            FigureType::Pawn | FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

fn is_king_backward_diagonal_attackable(king_pos: Position, color: Color, diagonal: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, diagonal, board) {
        return match opponent_fig_type {
            FigureType::Bishop | FigureType::Queen => true,
            FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

pub fn is_king_attackable_by_knight(king_pos: Position, color: Color, board: &Board) -> bool {
    for possible_knight_attacker_pos in king_pos.reachable_knight_positions(color, board) {
        if let Some(figure) = board.get_figure(possible_knight_attacker_pos) {
            // different color is already guaranteed by reachable_knight_positions
            if figure.fig_type == FigureType::Knight {
                return true;
            }
        }
    }
    false
}

/*
 * returns the optional figure_type and if that figure is right next to self
 */
fn get_first_opposite_color_figure_type_in_direction(
    king_pos: Position,
    king_color: Color,
    direction: Direction,
    board: &Board,
) -> Option<(FigureType, bool)> {
    let mut old_pos = king_pos;
    let mut is_attacker_next_to_self = true;
    loop {
        let new_pos = match old_pos.step(direction) {
            Some(pos) => {
                pos
            }
            None => { return None; }
        };
        if let Some(figure) = board.get_figure(new_pos) {
            return if figure.color == king_color {
                None
            } else {
                Some((figure.fig_type, is_attacker_next_to_self))
            };
        }
        old_pos = new_pos;
        is_attacker_next_to_self = false;
    }
}

/**
 * it's assumed that the passive king isn't in check at this point (because then the game should already by over).
 * this also means that the king
 */
pub fn is_king_in_check_after(latest_move: Move, king_pos: Position, color: Color, board: &Board) -> bool {
    match latest_move.move_type {
        MoveType::Castling(castling_type) => {
            let castling_rook_end_pos = if castling_type == CastlingType::KingSide {
                Position::new_unchecked(5, latest_move.to.row)
            } else {
                Position::new_unchecked(3, latest_move.to.row)
            };
            debug_assert!(!board.is_empty(castling_rook_end_pos), "{}", format!(
                "board at castling rock pos must contain rook but is empty. expected rook pos: {}, king_pos: {}, latest_move: {}, board: {}",
                castling_rook_end_pos, king_pos, latest_move, board
            ));
            if gives_chess(castling_rook_end_pos, king_pos, color, board).is_some() {
                return true;
            }
        }
        MoveType::EnPassant => {
            debug_assert!(!board.is_empty(latest_move.to), "{}", format!(
                "board is empty after enpassent move {} leading to board: {}",
                latest_move, board
            ));
            if gives_chess(latest_move.to, king_pos, color, board).is_some() ||
                find_attack_from_behind(latest_move.from, king_pos, color, board).is_some() {
                return true;
            }
            let taken_pawn_pos: Position = Position::new_unchecked(latest_move.to.column, latest_move.from.row);
            if find_attack_from_behind(taken_pawn_pos, king_pos, color, board).is_some() {
                return true;
            }
        }
        _ => {
            #[cfg(debug_assertions)]
            {
                match board.get_figure(latest_move.to) {
                    None => panic!(
                        "to field mustn't be empty after move {} leading to board: {}",
                        latest_move, board
                    ),
                    Some(attacker) => {
                        let king_color = color;
                        assert_ne!(
                            attacker.color, king_color,
                            "possible attacker should be of different color than king, but isn't: attacker pos: {}, latest_move: {}, attacker_type: {:?}, king_pos: {}, king_color: {}, board: {}",
                            latest_move.to, latest_move, attacker.fig_type, king_pos, king_color, board
                        )
                    }
                }
            }
            if gives_chess(latest_move.to, king_pos, color, board).is_some() ||
                find_attack_from_behind(latest_move.from, king_pos, color, board).is_some() {
                return true;
            }
        }
    };
    false
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Attack {
    OnLine(Direction, usize),
    ByPawn(Position),
    ByKnight(Position),
}

pub fn gives_chess(attacker_pos: Position, king_pos: Position, king_color: Color, board: &Board) -> Option<Attack> {
    #[cfg(debug_assertions)]
    {
        match board.get_figure(attacker_pos) {
            None => panic!(
                "attacker_pos shouldn't be empty: {}, king_pos: {}, king_color: {}, board: {}",
                attacker_pos, king_pos, king_color, board
            ),
            Some(attacker) => {
                assert_ne!(
                    attacker.color, king_color,
                    "attacker should be of different color than king, but isn't: {}, attacker_type: {:?}, king_pos: {}, king_color: {}, board: {}",
                    attacker_pos, attacker.fig_type, king_pos, king_color, board
                )
            }
        }
    }
    let attacker_type = board.get_figure(attacker_pos).unwrap().fig_type;
    match king_pos.get_direction(attacker_pos) {
        None => {
            if attacker_type == FigureType::Knight && attacker_pos.is_reachable_by_knight(king_pos) {
                Some(Attack::ByKnight(attacker_pos))
            } else {
                None
            }
        }
        Some(direction) => {
            match attacker_type {
                FigureType::Rook => {
                    if direction.is_straight() && board.are_intermediate_pos_free(
                        king_pos,
                        direction,
                        attacker_pos,
                    ) {
                        Some(get_queen_or_rook_attack(king_pos, direction, attacker_pos))
                    } else {
                        None
                    }
                },
                FigureType::Bishop => {
                    if direction.is_diagonal() && board.are_intermediate_pos_free(
                        king_pos,
                        direction,
                        attacker_pos,
                    ) {
                        Some(get_bishop_attack(king_pos, direction, attacker_pos))
                    } else {
                        None
                    }
                },
                FigureType::Queen => {
                    if board.are_intermediate_pos_free(
                        king_pos,
                        direction,
                        attacker_pos,
                    ) {
                        Some(get_queen_or_rook_attack(king_pos, direction, attacker_pos))
                    } else {
                        None
                    }
                },
                FigureType::Pawn => {
                    if (attacker_pos.column-king_pos.column).abs()==1 && {
                        let (forward_left, _, forward_right) = Direction::forward_directions(king_color);
                        direction== forward_left ||direction== forward_right
                    } {
                        Some(Attack::ByPawn(attacker_pos))
                    } else {
                        None
                    }
                },
                _ => {
                    None
                }
            }
        }
    }
}

#[allow(clippy::needless_return)]
pub fn find_attack_from_behind(freed_up_pos: Position, king_pos: Position, king_color: Color, board: &Board) -> Option<Attack> {
    match king_pos.get_direction(freed_up_pos) {
        None => {return None}
        Some(direction) => {
            let mut pos = freed_up_pos;
            loop {
                if let Some(new_pos) = pos.step(direction) {
                    pos = new_pos;
                } else {
                    return None;
                }
                if let Some(figure) = board.get_figure(pos) {
                    return if figure.color != king_color {
                        match figure.fig_type {
                            FigureType::Rook => {
                                if direction.is_straight() {
                                    Some(get_queen_or_rook_attack(king_pos, direction, pos))
                                } else {
                                    None
                                }
                            }
                            FigureType::Bishop => {
                                if direction.is_diagonal() {
                                    Some(get_bishop_attack(king_pos, direction, pos))
                                } else {
                                    None
                                }
                            }
                            FigureType::Queen => {
                                Some(get_queen_or_rook_attack(king_pos, direction, pos))
                            }
                            _ => { None }
                        }
                    } else {
                        None
                    }
                }
            }
        }
    };
}

fn get_queen_or_rook_attack(king_pos: Position, direction: Direction, attacker_pos: Position) -> Attack {
    Attack::OnLine(
        direction,
        max(
            (king_pos.row - attacker_pos.row).abs(),
            (king_pos.column - attacker_pos.column).abs(),
        ) as usize
    )
}

#[allow(clippy::cast_abs_to_unsigned)]
fn get_bishop_attack(king_pos: Position, direction: Direction, attacker_pos: Position) -> Attack {
    Attack::OnLine(
        direction,
        (king_pos.row - attacker_pos.row).abs() as usize
    )
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::game::{GameState};

    //♔♕♗♘♖♙♚♛♝♞♜♟
    #[rstest(
    color, game_state_config, king_pos_config, expected_is_check,
    case(Color::Black, "black ♔b6 ♙a7 ♚a8", "a8", false),
    case(Color::White, "white ♔h8 ♚f8 ♜e7 ♟e6 ♟d7", "h8", false),
    case(Color::White, "white ♔g3 ♖d1 ♚g1 ♙c2 ♙d3", "g3", false),
    case(Color::Black, "black ♔g3 ♖d1 ♚g1 ♙c2 ♙d3", "g1", true),
    case(Color::Black, "black ♔g3 ♘e2 ♚g1 ♙c2 ♙d3", "g1", true),
    case(Color::Black, "black ♔g3 ♗e3 ♚g1 ♙c2 ♙d3", "g1", true),
    case(Color::Black, "black ♔a1 ♚e4 ♙d3", "e4", true),
    case(Color::Black, "black ♔a1 ♚c4 ♙d3", "c4", true),
    case(Color::Black, "black ♔a1 ♚e2 ♙d3", "e2", false),
    case(Color::Black, "black ♔a1 ♚c2 ♙d3", "c2", false),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_is_king_in_check(
        color: Color,
        game_state_config: &str,
        king_pos_config: &str,
        expected_is_check: bool,
    ) {
        let game_state = game_state_config.parse::<GameState>().unwrap();
        let king_pos = king_pos_config.parse::<Position>().unwrap();

        let actual_in_check = is_king_in_check(king_pos, color, &game_state.board);
        assert_eq!(actual_in_check, expected_is_check);
    }

    //♔♕♗♘♖♙♚♛♝♞♜♟
    #[rstest(
    latest_move_config, color, game_state_config, king_pos_config, expected_is_check,
    case("e1Cc1", Color::Black, "black ♔c1 ♖d1 ♚e8", "e8", false),
    case("e1Cc1", Color::Black, "black ♔c1 ♖d1 ♞d8 ♚e8", "e8", false),
    case("e8Cc8", Color::White, "white ♔e1 ♜d8 ♚c8", "e1", false),
    case("e8Cc8", Color::White, "white ♘d1 ♔e1 ♜d8 ♚c8", "e1", false),
    case("e8Cc8", Color::White, "white ♔d1 ♜d8 ♚c8", "d1", true),
    case("e8cg8", Color::White, "white ♔e1 ♜f8 ♚g8", "e1", false),
    case("e8cg8", Color::White, "white ♘f1 ♔e1 ♜f8 ♚g8", "e1", false),
    case("e8cg8", Color::White, "white ♔f1 ♜f8 ♚g8", "f1", true),
    case("a4eb3", Color::White, "white ♔e1 ♟b3 ♝a5 ♚e8", "e1", true),
    case("a5eb6", Color::Black, "black ♔e1 ♙b6 ♗a4 ♚e8", "e8", true),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_is_king_in_check_after(
        latest_move_config: &str,
        color: Color,
        game_state_config: &str,
        king_pos_config: &str,
        expected_is_check: bool,
    ) {
        let latest_move = latest_move_config.parse::<Move>().unwrap();
        let game_state = game_state_config.parse::<GameState>().unwrap();
        let king_pos = king_pos_config.parse::<Position>().unwrap();

        let actual_in_check = is_king_in_check_after(latest_move, king_pos, color, &game_state.board);
        assert_eq!(actual_in_check, expected_is_check);
    }

    //♔♕♗♘♖♙♚♛♝♞♜♟
    #[rstest(
    attacker_pos_str, game_state_config, king_pos_str, expected_opt_attack,
    case("a7", "black ♔b3 ♙a7 ♚b8", "b8", Some(Attack::ByPawn("a7".parse::<Position>().unwrap()))),
    case("a6", "black ♔b3 ♘a6 ♚b8", "b8", Some(Attack::ByKnight("a6".parse::<Position>().unwrap()))),
    case("a8", "black ♔b3 ♖a8 ♚b8", "b8", Some(Attack::OnLine(Direction::Left, 1))),
    case("d6", "black ♔b3 ♕d6 ♚b8", "b8", Some(Attack::OnLine(Direction::DownRight, 2))),
    case("e5", "black ♔b3 ♗e5 ♚b8", "b8", Some(Attack::OnLine(Direction::DownRight, 3))),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_gives_check(
        attacker_pos_str: &str,
        game_state_config: &str,
        king_pos_str: &str,
        expected_opt_attack: Option<Attack>,
    ) {
        let attacker_pos = attacker_pos_str.parse::<Position>().unwrap();
        let game_state = game_state_config.parse::<GameState>().unwrap();
        let king_pos = king_pos_str.parse::<Position>().unwrap();

        let actual_opt_attack = gives_chess(attacker_pos, king_pos, game_state.turn_by, &game_state.board);
        assert_eq!(actual_opt_attack, expected_opt_attack);
    }
}