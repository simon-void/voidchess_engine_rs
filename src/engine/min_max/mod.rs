use crate::game::{Game, MoveResult};
use crate::engine::evaluations::{Evaluation, DrawReason, sort_evaluations_best_first};
use crate::base::{Color, Move};
use crate::engine::static_eval::{static_eval, StaticEvalType};

mod pruner;

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Test, // for testing
    Live,
}

pub fn evaluate_move(old_game: &Game, a_move: &Move, move_depth: usize, evaluate_for: Color, mode: Mode) -> Evaluation {
    let eval_type = match mode {
        Mode::Test => {StaticEvalType::AlwaysNull}
        Mode::Live => {StaticEvalType::Default}
    };
    let move_result = old_game.play(a_move);
    return match move_result {
        MoveResult::Stopped(reason) => {
            Evaluation::Draw(DrawReason::from(reason))
        }
        MoveResult::Ongoing(game, was_figure_caught) => {
            if game.is_passive_king_in_check() {
                return get_lose_eval(&game, 0, evaluate_for, eval_type);
            }
            let half_step_depth = 2 * move_depth;
            let moves = game.get_reachable_moves();
            let min_eval = moves.iter().map(|next_move|
                get_min(&game, next_move, 1, half_step_depth, evaluate_for, eval_type)
            ).max_by(sort_evaluations_best_first);
            min_eval.unwrap()
        }
    };
}

fn get_min(old_game: &Game, a_move: &Move, old_half_step: usize, half_step_depth: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    let move_result = old_game.play(a_move);
    return match move_result {
        MoveResult::Stopped(reason) => {
            Evaluation::Draw(DrawReason::from(reason))
        }
        MoveResult::Ongoing(game, was_figure_caught) => {
            let new_half_step = old_half_step + 1;
            if game.is_passive_king_in_check() {
                return Evaluation::WinIn((new_half_step/2) as u8);
            }
            if new_half_step == half_step_depth {
                return Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for));
            }
            let moves = game.get_reachable_moves();
            let min_eval = moves.iter().map(|next_move|
                get_max(&game, next_move, new_half_step, half_step_depth, evaluate_for, eval_type)
            ).min_by(sort_evaluations_best_first);
            min_eval.unwrap()
        }
    };
}

fn get_max(old_game: &Game, a_move: &Move, old_half_step: usize, half_step_depth: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    let move_result = old_game.play(a_move);
    return match move_result {
        MoveResult::Stopped(reason) => {
            Evaluation::Draw(DrawReason::from(reason))
        }
        MoveResult::Ongoing(game, was_figure_caught) => {
            let new_half_step = old_half_step + 1;
            if game.is_passive_king_in_check() {
                return get_lose_eval(&game, new_half_step, evaluate_for, eval_type);
            }
            if new_half_step == half_step_depth {
                return Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for));
            }
            let moves = game.get_reachable_moves();
            let min_eval = moves.iter().map(|next_move|
                get_min(&game, next_move, new_half_step, half_step_depth, evaluate_for, eval_type)
            ).max_by(sort_evaluations_best_first);
            min_eval.unwrap()
        }
    };
}

fn get_lose_eval(game: &Game, lost_after_nr_of_half_steps: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    Evaluation::LoseIn((lost_after_nr_of_half_steps/2) as u8, static_eval(game.get_game_state(), eval_type, evaluate_for))
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing_white, next_move_str, expected_evaluation,
    case("white ♔b6 ♙a7 ♚a8", "b6-a6", Evaluation::Draw(DrawReason::StaleMate)),
    case("white ♔b6 ♙a7 ♚a8", "b6-c7", Evaluation::Draw(DrawReason::InsufficientMaterial)),
    case("white ♔d6 ♖a7 ♚d8", "a7-a8", Evaluation::WinIn(1)),
    case("white ♔f8 ♜a7 ♚e6", "f8-e8", Evaluation::LoseIn(1, 0.0)),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    #[test]
    fn test_game_ends_bc_insufficient_material(
        game_config_testing_white: &str,
        next_move_str: &str,
        expected_evaluation: Evaluation,
    ) {
        let game = game_config_testing_white.parse::<Game>().unwrap();
        let next_move = next_move_str.parse::<Move>().unwrap();
        let actual_evaluation = evaluate_move(&game, &next_move, 1, Color::White, Mode::Test);
        assert_eq!(actual_evaluation, expected_evaluation);
    }
}