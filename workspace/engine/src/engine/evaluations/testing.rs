#![allow(unused)]

use super::*;
use crate::engine::evaluations::frontend::{GameEvaluation, GameEndResult};

#[derive(Debug, Copy, Clone)]
pub enum EvaluationMatcher {
    WinIn,
    AnyNumeric,
    PositiveNumeric,
    NegativeNumeric,
    Draw(DrawReason),
    LoseIn,
}

impl EvaluationMatcher {
    pub fn matches(&self, eval: &Evaluation) -> bool {
        match self {
            EvaluationMatcher::WinIn => {
                matches!(eval, Evaluation::WinIn(_))
            }
            EvaluationMatcher::AnyNumeric => {
                matches!(eval, Evaluation::Numeric(_))
            }
            EvaluationMatcher::PositiveNumeric => {
                if let Evaluation::Numeric(num_eval) = eval {
                    *num_eval >= 0.0
                } else {
                    false
                }
            }
            EvaluationMatcher::NegativeNumeric => {
                if let Evaluation::Numeric(num_eval) = eval {
                    *num_eval < 0.0
                } else {
                    false
                }
            }
            EvaluationMatcher::Draw(matcher_reason) => {
                if let Evaluation::Draw(eval_reason) = eval {
                    *matcher_reason == *eval_reason
                } else {
                    false
                }
            }
            EvaluationMatcher::LoseIn => {
                matches!(eval, Evaluation::LoseIn(_, _))
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MoveEvaluationMatcher {
    EngineCheckMatesIn(u8),
    AnyNumeric,
    PositiveNumeric,
    NegativeNumeric,
    Draw(DrawReason),
    EngineGetsCheckMatedIn(u8),
}

impl MoveEvaluationMatcher {
    pub fn matches(&self, move_eval: &MoveEvaluation) -> bool {
        match self {
            MoveEvaluationMatcher::EngineCheckMatesIn(matcher_checkmate_in) => {
                if let MoveEvaluation::EngineCheckMatesIn(eval_checkmate_in) = move_eval {
                    *matcher_checkmate_in == *eval_checkmate_in
                } else {
                    false
                }
            }
            MoveEvaluationMatcher::AnyNumeric => {
                matches!(move_eval, MoveEvaluation::Numeric(_))
            }
            MoveEvaluationMatcher::PositiveNumeric => {
                if let MoveEvaluation::Numeric(num_eval) = move_eval {
                    *num_eval >= 0.0
                } else {
                    false
                }
            }
            MoveEvaluationMatcher::NegativeNumeric => {
                if let MoveEvaluation::Numeric(num_eval) = move_eval {
                    *num_eval < 0.0
                } else {
                    false
                }
            }
            MoveEvaluationMatcher::Draw(matcher_reason) => {
                if let MoveEvaluation::Draw(eval_reason) = move_eval {
                    *matcher_reason == *eval_reason
                } else {
                    false
                }
            }
            MoveEvaluationMatcher::EngineGetsCheckMatedIn(matcher_checkmate_in) => {
                if let MoveEvaluation::EngineGetsCheckMatedIn(eval_checkmate_in, _) = move_eval {
                    *matcher_checkmate_in == *eval_checkmate_in
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameEvaluationMatcher {
    GameEnded(GameEndResult),
    MoveToPlay(Move, MoveEvaluationMatcher),
    Err,
}

impl GameEvaluationMatcher {
    pub fn matches(&self, game_eval: &GameEvaluation) -> bool {
        match self {
            GameEvaluationMatcher::GameEnded(matcher_result) => {
                if let GameEvaluation::GameEnded(game_result) = game_eval {
                    *matcher_result == *game_result
                } else {
                    false
                }
            }
            GameEvaluationMatcher::MoveToPlay(matcher_move, matcher_move_matcher) => {
                if let GameEvaluation::MoveToPlay(game_move, game_eval) = game_eval {
                    *matcher_move==*game_move && matcher_move_matcher.matches(game_eval)
                } else {
                    false
                }
            }
            GameEvaluationMatcher::Err => {
                matches!(game_eval, GameEvaluation::Err(_))
            }
        }
    }
}
