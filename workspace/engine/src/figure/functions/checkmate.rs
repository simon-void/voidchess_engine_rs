use crate::base::*;
use crate::game::{Board, FieldContent, GameState};
use crate::figure::FigureType;
use crate::figure::functions::check_search::{is_king_in_check, Attack, gives_chess, find_attack_from_behind};
use crate::base::direction::{Direction, ALL_DIRECTIONS};
use tinyvec::ArrayVec;

/**
 * it's assumed that the passive king isn't in check at this point (because then the game should already by over).
 * this also means that the king
 */
pub fn is_active_king_checkmate(king_pos: Position, king_color: Color, game_state: &GameState, after_move: Move) -> bool {
    let attack_situation = get_attack_situation(king_pos, king_color, game_state, after_move);

    match attack_situation {
        AttackerNumber::Zero => {false}
        AttackerNumber::One(attack) => {is_active_king_checkmate_from_attack(attack, king_pos, king_color, game_state)}
        AttackerNumber::Two => {can_king_move_without_being_in_check(king_pos, king_color, &game_state.board)}
    }
}

fn get_attack_situation(king_pos: Position, king_color: Color, game_state: &GameState, after_move: Move) -> AttackerNumber {
    match after_move.move_type {
        MoveType::Castling(castling_type) => {
            let rock_row = if game_state.turn_by==Color::White {
                7
            } else {
                0
            };
            let castling_rook_end_pos = if castling_type == CastlingType::KingSide {
                Position::new_unchecked(5, rock_row)
            } else {
                Position::new_unchecked(3, rock_row)
            };
            if let Some(attack) = gives_chess(castling_rook_end_pos, king_pos, king_color, &game_state.board) {
                AttackerNumber::One(attack)
            } else {
                AttackerNumber::Zero
            }
        }
        MoveType::EnPassant => {
            let is_check_from_end_pos = gives_chess(after_move.to, king_pos, king_color, &game_state.board);
            let is_check_from_behind_start_pos = find_attack_from_behind(after_move.from, king_pos, king_color, &game_state.board);
            match is_check_from_end_pos {
                None => {
                    let taken_pawn_pos: Position = Position::new_unchecked(after_move.to.column, after_move.from.row);
                    let is_check_from_behind_taken_pawn = find_attack_from_behind(taken_pawn_pos, king_pos, king_color, &game_state.board);
                    AttackerNumber::from_two_possibilities(is_check_from_behind_start_pos, is_check_from_behind_taken_pawn)
                }
                Some(_) => {
                    AttackerNumber::from_two_possibilities(is_check_from_behind_start_pos, is_check_from_end_pos)
                }
            }
        }
        _ => {
            let is_check_from_end_pos = gives_chess(after_move.to, king_pos, king_color, &game_state.board);
            let is_check_from_behind_start_pos = find_attack_from_behind(after_move.from, king_pos, king_color, &game_state.board);
            AttackerNumber::from_two_possibilities(is_check_from_end_pos, is_check_from_behind_start_pos)
        }
    }
}

fn is_active_king_checkmate_from_attack(attack: Attack, king_pos: Position, king_color: Color, game_state: &GameState) -> bool {
    if can_king_move_without_being_in_check(king_pos, king_color, &game_state.board) {
        return  false;
    }
    let bound_positions = get_bound_positions(king_pos, king_color, &game_state.board);

    for opt_defender in game_state.board.get_all_figures_of_color(king_color).iter() {
        if let Some((defender, defender_pos)) = opt_defender {
            if *defender_pos==king_pos || bound_positions.contains(defender_pos){
                continue;
            }
            if can_intercept(attack, defender.fig_type, *defender_pos, king_pos, game_state) {
                return false;
            }
        } else {
            break;
        }
    }
    true
}

