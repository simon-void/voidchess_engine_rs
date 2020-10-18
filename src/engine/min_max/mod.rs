use crate::game::{*};
use crate::engine::evaluations::{Evaluation, DrawReason, sort_evaluations_best_first};
use crate::base::{Color, Move};
use crate::engine::static_eval::{static_eval, StaticEvalType};
use crate::engine::evaluations::Evaluation::{WinIn, LoseIn};

mod pruner;

pub fn evaluate_move(
    old_game: &Game,
    a_move: &Move,
    move_depth: usize,
    evaluate_for: Color,
    eval_type: StaticEvalType,
) -> Evaluation {
    if old_game.is_passive_king_pos(&a_move.to) {
        return Evaluation::WinIn(0);
    }

    let new_half_step: usize = 1;
    let move_result = old_game.play(a_move);
    return match move_result {
        MoveResult::Stopped(reason, final_game_state) => {
            get_max_stopped_eval(reason, final_game_state, new_half_step, evaluate_for, eval_type)
        }
        MoveResult::Ongoing(game, _was_figure_caught) => {
            let half_step_depth = 2 * move_depth;
            let moves = game.get_reachable_moves();
            let max_eval = moves.iter().map(|next_move|
                get_min(&game, next_move, new_half_step, half_step_depth, evaluate_for, eval_type)
            ).max_by(sort_evaluations_best_first).unwrap();

            if is_max_eval_actually_stalemate(&max_eval, new_half_step, &game) {
                Evaluation::Draw(DrawReason::StaleMate)
            } else {
                max_eval
            }
        }
    };
}

fn get_min(old_game: &Game, a_move: &Move, old_half_step: usize, half_step_depth: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    let move_result = old_game.play(a_move);
    let new_half_step = old_half_step + 1;

    return match move_result {
        MoveResult::Stopped(reason, _) => {
            match reason {
                StoppedReason::KingInCheckAfterMove => Evaluation::WinIn(new_half_step as u8),
                StoppedReason::InsufficientMaterial => Evaluation::Draw(DrawReason::InsufficientMaterial),
                StoppedReason::ThreeTimesRepetition => Evaluation::Draw(DrawReason::ThreeTimesRepetition),
                StoppedReason::NoChangeIn50Moves => Evaluation::Draw(DrawReason::ThreeTimesRepetition),
            }
        }
        MoveResult::Ongoing(game, _was_figure_caught) => {
            if new_half_step >= half_step_depth {
                return Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for));
            }
            let moves = game.get_reachable_moves();
            let min_eval = moves.iter().map(|next_move|
                get_max(&game, next_move, new_half_step, half_step_depth, evaluate_for, eval_type)
            ).min_by(sort_evaluations_best_first).unwrap();

            if is_min_eval_actually_stalemate(&min_eval, new_half_step, &game) {
                Evaluation::Draw(DrawReason::StaleMate)
            } else {
                min_eval
            }
        }
    };
}

fn get_max(old_game: &Game, a_move: &Move, old_half_step: usize, half_step_depth: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    let move_result = old_game.play(a_move);
    let new_half_step = old_half_step + 1;

    return match move_result {
        MoveResult::Stopped(reason, final_game_state) => {
            get_max_stopped_eval(reason, final_game_state, new_half_step, evaluate_for, eval_type)
        }
        MoveResult::Ongoing(game, _was_figure_caught) => {
            if new_half_step >= half_step_depth {
                return Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for));
            }
            let moves = game.get_reachable_moves();
            let max_eval = moves.iter().map(|next_move|
                get_min(&game, next_move, new_half_step, half_step_depth, evaluate_for, eval_type)
            ).max_by(sort_evaluations_best_first).unwrap();

            if is_max_eval_actually_stalemate(&max_eval, new_half_step, &game) {
                Evaluation::Draw(DrawReason::StaleMate)
            } else {
                max_eval
            }
        }
    };
}

fn get_max_stopped_eval(
    reason: StoppedReason,
    final_game_state: GameState,
    new_half_step: usize,
    evaluate_for: Color,
    eval_type: StaticEvalType,
) -> Evaluation {
    match reason {
        StoppedReason::KingInCheckAfterMove => {
            get_lose_eval(&final_game_state, new_half_step, evaluate_for, eval_type)
        }
        StoppedReason::InsufficientMaterial => Evaluation::Draw(DrawReason::InsufficientMaterial),
        StoppedReason::ThreeTimesRepetition => Evaluation::Draw(DrawReason::ThreeTimesRepetition),
        StoppedReason::NoChangeIn50Moves => Evaluation::Draw(DrawReason::ThreeTimesRepetition),
    }
}

fn is_max_eval_actually_stalemate(max_eval: &Evaluation, half_step: usize, game: &Game) -> bool {
    if let Evaluation::WinIn(win_in_half_step) = max_eval {
        if *win_in_half_step as usize == (half_step + 1) && ! game.is_active_king_in_check() {
            return true;
        }
    }
    false
}

fn is_min_eval_actually_stalemate(min_eval: &Evaluation, half_step: usize, game: &Game) -> bool {
    if let Evaluation::LoseIn(lose_in_half_step, _) = min_eval {
        if *lose_in_half_step as usize == (half_step + 1) && ! game.is_active_king_in_check() {
            return true;
        }
    }
    false
}

fn get_lose_eval(game_state: &GameState, lost_after_nr_of_half_steps: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    Evaluation::LoseIn(lost_after_nr_of_half_steps as u8, static_eval(game_state, eval_type, evaluate_for))
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::engine::evaluations::*;
    use crate::engine::evaluations::testing::RoughEvaluation;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing_white, next_move_str, expected_evaluation,
    case("white ♔b6 ♙a7 ♚a8", "b6-a6", RoughEvaluation::Draw(DrawReason::StaleMate)),
    case("white ♔b6 ♙a7 ♚a8", "b6-c7", RoughEvaluation::Draw(DrawReason::InsufficientMaterial)),
    case("white ♔d6 ♖a7 ♚d8", "a7-a8", RoughEvaluation::WinIn(1)),
    case("white ♔f8 ♜a7 ♚e6", "f8-e8", RoughEvaluation::LoseIn(1)),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_evaluate_move(
        game_config_testing_white: &str,
        next_move_str: &str,
        expected_evaluation: RoughEvaluation,
    ) {
        let game = game_config_testing_white.parse::<Game>().unwrap();
        let next_move = next_move_str.parse::<Move>().unwrap();
        let actual_evaluation = evaluate_move(
            &game,
            &next_move,
            2,
            Color::White,
            StaticEvalType::Default,
        );
        assert_eq!(
            RoughEvaluation::from(&actual_evaluation),
            expected_evaluation,
            "original evaluation: {:?}", actual_evaluation
        );
    }
}