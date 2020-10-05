use crate::game::Game;
use crate::Move;
use crate::engine::evaluations::Evaluation;
use crate::base::Color;

pub fn evaluate_move(game: &Game, a_move: &Move) -> Evaluation {
    todo!();
}

fn get_min(game: &Game, half_step: usize, evaluate_for: Color) -> Evaluation {
    todo!();
}

fn get_max(game: &Game, half_step: usize, evaluate_for: Color) -> Evaluation {
    todo!();
}