fn can_intercept(attack: Attack, defender_type: FigureType, defender_pos: Position, king_pos: Position, game_state: &GameState) -> bool {
    match attack {
        Attack::ByPawn(pawn_pos) => {is_reachable(defender_type, defender_pos, pawn_pos, game_state)}
        Attack::ByKnight(knight_pos) => {is_reachable(defender_type, defender_pos, knight_pos, game_state)}
        Attack::OnLine(direction, number_of_pos) => {
            debug_assert!(number_of_pos!=0, "number_of_pos has to be at least 1, but was 0");
            let mut counter = number_of_pos;
            let mut intercept_pos = king_pos;
            loop {
                intercept_pos = intercept_pos.step_unchecked(direction);
                if is_reachable(defender_type, defender_pos, intercept_pos, game_state) {
                    return true;
                }
                counter -= 1;
                if counter==0 {
                    break
                }
            }
            false
        }
    }
}

fn get_bound_positions(king_pos: Position, king_color: Color, board: &Board) -> ArrayVec<[Position; 8]> {

    fn contains_bound_figure(direction: Direction, king_pos: Position, king_color: Color, board: &Board) -> Option<Position> {
        let mut opt_bound_position: Option<Position> = None;
        let mut pos = king_pos;
        loop {
            pos = match pos.step(direction) {
                None => {break;}
                Some(new_pos) => {new_pos}
            };
            if let Some(figure) = board.get_figure(pos) {
                if figure.color==king_color {
                    if opt_bound_position.is_none() {
                        opt_bound_position = Some(pos);
                    } else {
                        return None;
                    }
                } else {
                    return match opt_bound_position {
                        None => {None}
                        Some(maybe_bound_pos) => {
                            match figure.fig_type {
                                FigureType::Queen => {Some(maybe_bound_pos)}
                                FigureType::Rook => {
                                    if direction.is_straight() {Some(maybe_bound_pos)} else {None}
                                }
                                FigureType::Knight => {
                                    if direction.is_diagonal() {Some(maybe_bound_pos)} else {None}
                                }
                                _ => {None}
                            }
                        }
                    }
                }
            }
        }
        None
    }

    let mut bound_positions = ArrayVec::<[Position; 8]>::default();
    for direction in ALL_DIRECTIONS.iter() {
        if let Some(bound_pos) = contains_bound_figure(*direction, king_pos, king_color, board) {
            bound_positions.push(bound_pos)
        }
    }
    bound_positions
}

