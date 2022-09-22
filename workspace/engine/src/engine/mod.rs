use rand::prelude::*;
use crate::game::*;
use crate::engine::evaluations::*;
use crate::engine::min_max::{evaluate_move};
use crate::engine::static_eval::StaticEvalType;
use crate::base::{ErrorKind, Move};
use crate::engine::evaluations::frontend::*;
use crate::engine::min_max::pruner::*;

pub mod evaluations;
pub mod min_max;
mod static_eval;

pub fn evaluate_single_move(game_config: &str, next_move: Move, pruner: Pruner) -> GameEvaluation {
    let game_or_final_eval = init_game(game_config);
    let game = match game_or_final_eval {
        OngoingGameOrEvaluation::Ongoing(game) => {game}
        OngoingGameOrEvaluation::Ended(final_eval) => {return final_eval;}
    };

    let eval_type = get_eval_type_for(&game);

    let evaluation = evaluate_move(
        &game,
        next_move,
        pruner,
        game.get_game_state().turn_by,
        MIN_EVALUATION,
        eval_type,
    );

    GameEvaluation::MoveToPlay(next_move, MoveEvaluation::from(&evaluation))
}

pub fn evaluate(game_config: &str, pruner: Pruner) -> GameEvaluation {
    let game_or_final_eval = init_game(game_config);
    let game = match game_or_final_eval {
        OngoingGameOrEvaluation::Ongoing(game) => {game}
        OngoingGameOrEvaluation::Ended(final_eval) => {return final_eval;}
    };

    let best_move: EvaluatedMove = evaluate_game(&game, pruner);

    if let MoveEvaluation::EngineGetsCheckMatedIn(0, _) = best_move.evaluation {
        return if game.is_active_king_in_check() {
            GameEvaluation::GameEnded(GameEndResult::EngineLost)
        } else {
            GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::StaleMate))
        }
    }

    GameEvaluation::MoveToPlay(best_move.a_move, best_move.evaluation)
}

fn evaluate_game(game: &Game, pruner: Pruner) -> EvaluatedMove {

    let eval_type = get_eval_type_for(game);
    let mut evaluated_moves: Vec<EvaluatedMove> = vec![];
    let mut current_max_eval: Evaluation = MIN_EVALUATION;
    for next_move in game.get_reachable_moves().iter() {
        let evaluation = evaluate_move(
            game,
            *next_move,
            pruner,
            game.get_game_state().turn_by,
            current_max_eval,
            eval_type,
        );

        if evaluation > current_max_eval {
            current_max_eval = evaluation;
        }

        evaluated_moves.push(EvaluatedMove { a_move: *next_move, evaluation: MoveEvaluation::from(&evaluation) });
    }

    choose_next_move(evaluated_moves)
}

pub fn choose_next_move(mut evaluated_moves: Vec<EvaluatedMove>) -> EvaluatedMove {
    fn get_numeric_eval(evaluation: &MoveEvaluation) -> Option<f32> {
        match evaluation {
            MoveEvaluation::Numeric(numeric_eval) => Some(*numeric_eval),
            MoveEvaluation::Draw(_) => Some(0.0),
            _ => None,
        }
    }

    evaluated_moves.sort_unstable_by(|e_m1, e_m2| e_m2.evaluation.cmp(&e_m1.evaluation));

    let mut best_first_iter = evaluated_moves.iter();
    let mut chosen_move = *best_first_iter.next().expect("iterator should contain at least one move");
    let best_eval = match get_numeric_eval(&chosen_move.evaluation) {
        None => { return chosen_move; }
        Some(num_eval) => { num_eval }
    };
    let mut random = thread_rng();
    loop {
        if random.gen::<f32>() < 0.7 {
            break;
        }
        let next_move = match best_first_iter.next() {
            None => { break; }
            Some(eval_move) => { *eval_move }
        };
        let next_eval = match get_numeric_eval(&next_move.evaluation) {
            None => { break; }
            Some(num_eval) => { num_eval }
        };
        if best_eval - next_eval < 0.2 {
            chosen_move = next_move;
        } else {
            break;
        }
    }
    chosen_move
}

fn get_eval_type_for(_game: &Game) -> StaticEvalType {
    StaticEvalType::Default
}

