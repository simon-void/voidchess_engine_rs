use crate::game::{Game, MoveResult};
use crate::engine::evaluations::{Evaluation, DrawReason};
use crate::base::{Color, Move};

mod pruner;

pub fn evaluate_move(old_game: &Game, a_move: &Move) -> Evaluation {
    todo!();
}

fn get_min(old_game: &Game, a_move: &Move, half_step: usize, evaluate_for: Color) -> Evaluation {
    let move_result = old_game.play(a_move);
    match move_result {
        MoveResult::Stopped(reason) => {
            Evaluation::Draw(DrawReason::from(reason))
        },
        MoveResult::Ongoing(game, was_figure_caught) => {
            if game.can_passive_players_king_be_caught() {
                return Evaluation::LooseIn(half_step)
            }

            todo!();
        }
    }
}

fn get_max(old_game: &Game, a_move: &Move, half_step: usize, evaluate_for: Color) -> Evaluation {
    todo!();
}