use crate::game::Game;
use crate::engine::evaluations::*;
use crate::engine::min_max::{evaluate_move};

pub(crate) mod evaluations;
mod min_max;
mod static_eval;

pub fn evaluate(game: Game, move_depth: usize) -> Vec<EvaluatedMove> {
    // let mut results: Vec<EvaluatedMove> = vec![];
    // for next_move in game.get_reachable_moves().iter() {
    //     let eval = evaluate_move(&game, next_move, 3, game.get_game_state().turn_by, StaticEvalType::default);
    //     results.push(EvaluatedMove{a_move: *next_move, evaluation: eval,})
    // }

    let mut results: Vec<EvaluatedMove> = game.get_reachable_moves().iter().map(|next_move| {
        let evaluation = evaluate_move(&game, next_move, move_depth, game.get_game_state().turn_by);
        EvaluatedMove { a_move: *next_move, evaluation }
    }).collect();

    results.sort_unstable_by(|e_m1, e_m2|
        sort_evaluations_best_first(&e_m1.evaluation, &e_m2.evaluation)
    );
    results
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::Game;
    use crate::engine::evaluations::*;
    use crate::engine::evaluate;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config, move_depth, expected_evaluation,
    case("black ♔b6 ♙a7 ♚a8", 1, RoughEvaluation::Draw(DrawReason::StaleMate)),
    case("white ♔b1 ♛c1 ♚a8", 1, RoughEvaluation::Draw(DrawReason::InsufficientMaterial)),
    case("white ♔d6 ♖a8 ♚d8", 1, RoughEvaluation::WinIn(0)),
    case("white ♔d6 ♖a7 ♚d8", 1, RoughEvaluation::WinIn(1)),
    case("white ♔d6 ♖a5 ♖b6 ♚h8", 2, RoughEvaluation::WinIn(2)),
    case("white ♔f8 ♜a8 ♚f6", 1, RoughEvaluation::LoseIn(0)),
    case("white ♔h8 ♜a7 ♚g6", 1, RoughEvaluation::LoseIn(1)),
    case("white ♔h7 ♜a5 ♜b6 ♚h8", 2, RoughEvaluation::LoseIn(2)),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    #[test]
    fn test_evaluate(
        game_config: &str,
        move_depth: usize,
        expected_evaluation: RoughEvaluation,
    ) {
        let game = game_config.parse::<Game>().unwrap();
        let actual_evaluation = evaluate(game, move_depth).iter().next().map(|it| it.evaluation).unwrap();
        assert_eq!(
            RoughEvaluation::of(&actual_evaluation),
            expected_evaluation,
            "original evaluation: {:?}", actual_evaluation
        );
    }
}