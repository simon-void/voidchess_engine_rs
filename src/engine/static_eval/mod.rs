use crate::GameState;
use crate::engine::evaluations::Evaluation;
use crate::engine::static_eval::default::default_static_eval_for_white;
use crate::base::Color;

mod default;

pub enum StaticEvalType {
    Default,
}

pub fn static_eval(game_state: &GameState, eval_type: StaticEvalType, for_color: Color) -> Evaluation {
    let eval_for_white = match eval_type {
        StaticEvalType::Default => default_static_eval_for_white(game_state),
    };
    let value = if for_color==Color::White {eval_for_white} else {-eval_for_white};
    Evaluation::Numeric(value)
}