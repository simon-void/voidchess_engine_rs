mod game_state;
mod board;

pub use crate::game::game_state::*;
pub use crate::game::board::*;
use crate::base::{Moves, ChessError, ErrorKind, Move};
use std::iter::Peekable;
use std::{str, fmt};

#[derive(Clone, Debug)]
pub struct Game {
    latest_state: GameState,
    reachable_moves: Moves,
}

impl Game {
    pub fn classic() -> Game {
        let latest_state = GameState::classic();
        Game::from(latest_state)
    }

    pub fn from(game_state: GameState) -> Game {
        let latest_reachable_moves = game_state.get_reachable_moves();
        Game {
            latest_state: game_state,
            reachable_moves: latest_reachable_moves,
        }
    }

    pub fn play(&self, a_move: &Move) -> MoveResult {
        let new_game_state = self.latest_state.do_move(*a_move);
        let new_game = Game::from(new_game_state);
        let move_result = MoveResult::Ongoing(new_game, false); //TODO don't hardcode Ongoing and don't hardcode false(=no figure hit)!
        move_result
    }

    pub fn get_reachable_moves(&self) -> &Moves {
        &self.reachable_moves
    }

    pub fn can_passive_players_king_be_caught(&self) -> bool {
        todo!();
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
        let mut token_iter = trimmed_desc.split(" ").into_iter().peekable();
        let first_token = *token_iter.peek().unwrap();

        if first_token=="white" || first_token=="black" {
            game_by_figures_on_board(token_iter)
        } else {
            game_by_moves_from_start(token_iter)
        }
    }
}

fn game_by_moves_from_start(token_iter: Peekable<str::Split<&str>>) -> Result<Game, ChessError> {
    let mut game = Game::classic();
    for token in token_iter {
        let a_move = token.parse::<Move>()?;
        let move_result = game.play(&a_move);
        match move_result {
            MoveResult::Ongoing(new_game, _) => {
                game = new_game;
            }
            MoveResult::Stopped(reason) => {
                return Err(ChessError {
                    msg: format!("game has already ended after move {} in final state {}", a_move, game),
                    kind: ErrorKind::IllegalConfiguration,
                });
            }
        }
    }
    Ok(game)
}

fn game_by_figures_on_board(token_iter: Peekable<str::Split<&str>>) -> Result<Game, ChessError> {
    todo!()
}

pub enum MoveResult {
    /*
     * bool: was figure taken
     */
    Ongoing(Game, bool),
    Stopped(StoppedReason),
}

pub enum StoppedReason {
    InsufficientMaterial,
    ThreeTimesRepetition,
    NoChangeIn50Moves,
}