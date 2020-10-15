use crate::game::Game;
use crate::engine::evaluations::*;
use crate::engine::min_max::evaluate_move;
use crate::engine::static_eval::StaticEvalType;

pub(crate) mod evaluations;
mod min_max;
mod static_eval;

pub fn evaluate(game: Game) -> Vec<EvaluatedMove> {
    let mut results: Vec<EvaluatedMove> = vec![];
    for next_move in game.get_reachable_moves().iter() {
        let eval = evaluate_move(&game, next_move, 3, game.get_game_state().turn_by, StaticEvalType::default);
        results.push(EvaluatedMove{a_move: *next_move, evaluation: eval,})
    }
    results.sort_unstable_by(|eval_move1, eval_move2|{
        let ascending_order = eval_move1.evaluation.partial_cmp(&eval_move2.evaluation).unwrap();
        ascending_order.reverse()
    });
    results
}