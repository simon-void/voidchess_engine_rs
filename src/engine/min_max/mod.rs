use crate::game::{*};
use crate::engine::evaluations::{Evaluation, DrawReason, MIN_EVALUATION, MAX_EVALUATION};
use crate::base::{Color, Move, Moves};
use crate::engine::static_eval::{static_eval, StaticEvalType};

mod pruner;

pub fn evaluate_move(
    old_game: &Game,
    a_move: Move,
    move_depth: usize,
    evaluate_for: Color,
    eval_type: StaticEvalType,
) -> Evaluation {
    let current_max_one_level_up = MIN_EVALUATION;

    get_min_after(
        old_game,
        a_move,
        0,
        2 * move_depth,
        evaluate_for,
        current_max_one_level_up,
        eval_type,
    )
}

fn get_max_after(
    old_game: &Game,
    a_move: Move,
    old_half_step: usize,
    half_step_depth: usize,
    evaluate_for: Color,
    current_min_one_level_up: Evaluation,
    eval_type: StaticEvalType
) -> Evaluation {
    let move_result = old_game.play(a_move);
    let new_half_step = old_half_step + 1;

    debug_assert!(new_half_step%2==0, "get_min's new_half_step is supposed to be even, but was {}", new_half_step);

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
            let mut current_max = MIN_EVALUATION;
            for next_move in scramble(moves).iter() {
                let eval = get_min_after(
                    &game,
                    *next_move,
                    new_half_step,
                    half_step_depth,
                    evaluate_for,
                    current_max,
                    eval_type
                );
                if eval>current_max {
                    current_max = eval;
                    if eval >= current_min_one_level_up {
                        return eval;
                    }
                }
            }

            if is_max_eval_actually_stalemate(current_max, new_half_step, &game) {
                Evaluation::Draw(DrawReason::StaleMate)
            } else {
                current_max
            }
        }
    };
}

fn get_min_after(
    old_game: &Game,
    a_move: Move,
    old_half_step: usize,
    half_step_depth: usize,
    evaluate_for: Color,
    current_max_one_level_up: Evaluation,
    eval_type: StaticEvalType
) -> Evaluation {
    let move_result = old_game.play(a_move);
    let new_half_step = old_half_step + 1;

    debug_assert!(new_half_step%2==1, "get_max's new_half_step is supposed to be odd, but was {}", new_half_step);

    return match move_result {
        MoveResult::Stopped(reason, final_game_state) => {
            get_min_after_stopped_eval(reason, final_game_state, new_half_step, evaluate_for, eval_type)
        }
        MoveResult::Ongoing(game, _was_figure_caught) => {
            if new_half_step >= half_step_depth {
                return Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for));
            }
            let moves = game.get_reachable_moves();
            let mut current_min = MAX_EVALUATION;
            for next_move in scramble(moves).iter() {
                let eval = get_max_after(
                    &game,
                    *next_move,
                    new_half_step,
                    half_step_depth,
                    evaluate_for,
                    current_min,
                    eval_type
                );
                if eval<current_min {
                    current_min = eval;
                    if eval <= current_max_one_level_up {
                        return eval;
                    }
                }
            }

            if is_min_eval_actually_stalemate(current_min, new_half_step, &game) {
                Evaluation::Draw(DrawReason::StaleMate)
            } else {
                current_min
            }
        }
    };
}

fn get_min_after_stopped_eval(
    reason: StoppedReason,
    final_game_state: GameState,
    new_half_step: usize,
    evaluate_for: Color,
    eval_type: StaticEvalType,
) -> Evaluation {
    println!("get_min_after_stopped_eval reached with new_half_step {}", new_half_step);
    match reason {
        StoppedReason::KingInCheckAfterMove => {
            get_lose_eval(&final_game_state, new_half_step, evaluate_for, eval_type)
        }
        StoppedReason::InsufficientMaterial => Evaluation::Draw(DrawReason::InsufficientMaterial),
        StoppedReason::ThreeTimesRepetition => Evaluation::Draw(DrawReason::ThreeTimesRepetition),
        StoppedReason::NoChangeIn50Moves => Evaluation::Draw(DrawReason::ThreeTimesRepetition),
    }
}

fn is_min_eval_actually_stalemate(max_eval: Evaluation, half_step: usize, game: &Game) -> bool {
    debug_assert!(half_step%2==1, "is_max_eval_actually_stalemate's half_step is supposed to be odd, but was {}", half_step);
    if let Evaluation::WinIn(win_in_half_step) = max_eval {
        debug_assert!(win_in_half_step%2==0, "is_max_eval_actually_stalemate's win_in_half_step is supposed to be even, but was {}", win_in_half_step);
        if win_in_half_step as usize == (half_step + 1) && !game.is_active_king_in_check() {
            return true;
        }
    }
    false
}

fn is_max_eval_actually_stalemate(min_eval: Evaluation, half_step: usize, game: &Game) -> bool {
    debug_assert!(half_step%2==0, "is_min_eval_actually_stalemate's half_step is supposed to be even, but was {}", half_step);
    if let Evaluation::LoseIn(lose_in_half_step, _) = min_eval {
        debug_assert!(lose_in_half_step%2==1, "is_min_eval_actually_stalemate's half_step is supposed to be odd, but was {}", lose_in_half_step);
        if lose_in_half_step as usize == (half_step + 1) && !game.is_active_king_in_check() {
            return true;
        }
    }
    false
}

fn get_lose_eval(game_state: &GameState, lost_after_nr_of_half_steps: usize, evaluate_for: Color, eval_type: StaticEvalType) -> Evaluation {
    debug_assert!(lost_after_nr_of_half_steps%2==1, "get_lose_eval's half_step is supposed to be odd, but was {}", lost_after_nr_of_half_steps);
    Evaluation::LoseIn(lost_after_nr_of_half_steps as u8, static_eval(game_state, eval_type, evaluate_for))
}

// improves the performance of alpha-beta pruning
fn scramble(moves: &Moves) -> Moves {
    let mut clones = moves.clone();
    let nr_of_moves = moves.len();
    if nr_of_moves > 4 {
        for start in (1..(nr_of_moves - 1) / 2).step_by(2) {
            clones.swap(start, nr_of_moves - start);
        }
    }
    clones
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
            next_move,
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