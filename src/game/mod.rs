mod game_state;
mod board;

pub use crate::game::game_state::*;
pub use crate::game::board::*;
use crate::base::{Moves, ChessError, ErrorKind, Move, Position, MoveArray, Color};
use std::{str, fmt};
use tinyvec::*;

#[derive(Clone, Debug)]
pub struct Game {
    latest_state: GameState,
    reachable_moves: Moves,
    white_board_states_history: BoardStates,
    black_board_states_history: BoardStates,
}

impl Game {
    pub fn classic() -> Game {
        let latest_state = GameState::classic();
        Game::from_state(latest_state)
    }

    pub fn from_state(game_state: GameState) -> Game {
        let reachable_moves = game_state.get_reachable_moves();
        Game::from_state_and_reachable_moves(game_state, reachable_moves)
    }

    fn from_state_and_reachable_moves(game_state: GameState, reachable_moves: Moves) -> Game {
        let mut past_white_board_states: BoardStates = tiny_vec!();
        let mut past_black_board_states: BoardStates = tiny_vec!();
        let current_board_state = game_state.board.encode();
        match game_state.turn_by {
            Color::White => {past_white_board_states.push(current_board_state)}
            Color::Black => {past_black_board_states.push(current_board_state)}
        }
        Game {
            latest_state: game_state,
            reachable_moves,
            white_board_states_history: past_white_board_states,
            black_board_states_history: past_black_board_states,
        }
    }

    pub fn play(&self, a_move: &Move) -> MoveResult {
        let (new_game_state, move_stats) = self.latest_state.do_move(*a_move);

        let reachable_moves = match verify_game_state(&new_game_state) {
            Ok(moves) => { moves }
            Err(stoppedReason) => {
                return MoveResult::Stopped(stoppedReason, new_game_state);
            }
        };
        // since past_game_states are always empty on Game construction
        // we don't need a three-times repetition check in verify_game_state
        let (
            new_white_board_states_history,
            new_black_board_states_history,
        ) = {
            let new_board_state = new_game_state.board.encode();

            if move_stats.did_move_pawn || move_stats.did_catch_figure {
                let mut new_white_board_states = tiny_vec!();
                let mut new_black_board_states = tiny_vec!();
                match new_game_state.turn_by {
                    Color::White => { new_white_board_states.push(new_board_state); }
                    Color::Black => { new_black_board_states.push(new_board_state); }
                };
                (new_white_board_states, new_black_board_states)
            } else {
                let mut new_white_board_states = self.white_board_states_history.clone();
                let mut new_black_board_states = self.black_board_states_history.clone();

                fn push_onto_list_and_checks_if_three_fold_repetition(
                    board_state: BoardState,
                    board_states: &mut BoardStates,
                ) -> bool {

                    let is_three_fold_repetition: bool = if board_states.len() < 4 {
                        false
                    } else {
                        let mut occurrence_of_board_state: usize = 1;
                        board_states.iter().for_each(|it| {
                            if board_state.eq(it) {
                                occurrence_of_board_state += 1;
                            }
                        });
                        debug_assert!(
                            occurrence_of_board_state<4,
                            "maximum occurrence of a board state should be 3 but is {}",
                            occurrence_of_board_state
                        );
                        occurrence_of_board_state == 3
                    };
                    board_states.push(board_state);
                    is_three_fold_repetition
                }

                let is_three_fold_repetition = match new_game_state.turn_by {
                    Color::White => {
                        push_onto_list_and_checks_if_three_fold_repetition(
                            new_board_state,
                            &mut new_white_board_states,
                        )
                    }
                    Color::Black => {
                        push_onto_list_and_checks_if_three_fold_repetition(
                            new_board_state,
                            &mut new_black_board_states,
                        )
                    }
                };
                if is_three_fold_repetition {
                    return MoveResult::Stopped(StoppedReason::ThreeTimesRepetition, new_game_state);
                }
                (new_white_board_states, new_black_board_states)
            }
        };

        debug_assert!(
            {
                let white_states_count = new_white_board_states_history.len() as isize;
                let black_states_count = new_black_board_states_history.len() as isize;
                (white_states_count-black_states_count).abs() <2
            },
            "number of white({})/black({}) board_states can only differ by 1",
            new_white_board_states_history.len(),
            new_black_board_states_history.len(),
        );
        debug_assert!(
            {
                if move_stats.did_catch_figure || move_stats.did_move_pawn {
                    let white_states_count = new_white_board_states_history.len() as isize;
                    let black_states_count = new_black_board_states_history.len() as isize;
                    match new_game_state.turn_by {
                        Color::White => {
                            white_states_count == 1 && black_states_count == 0
                        }
                        Color::Black => {
                            white_states_count == 0 && black_states_count == 1
                        }
                    }
                } else {true}
            },
            "catching a figure or moving a pawn resets the board states: color {}, white# {}, black# {}",
            new_game_state.turn_by,
            new_white_board_states_history.len(),
            new_black_board_states_history.len(),
        );

        let new_game = Game {
            latest_state: new_game_state,
            reachable_moves,
            white_board_states_history: new_white_board_states_history,
            black_board_states_history: new_black_board_states_history,
        };
        let move_result = MoveResult::Ongoing(new_game, move_stats);
        move_result
    }

