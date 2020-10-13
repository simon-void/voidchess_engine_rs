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
    // fields between rook and king have to be free
    for column in (1 as i8)..=3 {
        if !board.is_empty(Position::new_unchecked(column, king_pos.row)) {
            return None;
        }
    }

    // king can't be in check from forward, or diagonal direction on from king start to king end pos
    for column in (2 as i8)..=4 {
        if is_king_is_attackable(Position::new_unchecked(column, king_pos.row), color, board) {
            return None;
        }
    }

    // king is not allowed to be attackable on his start field from the side
    if is_king_straight_attackable(king_pos, color, Direction::Right, board) {
        return None;
    }
    Some(Position::new_unchecked(2, king_pos.row))
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
    // fields between rook and king have to be free
    for column in (5 as i8)..=6 {
        if !board.is_empty(Position::new_unchecked(column, king_pos.row)) {
            return None;
        }
    }

    // king can't be in check from forward, or diagonal direction on from king start to king end pos
    for column in (4 as i8)..=6 {
        if is_king_is_attackable(Position::new_unchecked(column, king_pos.row), color, board) {
            return None;
        }
    }

    // king is not allowed to be attackable on his start field from the side
    if is_king_straight_attackable(king_pos, color, Direction::Left, board) {
        return None;
    }
    Some(Position::new_unchecked(6, king_pos.row))
}

fn is_king_is_attackable(king_pos: Position, color: Color, board: &Board) -> bool {
    let (forward_left, forward, forward_right) = Direction::forward_directions(color);
    if is_king_straight_attackable(king_pos, color, forward, board) {
        return true;
    }
    if [forward_left, forward_right].iter().any(|diagonal| { is_king_diagonal_attackable(king_pos, color, *diagonal, board) }) {
        return true;
    }
    is_king_attackable_by_knight(king_pos, color, board)
}

