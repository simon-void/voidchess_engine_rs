use crate::base::{Moves};
use crate::game::Game;
use crate::engine::evaluations::*;
use crate::engine::min_max::evaluate_move;

pub(crate) mod evaluations;
mod min_max;
mod static_eval;

pub fn evaluate_position_after(moves_so_far: Moves) -> Vec<EvaluatedMove> {
    let game = Game::classic_and_then(moves_so_far);
    let mut results: Vec<EvaluatedMove> = vec![];
    for next_move in game.get_reachable_moves().iter() {
        let eval = evaluate_move(&game, next_move);
        results.push(EvaluatedMove{a_move: *next_move, evaluation: eval,})
    }
    results.sort_unstable_by(|eval_move1, eval_move2|{
        let ascending_order = eval_move1.evaluation.partial_cmp(&eval_move2.evaluation).unwrap();
        ascending_order.reverse()
    });
    results
}