fn can_king_move_without_being_in_check(king_pos: Position, king_color: Color, board: &Board) -> bool {
    let board_without_king = {
        let mut cloned_board = board.clone();
        cloned_board.clear_field(king_pos);
        cloned_board
    };
    for direction in ALL_DIRECTIONS.iter() {
        if let Some(new_king_pos) = king_pos.step(*direction) {
            let field_content = board.get_content_type(new_king_pos, king_color);
            if field_content != FieldContent::OwnFigure && !is_king_in_check(new_king_pos, king_color, &board_without_king) {
                return true
            }
        }
    }
    false
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum AttackerNumber {
    Zero,
    One(Attack),
    Two,
}

impl AttackerNumber {
    fn from_two_possibilities(opt_attack1: Option<Attack>, opt_attack2: Option<Attack>) -> AttackerNumber {
        match opt_attack1 {
            None => {
                match opt_attack2 {
                    None => {
                        AttackerNumber::Zero
                    }
                    Some(attack2) => {
                        AttackerNumber::One(attack2)
                    }
                }
            }
            Some(attack1) => {
                match opt_attack2 {
                    None => {
                        AttackerNumber::One(attack1)
                    }
                    Some(_) => {
                        AttackerNumber::Two
                    }
                }
            }
        }
    }
}

/**
 * it is guaranteed that to_pos is either free or a figure of opposite color
 */
fn is_reachable(
    fig_type: FigureType,
    fig_pos: Position,
    to_pos: Position,
    game_state: &GameState,
) -> bool {
    match fig_type {
        FigureType::Pawn => is_reachable_by_pawn(
            fig_pos,
            to_pos,
            game_state,
        ),
        FigureType::Rook => is_reachable_by_rook(
            fig_pos,
            to_pos,
            &game_state.board,
        ),
        FigureType::Knight => is_reachable_by_knight(
            fig_pos,
            to_pos,
        ),
        FigureType::Bishop => is_reachable_by_bishop(
            fig_pos,
            to_pos,
            &game_state.board,
        ),
        FigureType::Queen => is_reachable_by_queen(
            fig_pos,
            to_pos,
            &game_state.board,
        ),
        FigureType::King => {
            panic!("is_reachable isn't implemented for ")
        },
    }
}

fn is_reachable_by_pawn(
    pawn_pos: Position,
    to_pos: Position,
    game_state: &GameState,
) -> bool {
    let pawn_is_white = game_state.turn_by==Color::White;
    let row_step = if pawn_is_white {
        1
    } else {
        -1
    };
    let column_diff = (pawn_pos.column-to_pos.column).abs();
    if column_diff > 1 {
        return false;
    }
    if column_diff==1 {
        if (to_pos.row - pawn_pos.row) != row_step {
            return false;
        }
        if game_state.board.get_content_type(to_pos, game_state.turn_by) == FieldContent::OpponentFigure {
            return true;
        }
        if let Some(en_passant_pos) = game_state.en_passant_intercept_pos {
            to_pos==en_passant_pos
        } else {
            false
        }
    } else {
        let one_step_forward_row = pawn_pos.row + row_step;
        if to_pos.row==one_step_forward_row {
            game_state.board.is_empty(to_pos)
        } else {
            let two_steps_forward_row = one_step_forward_row + row_step;
            let start_row: i8 = if pawn_is_white {1} else {6};
            to_pos.row==two_steps_forward_row &&
                pawn_pos.row==start_row &&
                game_state.board.is_empty(to_pos) &&
                game_state.board.is_empty(Position::new_unchecked(pawn_pos.column, one_step_forward_row))

        }
    }
}

fn is_reachable_by_rook(
    rook_pos: Position,
    to_pos: Position,
    board: &Board,
) -> bool {
    if let Some(direction) = rook_pos.get_direction(to_pos) {
        direction.is_straight() && board.are_intermediate_pos_free(rook_pos, direction, to_pos)
    } else {
        false
    }
}

fn is_reachable_by_knight(
    knight_pos: Position,
    to_pos: Position,
) -> bool {
    knight_pos.is_reachable_by_knight(to_pos)
}

fn is_reachable_by_bishop(
    bishop_pos: Position,
    to_pos: Position,
    board: &Board,
) -> bool {
    if let Some(direction) = bishop_pos.get_direction(to_pos) {
        direction.is_diagonal() && board.are_intermediate_pos_free(bishop_pos, direction, to_pos)
    } else {
        false
    }
}

fn is_reachable_by_queen(
    queen_pos: Position,
    to_pos: Position,
    board: &Board,
) -> bool {
    if let Some(direction) = queen_pos.get_direction(to_pos) {
        board.are_intermediate_pos_free(queen_pos, direction, to_pos)
    } else {
        false
    }
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::game::{GameState};

    //♔♕♗♘♖♙♚♛♝♞♜♟
    #[rstest(
    game_state_config, move_config, expected_is_mate,
    case("e2-e4 e7-e5 f1-c4 d7-d6 d1-f3 c7-c5", "f3-f7", true),
    case("e2-e4 f7-f5 e4-f5 g7-g5", "d1-h5", true),
    case("e2-e4 f7-f5", "d1-h5", false),
    case("white ♔e1 ♖a7 ♖b7 ♚e8", "a7-a8", true),
    case("black ♔f1 ♖e1 ♖g1 ♙e2 ♙g2 ♚e8 ♜h8", "e8cg8", true),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_is_active_king_checkmate(
        game_state_config: &str,
        move_config: &str,
        expected_is_mate: bool,
    ) {
        // pub fn is_active_king_checkmate(king_pos: Position, king_color: Color, game_state: &GameState, after_move: Move) -> bool {
        let latest_move = move_config.parse::<Move>().unwrap();
        let (game_state, _) = game_state_config.parse::<GameState>().unwrap().do_move(latest_move);
        let king_pos = game_state.get_active_king();
        let color = game_state.turn_by;

        let actual_is_mate = is_active_king_checkmate(king_pos, color, &game_state, latest_move);
        assert_eq!(actual_is_mate, expected_is_mate);

        let actual_is_mate_toggled = is_active_king_checkmate(king_pos.toggle_row(), color.toggle(), &game_state.toggle_colors(), latest_move.toggle_rows());
        assert_eq!(actual_is_mate_toggled, expected_is_mate);
    }

    //♔♕♗♘♖♙♚♛♝♞♜♟
    #[rstest(
    game_state_config, move_config, expected_attack_situation,
    case("e2-e4 e7-e5", "d1-h5", AttackerNumber::Zero),
    case("e2-e4 f7-f5", "d1-h5", AttackerNumber::One(Attack::OnLine(Direction::DownRight, 3))),
    case("e2-e4 e7-e5 f1-c4 d7-d6 d1-f3 c7-c5", "f3-f7", AttackerNumber::One(Attack::OnLine(Direction::DownRight, 1))),
    case("white ♔e1 ♖a7 ♖b7 ♚e8", "a7-a8", AttackerNumber::One(Attack::OnLine(Direction::Left, 4))),
    case("white ♔e1 ♕a1 ♚b8", "a1-a8", AttackerNumber::One(Attack::OnLine(Direction::Left, 1))),
    case("white ♔e1 ♕a1 ♚c8", "a1-a8", AttackerNumber::One(Attack::OnLine(Direction::Left, 2))),
    case("white ♔e1 ♕a1 ♚b8", "a1-a7", AttackerNumber::One(Attack::OnLine(Direction::DownLeft, 1))),
    case("black ♔f1 ♖e1 ♖g1 ♙e2 ♙g2 ♚e8 ♜h8", "e8cg8", AttackerNumber::One(Attack::OnLine(Direction::Up, 7))),
    case("black ♔f1 Eb3 ♙b4 ♟c4 ♚e8 ♝a6", "c4eb3", AttackerNumber::One(Attack::OnLine(Direction::UpLeft, 5))),
    case("black ♔c2 Eb3 ♙b4 ♟c4 ♚e8", "c4eb3", AttackerNumber::One(Attack::ByPawn("b3".parse::<Position>().unwrap()))),
    case("black ♔e1 ♝a2 ♞a1 ♚e8", "a1-c2", AttackerNumber::One(Attack::ByKnight("c2".parse::<Position>().unwrap()))),
    case("black ♔f1 ♜a1 ♞b1 ♚e8", "b1-d2", AttackerNumber::Two),
    case("black ♔f1 ♜a1 ♝b1 ♚e8", "b1-d3", AttackerNumber::Two),
    case("black ♔c2 Eb3 ♙b4 ♟c4 ♜c5 ♚e8", "c4eb3", AttackerNumber::Two),
    case("black ♔c3 Eb3 ♙b4 ♟c4 ♜c5 ♝a5 ♚e8", "c4eb3", AttackerNumber::Two),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_attack_situation(
        game_state_config: &str,
        move_config: &str,
        expected_attack_situation: AttackerNumber,
    ) {
        // pub fn is_active_king_checkmate(king_pos: Position, king_color: Color, game_state: &GameState, after_move: Move) -> bool {
        let latest_move = move_config.parse::<Move>().unwrap();
        let (game_state, _) = game_state_config.parse::<GameState>().unwrap().do_move(latest_move);
        let king_pos = game_state.get_active_king();
        let color = game_state.turn_by;

        let actual_attack_situation = get_attack_situation(king_pos, color, &game_state, latest_move);
        assert_eq!(actual_attack_situation, expected_attack_situation);
    }
}