fn init_game(game_config: &str) -> OngoingGameOrEvaluation {
    let game = match game_config.parse::<Game>() {
        Err(err) => {
            let game_eval = if let ErrorKind::HighLevelErr(stopped_reason) = err.kind {
                match stopped_reason {
                    StoppedReason::KingInCheckAfterMove => {GameEvaluation::GameEnded(GameEndResult::EngineWon)}
                    StoppedReason::InsufficientMaterial => {GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::InsufficientMaterial))}
                    StoppedReason::ThreeTimesRepetition => {GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::ThreeTimesRepetition))}
                    StoppedReason::NoChangeIn50Moves => {GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::NoChangeIn50Moves))}
                }
            } else {
                GameEvaluation::Err(format!("unexpected error {}, reason: {}", game_config, err))
            };
            return OngoingGameOrEvaluation::Ended(game_eval);
        }
        Ok(game) => {game}
    };
    OngoingGameOrEvaluation::Ongoing(Box::new(game))
}

enum OngoingGameOrEvaluation {
    Ongoing(Box<Game>),
    Ended(GameEvaluation),
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;
    use crate::base::Move;
    use crate::engine::evaluations::testing::{GameEvaluationMatcher, MoveEvaluationMatcher};

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config, pruner, expected_matcher,
    case("black ♔b6 ♙a7 ♚a8", PRUNER_L0, GameEvaluationMatcher::GameEnded(GameEndResult::Draw(DrawReason::StaleMate))),
    case("white ♔h8 ♚f8 ♜e7 ♟e6 ♟d7", PRUNER_L0, GameEvaluationMatcher::GameEnded(GameEndResult::Draw(DrawReason::StaleMate))),
    case("white ♔h8 ♚f8 ♞a7", PRUNER_L0, GameEvaluationMatcher::GameEnded(GameEndResult::Draw(DrawReason::InsufficientMaterial))),
    case("b1-c3 b8-c6 c3-b1 c6-b8 b1-c3 b8-c6 c3-b1 c6-b8 b1-c3", PRUNER_L0, GameEvaluationMatcher::GameEnded(GameEndResult::Draw(DrawReason::ThreeTimesRepetition))),
    case("white ♔g3 ♖d1 ♚g1 ♙c2 ♙d3", PRUNER_L0, GameEvaluationMatcher::GameEnded(GameEndResult::EngineWon)),
    case("black ♔g3 ♖d1 ♚g1 ♙c2 ♙d3", PRUNER_L0, GameEvaluationMatcher::GameEnded(GameEndResult::EngineLost)),
    case("white ♔g3 ♖d2 ♚g1 ♙c2 ♙d3", PRUNER_L0, GameEvaluationMatcher::MoveToPlay(Move::from_code("d2-d1"), MoveEvaluationMatcher::EngineCheckMatesIn(0))),
    case("white ♔f3 ♖d2 ♚h1 ♙c2 ♙d3", PRUNER_L2, GameEvaluationMatcher::MoveToPlay(Move::from_code("f3-g3"), MoveEvaluationMatcher::EngineCheckMatesIn(1))),
    case("white ♔h6 ♙g6 ♚h8 ♗f5 ♙e4", PRUNER_L2, GameEvaluationMatcher::MoveToPlay(Move::from_code("g6-g7"), MoveEvaluationMatcher::EngineCheckMatesIn(1))),
    case("white ♔e3 ♖d2 ♚g1 ♙c2 ♙d3", PRUNER_L3, GameEvaluationMatcher::MoveToPlay(Move::from_code("e3-f3"), MoveEvaluationMatcher::EngineCheckMatesIn(2))),
    case("black ♔h6 ♙g7 ♚h8 ♗f5 ♙e4", PRUNER_L1, GameEvaluationMatcher::MoveToPlay(Move::from_code("h8-g8"), MoveEvaluationMatcher::EngineGetsCheckMatedIn(1))),
    case("black ♔g3 ♖d2 ♚h1 ♙c2 ♙d3", PRUNER_L1, GameEvaluationMatcher::MoveToPlay(Move::from_code("h1-g1"), MoveEvaluationMatcher::EngineGetsCheckMatedIn(1))),
    case("black ♔f3 ♖d2 ♚g1 ♙c2 ♙d3", PRUNER_L2, GameEvaluationMatcher::MoveToPlay(Move::from_code("g1-h1"), MoveEvaluationMatcher::EngineGetsCheckMatedIn(2))),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_evaluate(
        game_config: &str,
        pruner: Pruner,
        expected_matcher: GameEvaluationMatcher,
    ) {
        let actual_evaluation = evaluate(game_config, pruner);
        assert!(
            expected_matcher.matches(&actual_evaluation),
            "actual_eval: {:?}",
            actual_evaluation,
        );
    }
}