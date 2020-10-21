mod game_state;
mod board;
mod board_state;

pub use crate::game::game_state::*;
pub use crate::game::board::*;
use crate::base::{Moves, ChessError, ErrorKind, Move, Position};
use std::{str, fmt};
use crate::game::board_state::{BoardStates};

#[derive(Clone, Debug)]
pub struct Game {
    latest_state: GameState,
    reachable_moves: Moves,
    board_states: BoardStates,
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
        let board_state = game_state.board.encode();
        let turn_by = game_state.turn_by;
        Game {
            latest_state: game_state,
            reachable_moves,
            board_states: BoardStates::new(board_state, turn_by),
        }
    }

    pub fn play(&self, a_move: &Move) -> MoveResult {
        let (new_game_state, move_stats) = self.latest_state.do_move(*a_move);

        let reachable_moves = match verify_game_state(&new_game_state) {
            Ok(moves) => { moves }
            Err(stopped_reason) => {
                return MoveResult::Stopped(stopped_reason, new_game_state);
            }
        };

        let new_board_states: BoardStates = {
            let new_board_state = new_game_state.board.encode();
            let new_turn_by = new_game_state.turn_by;
            let new_board_state_or_stopped_reason =
                self.board_states.add_board_state_and_check_for_draw(
                    new_board_state,
                    new_turn_by,
                    &move_stats,
                );

            match new_board_state_or_stopped_reason {
                Ok(board_states) => {board_states},
                Err(stopped_reason) => {
                    return MoveResult::Stopped(stopped_reason, new_game_state);
                },
            }
        };

        let new_game = Game {
            latest_state: new_game_state,
            reachable_moves,
            board_states: new_board_states,
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
    if let Err(stopped_reason) = verify_game_state(&game_state) {
        return Err(ChessError {
            msg: format!("game_state {} failed to pass verification: {:?}", game_state, &stopped_reason),
            kind: ErrorKind::HighLevelErr(stopped_reason),
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