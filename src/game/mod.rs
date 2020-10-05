mod game_state;
mod board;

pub use crate::game::game_state::*;
pub use crate::game::board::*;
use crate::{Move, Position};
use tinyvec::TinyVec;
use crate::base::{Moves};

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

    pub fn classic_and_then(moves_so_far: Moves) -> Game {
        let latest_state = GameState::classic();
        for past_move in moves_so_far.iter() {
            latest_state.do_move(*past_move);
        }
        Game::from(latest_state)
    }

    pub fn from(game_state: GameState) -> Game {
        let latest_reachable_moves = game_state.get_reachable_moves();
        Game {
            latest_state: game_state,
            reachable_moves: latest_reachable_moves,
        }
    }

    pub fn play(&self, a_move: Move) -> Game {
        todo!();
    }

    pub fn get_reachable_moves(&self) -> &Moves {
        &self.reachable_moves
    }

    pub fn can_passive_players_king_be_caught(&self) -> bool {
        todo!();
    }
}