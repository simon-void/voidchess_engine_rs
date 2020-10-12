use crate::base::{Color, Position, Direction};
use crate::game::Board;
use crate::figure::FigureType;

/*
 * it is assumed that king and the respective rook haven't moved yet
 * and that the game is played in classical chess.
 */
pub fn is_queen_side_castling_allowed(
    color: Color,
    king_pos: Position,
    board: &Board,
) -> Option<Position> {
    let positions_between_king_and_rook = [Position::new_unchecked(1, king_pos.row), Position::new_unchecked(2, king_pos.row), Position::new_unchecked(3, king_pos.row)];
    if positions_between_king_and_rook.iter().any(|&pos|{!board.is_empty(pos)}) {
        return None
    }
    if let Some((opponent_fig_type, _)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, Direction::Right, board) {
        if opponent_fig_type==FigureType::Rook || opponent_fig_type==FigureType::Queen {
            return None
        }
    }
    let d_column_pos = king_pos.step_unchecked(Direction::Left);
    if is_king_is_interceptable_on(king_pos, color, board) || is_king_is_interceptable_on(d_column_pos, color, board) {
        return None
    }
    Some(d_column_pos.step_unchecked(Direction::Left))
}


/*
 * it is assumed that king and the respective rook haven't moved yet
 * and that the game is played in classical chess.
 */
pub fn is_king_side_castling_allowed(
    color: Color,
    king_pos: Position,
    board: &Board,
) -> Option<Position> {
    let positions_between_king_and_rook = [Position::new_unchecked(5, king_pos.row), Position::new_unchecked(6, king_pos.row)];
    if positions_between_king_and_rook.iter().any(|&pos|{!board.is_empty(pos)}) {
        return None
    }
    if let Some((opponent_fig_type, _)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, Direction::Left, board) {
        if opponent_fig_type==FigureType::Rook || opponent_fig_type==FigureType::Queen {
            return None
        }
    }
    let f_column_pos = king_pos.step_unchecked(Direction::Right);
    if is_king_is_interceptable_on(king_pos, color, board) || is_king_is_interceptable_on(f_column_pos, color, board) {
        return None
    }
    Some(f_column_pos.step_unchecked(Direction::Right))
}

fn is_king_is_interceptable_on(king_pos: Position, color: Color, board: &Board) -> bool {
    let (forward_left, forward, forward_right) = Direction::forward_directions(color);
    if let Some((opponent_fig_type, _)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, forward, board) {
        if opponent_fig_type==FigureType::Rook || opponent_fig_type==FigureType::Queen {
            return true
        }
    }
    let can_be_intercepted_on_diagonal = [forward_left, forward_right].iter().any(|diagonal_direction| {
        if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, *diagonal_direction, board) {
            return match opponent_fig_type {
                FigureType::Bishop | FigureType::Queen => true,
                FigureType::Pawn if is_attacker_next_to_self => true,
                _ => false,
            }
        }
        false
    });
    if can_be_intercepted_on_diagonal {
        return true
    }
    for possible_knight_attacker_pos in king_pos.reachable_knight_positions(color, board) {
        if let Some(figure) = board.get_figure(possible_knight_attacker_pos){
            // different color is already guaranteed by reachable_knight_positions
            if figure.fig_type == FigureType::Knight {
                return true
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
    board: & Board,
) -> Option<(FigureType,bool)> {
    let mut old_pos = king_pos;
    let mut is_attacker_next_to_self = true;
    loop {
        let new_pos = match old_pos.step(direction) {
            Some(pos) => {
                pos
            },
            None => {return None},
        };
        if let Some(figure) = board.get_figure(new_pos) {
            return if figure.color == king_color {
                None
            } else {
                Some((figure.fig_type, is_attacker_next_to_self))
            }
        }
        old_pos = new_pos;
        is_attacker_next_to_self = false;
    }
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::game::{WHITE_KING_STARTING_POS, GameState, BLACK_KING_STARTING_POS};
    use crate::Game;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing_white, expected_castling_is_allowed,
    case("white ♖a1 ♔e1 ♚e8", true),
    case("white ♖a1 ♔e1 ♚b2", false),
    case("white ♖a1 ♔e1 ♚c2", false),
    case("white ♖a1 ♔e1 ♚d2", false),
    case("white ♖a1 ♔e1 ♚e2", false),
    case("white ♖a1 ♔e1 ♚f2", false),
    case("white ♖a1 ♔e1 ♚f1", false),
    case("white ♖a1 ♔e1 ♟a2, ♚e8", true),
    case("white ♖a1 ♔e1 ♟a3, ♚e8", true),
    case("white ♖a1 ♔e1 ♟b2, ♚e8", false),
    case("white ♖a1 ♔e1 ♟c2, ♚e8", false),
    case("white ♖a1 ♔e1 ♟d2, ♚e8", false),
    case("white ♖a1 ♔e1 ♟e2, ♚e8", false),
    case("white ♖a1 ♔e1 ♟f2, ♚e8", false),
    case("white ♖a1 ♔e1 ♟g2, ♚e8", true),
    case("white ♖a1 ♔e1 ♗f1 ♚e8", true),
    case("white ♖a1 ♗b1 ♔e1 ♚e8", false),
    case("white ♖a1 ♗c1 ♔e1 ♚e8", false),
    case("white ♖a1 ♗d1 ♔e1 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a2 ♚e8", true),
    case("white ♖a1 ♔e1 ♝a3 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a4 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a5 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a6 ♚e8", true),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_is_queen_side_castling_allowed_in_classical_config(
        game_config_testing_white: &str,
        expected_castling_is_allowed: bool,
    ) {
        let game_state = game_config_testing_white.parse::<GameState>().unwrap();
        let white_castling_is_allowed = is_queen_side_castling_allowed(Color::White, WHITE_KING_STARTING_POS, &game_state.board).is_some();
        assert_eq!(white_castling_is_allowed, expected_castling_is_allowed, "testing: {}", game_config_testing_white);

        // let black_castling_is_allowed = is_queen_side_castling_allowed(Color::Black, BLACK_KING_STARTING_POS, &game_state.board).is_some();
        // assert_eq!(black_castling_is_allowed, expected_castling_is_allowed, "testing inverted of: {}", game_config_testing_white);
    }
}