    pub fn get_reachable_moves(&self) -> &Moves {
        &self.reachable_moves
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.latest_state
    }

    pub fn is_passive_king_pos(&self, reachable_field: &Position) -> bool {
        *reachable_field == self.latest_state.get_passive_king_pos()
    }

    pub fn is_active_king_in_check(&self) -> bool {
        self.latest_state.is_active_king_in_check()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.latest_state)
    }
}

impl str::FromStr for Game {
    type Err = ChessError;

    fn from_str(desc: &str) -> Result<Self, Self::Err> {
        let trimmed_desc = desc.trim();
        if trimmed_desc.is_empty() {
            return Ok(Game::classic())
        }
        let token_iter = trimmed_desc.split(" ").into_iter();

        // let desc_contains_figures: bool = "♔♕♗♘♖♙♚♛♝♞♜♟".chars().any(|symbol|{desc.contains(symbol)});
        let desc_contains_moves: bool = trimmed_desc.is_empty() || trimmed_desc.contains("-");
        if desc_contains_moves {
            game_by_moves_from_start(token_iter)
        } else {
            game_by_figures_on_board(trimmed_desc)
        }
    }
}

fn game_by_figures_on_board(trimmed_game_config: &str) -> Result<Game, ChessError> {
    let game_state = trimmed_game_config.parse::<GameState>()?;
    if let Err(stoppedReason) = verify_game_state(&game_state) {
        return Err(ChessError {
            msg: format!("game_state {} failed to pass verification: {:?}", game_state, &stoppedReason),
            kind: ErrorKind::HighLevelErr(stoppedReason),
        })
    }
    Ok(Game::from_state(game_state))
}

fn game_by_moves_from_start(token_iter: str::Split<&str>) -> Result<Game, ChessError> {
    let mut game = Game::classic();
    for token in token_iter {
        let a_move = token.parse::<Move>()?;
        let move_result = game.play(&a_move);
        match move_result {
            MoveResult::Ongoing(new_game, _) => {
                game = new_game;
            }
            MoveResult::Stopped(reason, _) => {
                return Err(ChessError {
                    msg: format!("game has already ended after move {} because of {:?} in final state {}", a_move, reason, game),
                    kind: ErrorKind::HighLevelErr(reason),
                })
            }
        }
    }
    Ok(game)
}

fn verify_game_state(game_state: &GameState) -> Result<Moves, StoppedReason> {
    let passive_king_pos = game_state.get_passive_king_pos();
    let reachable_moves = game_state.get_reachable_moves();
    if reachable_moves.iter().any(|reachable_move| reachable_move.to == passive_king_pos) {
        return Err(StoppedReason::KingInCheckAfterMove);
    }
    if !game_state.board.contains_sufficient_material_to_continue() {
        return Err(StoppedReason::InsufficientMaterial);
    }
    Ok(reachable_moves)
}

