use crate::game::{*};
use crate::engine::evaluations::{Evaluation, DrawReason, MIN_EVALUATION, MAX_EVALUATION};
use crate::base::{Color, Move, Moves};
use crate::engine::static_eval::{static_eval, StaticEvalType};
use crate::engine::min_max::pruner::Pruner;

pub mod pruner;

pub fn evaluate_move(
    old_game: &Game,
    a_move: Move,
    pruner: Pruner,
    evaluate_for: Color,
    current_max_one_level_up: Evaluation,
    eval_type: StaticEvalType,
) -> Evaluation {
    get_min_after(
        OldGameData {
            old_game,
            old_half_step: 0,
            was_check: false,
            old_move_stats: MoveStats::default(),
        },
        a_move,
        pruner,
        evaluate_for,
        current_max_one_level_up,
        eval_type,
    )
}

struct OldGameData<'a> {
    old_game: &'a Game,
    old_half_step: usize,
    was_check: bool,
    old_move_stats: MoveStats,
}

fn get_max_after(
    old_game_data: OldGameData,
    a_move: Move,
    pruner: Pruner,
    evaluate_for: Color,
    current_min_one_level_up: Evaluation,
    eval_type: StaticEvalType
) -> Evaluation {
    let move_result = old_game_data.old_game.play(a_move);
    let new_half_step = old_game_data.old_half_step + 1;

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
        MoveResult::Ongoing(game, move_stats) => {
            let is_check = game.is_active_king_in_check();
            if pruner.should_stop_min_max_ing(new_half_step, move_stats, old_game_data.old_move_stats, is_check, old_game_data.was_check) {
                return if game.is_active_king_checkmate() {
                    get_lose_eval(game.get_game_state(), new_half_step + 1, evaluate_for, eval_type)
                } else {
                    Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for))
                }
            }
            let moves = game.get_reachable_moves();
            let mut current_max = MIN_EVALUATION;
            for next_move in scramble(moves).iter() {
                let eval = get_min_after(
                    OldGameData {
                        old_game: &game,
                        old_half_step: new_half_step,
                        was_check: is_check,
                        old_move_stats: move_stats,
                    },
                    *next_move,
                    pruner,
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
    old_game_data: OldGameData,
    a_move: Move,
    pruner: Pruner,
    evaluate_for: Color,
    current_max_one_level_up: Evaluation,
    eval_type: StaticEvalType
) -> Evaluation {
    let move_result = old_game_data.old_game.play(a_move);
    let new_half_step = old_game_data.old_half_step + 1;

    debug_assert!(new_half_step%2==1, "get_max's new_half_step is supposed to be odd, but was {}", new_half_step);

    return match move_result {
        MoveResult::Stopped(reason, final_game_state) => {
            get_min_after_stopped_eval(reason, *final_game_state, new_half_step, evaluate_for, eval_type)
        }
        MoveResult::Ongoing(game, move_stats) => {
            let is_check = game.is_active_king_in_check();
            if pruner.should_stop_min_max_ing(new_half_step, move_stats, old_game_data.old_move_stats, is_check, old_game_data.was_check) {
                return if game.is_active_king_checkmate() {
                    Evaluation::WinIn((new_half_step + 1) as u8)
                } else {
                    Evaluation::Numeric(static_eval(game.get_game_state(), eval_type, evaluate_for))
                }
            }
            let moves = game.get_reachable_moves();
            let mut current_min = MAX_EVALUATION;
            for next_move in scramble(moves).iter() {
                let eval = get_max_after(
                    OldGameData {
                        old_game: &game,
                        old_half_step: new_half_step,
                        was_check: is_check,
                        old_move_stats: move_stats,
                    },
                    *next_move,
                    pruner,
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
    use crate::engine::evaluations::testing::{EvaluationMatcher};
    use crate::engine::min_max::pruner::PRUNER_L2;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config_testing_white, next_move_str, expected_matcher,
    case("white ♔b6 ♙a7 ♚a8", "b6-a6", EvaluationMatcher::Draw(DrawReason::StaleMate)),
    case("white ♔b6 ♙a7 ♚a8", "b6-c7", EvaluationMatcher::Draw(DrawReason::InsufficientMaterial)),
    case("white ♔d6 ♖a7 ♚d8", "a7-a8", EvaluationMatcher::WinIn),
    case("white ♔f8 ♜a7 ♚e6", "f8-e8", EvaluationMatcher::LoseIn),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_evaluate_move(
        game_config_testing_white: &str,
        next_move_str: &str,
        expected_matcher: EvaluationMatcher,
    ) {
        let game = game_config_testing_white.parse::<Game>().unwrap();
        let next_move = next_move_str.parse::<Move>().unwrap();
        let actual_evaluation = evaluate_move(
            &game,
            next_move,
            PRUNER_L2,
            Color::White,
            MIN_EVALUATION,
            StaticEvalType::Default,
        );
        assert!(
            expected_matcher.matches(&actual_evaluation),
            "actual_eval: {:?}",
            actual_evaluation,
        );
    }
}