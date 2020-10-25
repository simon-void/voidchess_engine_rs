use crate::game::*;
use crate::engine::evaluations::*;
use crate::engine::min_max::{evaluate_move};
use crate::engine::static_eval::StaticEvalType;
use crate::base::{ErrorKind};
use crate::engine::evaluations::frontend::*;

pub(crate) mod evaluations;
mod min_max;
mod static_eval;

pub fn evaluate(game_config: &str, move_depth: usize) -> GameEvaluation {
    let game = match game_config.parse::<Game>() {
        Err(err) => {
            return if let ErrorKind::HighLevelErr(stopped_reason) = err.kind {
                match stopped_reason {
                    StoppedReason::KingInCheckAfterMove => {GameEvaluation::GameEnded(GameEndResult::EngineWon)}
                    StoppedReason::InsufficientMaterial => {GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::InsufficientMaterial))}
                    StoppedReason::ThreeTimesRepetition => {GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::ThreeTimesRepetition))}
                    StoppedReason::NoChangeIn50Moves => {GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::NoChangeIn50Moves))}
                }
            } else {
                GameEvaluation::Err(format!("unexpected error {}, reason: {}", game_config, err))
            }
        }
        Ok(game) => {game}
    };

    let best_move: EvaluatedMove = evaluate_game(&game, move_depth);

    if let Evaluation::LoseIn(1, _) = best_move.evaluation {
        return if game.is_active_king_in_check() {
            GameEvaluation::GameEnded(GameEndResult::EngineLost)
        } else {
            GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::StaleMate))
        }
    }

    GameEvaluation::MoveToPlay(best_move.a_move, MoveEvaluation::from(&best_move.evaluation))
}

fn evaluate_game(game: &Game, move_depth: usize) -> EvaluatedMove {

    let eval_type = StaticEvalType::Default;
    let mut evaluated_moves: Vec<EvaluatedMove> = game.get_reachable_moves().iter().map(|next_move| {
        let evaluation = evaluate_move(
            &game,
            *next_move,
            move_depth,
            game.get_game_state().turn_by,
            eval_type,
        );
        EvaluatedMove { a_move: *next_move, evaluation }
    }).collect();

    evaluated_moves.sort_unstable_by(|e_m1, e_m2| e_m2.evaluation.cmp(&e_m1.evaluation));

    evaluated_moves.iter().for_each(|it|{
        println!("{} - {:?}", it.a_move, it.evaluation);
    });

    evaluated_moves.remove(0)
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;
    use crate::Game;
    use crate::engine::evaluations::*;
    use crate::engine::evaluations::frontend::*;
    use crate::base::Move;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config, move_depth, expected_evaluation,
    case("black ♔b6 ♙a7 ♚a8", 0, GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::StaleMate))),
    case("white ♔h8 ♚f8 ♜e7 ♟e6 ♟d7", 0, GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::StaleMate))),
    case("white ♔h8 ♚f8 ♞a7", 0, GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::InsufficientMaterial))),
    case("b1-c3 b8-c6 c3-b1 c6-b8 b1-c3 b8-c6 c3-b1 c6-b8 b1-c3", 0, GameEvaluation::GameEnded(GameEndResult::Draw(DrawReason::ThreeTimesRepetition))),
    case("white ♔g3 ♖d1 ♚g1 ♙c2 ♙d3", 0, GameEvaluation::GameEnded(GameEndResult::EngineWon)),
    case("black ♔g3 ♖d1 ♚g1 ♙c2 ♙d3", 0, GameEvaluation::GameEnded(GameEndResult::EngineLost)),
    case("white ♔g3 ♖d2 ♚g1 ♙c2 ♙d3", 1, GameEvaluation::MoveToPlay(Move::from_code("d2-d1"), MoveEvaluation::EngineCheckMatesIn(0))),
    case("white ♔f3 ♖d2 ♚h1 ♙c2 ♙d3", 2, GameEvaluation::MoveToPlay(Move::from_code("f3-g3"), MoveEvaluation::EngineCheckMatesIn(1))),
    case("white ♔h6 ♙g6 ♚h8 ♗f5 ♙e4", 2, GameEvaluation::MoveToPlay(Move::from_code("g6-g7"), MoveEvaluation::EngineCheckMatesIn(1))),
    case("white ♔e3 ♖d2 ♚g1 ♙c2 ♙d3", 3, GameEvaluation::MoveToPlay(Move::from_code("e3-f3"), MoveEvaluation::EngineCheckMatesIn(2))),
    case("black ♔h6 ♙g7 ♚h8 ♗f5 ♙e4", 2, GameEvaluation::MoveToPlay(Move::from_code("h8-g8"), MoveEvaluation::EngineGetsCheckMatedIn(1))),
    case("black ♔g3 ♖d2 ♚h1 ♙c2 ♙d3", 2, GameEvaluation::MoveToPlay(Move::from_code("h1-g1"), MoveEvaluation::EngineGetsCheckMatedIn(1))),
    case("black ♔f3 ♖d2 ♚g1 ♙c2 ♙d3", 3, GameEvaluation::MoveToPlay(Move::from_code("g1-h1"), MoveEvaluation::EngineGetsCheckMatedIn(2))),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_evaluate(
        game_config: &str,
        move_depth: usize,
        expected_evaluation: GameEvaluation,
    ) {
        let actual_evaluation = evaluate(game_config, move_depth);
        assert_eq!(
            actual_evaluation,
            expected_evaluation
        );
    }
}