#[derive(Debug)]
pub enum MoveResult {
    /*
     * bool: was figure taken
     */
    Ongoing(Game, MoveStats),
    Stopped(StoppedReason, GameState),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StoppedReason {
    KingInCheckAfterMove,
    InsufficientMaterial,
    ThreeTimesRepetition,
    NoChangeIn50Moves,
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing_white, next_move_str, expected_is_insufficient_material,
    case("white ♔e1 ♜f1 ♚e8", "e1-f1", true),
    case("white ♔e1 ♜f1 ♘b1 ♘g1 ♚e8 ♞b8 ♞g8", "e1-f1", true),
    case("white ♔e1 ♜f1 ♘b1 ♘g1 ♚e8 ♞b8 ♞g8 ♞h8", "e1-f1", false),
    case("white ♔e1 ♜f1 ♚e8 ♞b8 ♞g8 ♞h8", "e1-f1", false),
    case("white ♔e1 ♜f1 ♗b1 ♚e8 ♞b8 ♞g8", "e1-f1", true),
    case("white ♔e1 ♜f1 ♗b1 ♚e8 ♝g8", "e1-f1", true),
    case("white ♔e1 ♜f1 ♗b1 ♘g8 ♚e8", "e1-f1", false),
    case("white ♔e1 ♜f1 ♗b1 ♗g8 ♚e8", "e1-f1", false),
    case("white ♔e1 ♖b1 ♚e8", "e1-f1", false),
    case("white ♔e1 ♛b1 ♚e8", "e1-f1", false),
    case("white ♔e1 ♙b2 ♚e8", "e1-f1", false),
    case("white ♔e1 ♙a2 ♙b2 ♙c2 ♙d2 ♙e2 ♙f2 ♚e8", "e1-f1", false),
    case("white ♔a1 ♟a2 ♚a3", "a1-a2", false), // because king is in check after move
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_game_ends_bc_insufficient_material(
        game_config_testing_white: &str,
        next_move_str: &str,
        expected_is_insufficient_material: bool,
    ) {
        fn is_insufficient_material(move_result: MoveResult) -> bool {
            match move_result {
                MoveResult::Stopped(StoppedReason::InsufficientMaterial, _) => true,
                _ => false,
            }
        }

        let game = game_config_testing_white.parse::<Game>().unwrap();
        let next_move = next_move_str.parse::<Move>().unwrap();
        let move_result = game.play(&next_move);
        assert_eq!(is_insufficient_material(move_result), expected_is_insufficient_material);
    }

    #[rstest(
    game_config, expected_stop_reason,
    case("white ♔h8 ♚f8 ♞a7", StoppedReason::InsufficientMaterial),
    case("black ♔h8 ♚f8 ♛g7", StoppedReason::KingInCheckAfterMove),
    case("b1-c3 b8-c6 c3-b1 c6-b8 b1-c3 b8-c6 c3-b1 c6-b8 b1-c3", StoppedReason::ThreeTimesRepetition),
    case("b1-c3 b8-c6 c3-b1 c6-b8 b1-c3 b8-c6 c3-b1 c6-b8", StoppedReason::ThreeTimesRepetition),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_parse_stopped_game(
        game_config: &str,
        expected_stop_reason: StoppedReason,
    ) {
        // pub enum StoppedReason {
        //     KingInCheckAfterMove,
        //     InsufficientMaterial,
        //     ThreeTimesRepetition,
        //     NoChangeIn50Moves,
        // }
        let game = match game_config.parse::<Game>() {
            Err(err) => {
                if let ErrorKind::HighLevelErr(actual_stopped_reason) = err.kind {
                    assert_eq!(actual_stopped_reason, expected_stop_reason);
                } else {
                    panic!("expected HighLevelErr but got {}", err);
                }
            }
            Ok(_) => {
                panic!("expected HighLevelErr but got game");
            }
        };
    }
}