fn is_king_straight_attackable(king_pos: Position, color: Color, forward: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, forward, board) {
        return match opponent_fig_type {
            FigureType::Rook | FigureType::Queen => true,
            FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

fn is_king_diagonal_attackable(king_pos: Position, color: Color, diagonal: Direction, board: &Board) -> bool {
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

fn is_king_attackable_by_knight(king_pos: Position, color: Color, board: &Board) -> bool {
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

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::game::{WHITE_KING_STARTING_POS, GameState, BLACK_KING_STARTING_POS};

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing_white, expected_castling_is_allowed,
    case("white ♖a1 ♔e1 ♚e8", true),
    case("white ♖a1 ♔e1 ♚c2", false),
    case("white ♖a1 ♔e1 ♚d2", false),
    case("white ♖a1 ♔e1 ♚e2", false),
    case("white ♖a1 ♔e1 ♚f2", false),
    case("white ♖a1 ♔e1 ♚f1", false),
    case("white ♖a1 ♔e1 ♚b3", true),
    case("white ♖a1 ♔e1 ♚c3", true),
    case("white ♖a1 ♔e1 ♚d3", true),
    case("white ♖a1 ♔e1 ♚e3", true),
    case("white ♖a1 ♔e1 ♚f3", true),
    case("white ♖a1 ♔e1 ♟a2 ♚e8", true),
    case("white ♖a1 ♔e1 ♟b2 ♚e8", false),
    case("white ♖a1 ♔e1 ♟c2 ♚e8", false),
    case("white ♖a1 ♔e1 ♟d2 ♚e8", false),
    case("white ♖a1 ♔e1 ♟e2 ♚e8", false),
    case("white ♖a1 ♔e1 ♟f2 ♚e8", false),
    case("white ♖a1 ♔e1 ♟g2 ♚e8", true),
    case("white ♖a1 ♔e1 ♟a3 ♚e8", true),
    case("white ♖a1 ♔e1 ♟b3 ♚e8", true),
    case("white ♖a1 ♔e1 ♟c3 ♚e8", true),
    case("white ♖a1 ♔e1 ♟d3 ♚e8", true),
    case("white ♖a1 ♔e1 ♟e3 ♚e8", true),
    case("white ♖a1 ♔e1 ♟f3 ♚e8", true),
    case("white ♖a1 ♔e1 ♟g3 ♚e8", true),
    case("white ♖a1 ♔e1 ♗f1 ♚e8", true),
    case("white ♖a1 ♗b1 ♔e1 ♚e8", false),
    case("white ♖a1 ♗c1 ♔e1 ♚e8", false),
    case("white ♖a1 ♗d1 ♔e1 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a2 ♚e8", true),
    case("white ♖a1 ♔e1 ♝a3 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a4 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a5 ♚e8", false),
    case("white ♖a1 ♔e1 ♝a6 ♚e8", true),
    case("white ♖a1 ♔e1 ♛a2 ♚e8", true),
    case("white ♖a1 ♔e1 ♛a3 ♚e8", false),
    case("white ♖a1 ♔e1 ♛a4 ♚e8", false),
    case("white ♖a1 ♔e1 ♛a5 ♚e8", false),
    case("white ♖a1 ♔e1 ♛a6 ♚e8", true),
    case("white ♖a1 ♔e1 ♛b7 ♚e8", true),
    case("white ♖a1 ♔e1 ♛c7 ♚e8", false),
    case("white ♖a1 ♔e1 ♛d7 ♚e8", false),
    case("white ♖a1 ♔e1 ♛e7 ♚e8", false),
    case("white ♖a1 ♔e1 ♛f7 ♚e8", true),
    case("white ♖a1 ♔e1 ♛h1 ♚e8", false),
    case("white ♖a1 ♔e1 ♜h1 ♚e8", false),
    case("white ♖a1 ♔e1 ♜a7 ♚e8", true),
    case("white ♖a1 ♔e1 ♜b7 ♚e8", true),
    case("white ♖a1 ♔e1 ♜c7 ♚e8", false),
    case("white ♖a1 ♔e1 ♜d7 ♚e8", false),
    case("white ♖a1 ♔e1 ♜e7 ♚e8", false),
    case("white ♖a1 ♔e1 ♜f7 ♚e8", true),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_is_queen_side_castling_allowed_in_classical_config(
        game_config_testing_white: &str,
        expected_castling_is_allowed: bool,
    ) {
        let game_state = game_config_testing_white.parse::<GameState>().unwrap();
        let white_castling_is_allowed = is_queen_side_castling_allowed(Color::White, WHITE_KING_STARTING_POS, &game_state.board).is_some();
        assert_eq!(white_castling_is_allowed, expected_castling_is_allowed, "testing: {}", game_config_testing_white);

        let black_castling_is_allowed = is_queen_side_castling_allowed(Color::Black, BLACK_KING_STARTING_POS, &game_state.toggle_colors().board).is_some();
        assert_eq!(black_castling_is_allowed, expected_castling_is_allowed, "testing inverted of: {}", game_config_testing_white);
    }

    #[rstest(
    game_config_testing_white, expected_castling_is_allowed,
    case("white ♔e1 ♖h1 ♚e8", true),
    case("white ♔e1 ♖h1 ♚d2", false),
    case("white ♔e1 ♖h1 ♚e2", false),
    case("white ♔e1 ♖h1 ♚f2", false),
    case("white ♔e1 ♖h1 ♚g2", false),
    case("white ♔e1 ♖h1 ♚h2", false),
    case("white ♔e1 ♖h1 ♚d1", false),
    case("white ♔e1 ♖h1 ♚d3", true),
    case("white ♔e1 ♖h1 ♚e3", true),
    case("white ♔e1 ♖h1 ♚f3", true),
    case("white ♔e1 ♖h1 ♚g3", true),
    case("white ♔e1 ♖h1 ♚h3", true),
    case("white ♔e1 ♖h1 ♟d2 ♚e8", false),
    case("white ♔e1 ♖h1 ♟e2 ♚e8", false),
    case("white ♔e1 ♖h1 ♟f2 ♚e8", false),
    case("white ♔e1 ♖h1 ♟g2 ♚e8", false),
    case("white ♔e1 ♖h1 ♟h2 ♚e8", false),
    case("white ♔e1 ♖h1 ♟d3 ♚e8", true),
    case("white ♔e1 ♖h1 ♟e3 ♚e8", true),
    case("white ♔e1 ♖h1 ♟f3 ♚e8", true),
    case("white ♔e1 ♖h1 ♟g3 ♚e8", true),
    case("white ♔e1 ♖h1 ♟h3 ♚e8", true),
    case("white ♔e1 ♖h1 ♗c1 ♚e8", true),
    case("white ♔e1 ♖h1 ♗f1 ♚e8", false),
    case("white ♔e1 ♖h1 ♗g1 ♚e8", false),
    case("white ♔e1 ♖h1 ♝h2 ♚e8", false),
    case("white ♔e1 ♖h1 ♝h3 ♚e8", false),
    case("white ♔e1 ♖h1 ♝h4 ♚e8", false),
    case("white ♔e1 ♖h1 ♝h5 ♚e8", true),
    case("white ♔e1 ♖h1 ♝d3 ♚e8", false),
    case("white ♔e1 ♖h1 ♛h2 ♚e8", false),
    case("white ♔e1 ♖h1 ♛h3 ♚e8", false),
    case("white ♔e1 ♖h1 ♛h4 ♚e8", false),
    case("white ♔e1 ♖h1 ♛h5 ♚e8", true),
    case("white ♔e1 ♖h1 ♛d3 ♚e8", false),
    case("white ♔e1 ♖h1 ♛d7 ♚e8", true),
    case("white ♔e1 ♖h1 ♛e7 ♚e8", false),
    case("white ♔e1 ♖h1 ♛f7 ♚e8", false),
    case("white ♔e1 ♖h1 ♛g7 ♚e8", false),
    case("white ♔e1 ♖h1 ♛h7 ♚e8", true),
    case("white ♔e1 ♖h1 ♛a1 ♚e8", false),
    case("white ♔e1 ♖h1 ♜d7 ♚e8", true),
    case("white ♔e1 ♖h1 ♜e7 ♚e8", false),
    case("white ♔e1 ♖h1 ♜f7 ♚e8", false),
    case("white ♔e1 ♖h1 ♜g7 ♚e8", false),
    case("white ♔e1 ♖h1 ♜h7 ♚e8", true),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_is_king_side_castling_allowed_in_classical_config(
        game_config_testing_white: &str,
        expected_castling_is_allowed: bool,
    ) {
        let game_state = game_config_testing_white.parse::<GameState>().unwrap();
        let white_castling_is_allowed = is_king_side_castling_allowed(Color::White, WHITE_KING_STARTING_POS, &game_state.board).is_some();
        assert_eq!(white_castling_is_allowed, expected_castling_is_allowed, "testing: {}", game_config_testing_white);

        let black_castling_is_allowed = is_king_side_castling_allowed(Color::Black, BLACK_KING_STARTING_POS, &game_state.toggle_colors().board).is_some();
        assert_eq!(black_castling_is_allowed, expected_castling_is_allowed, "testing inverted of: {}", game_config_testing_white);
    }
}