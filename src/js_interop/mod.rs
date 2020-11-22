use serde::{Serialize, Deserialize};
use web_sys::console;
use crate::engine::evaluations::frontend::{GameEndResult, MoveEvaluation, GameEvaluation};
use crate::engine::evaluations::{Evaluation, MIN_EVALUATION, DrawReason};
use crate::base::Move;
use crate::engine::min_max::evaluate_move;
use crate::{Pruner, Game};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug)]
struct FenResult {
    is_ok: bool,
    value: String,
}

fn eval_to_json(game_eval: GameEvaluation, game_config: &str) -> String {
    match game_eval {
        GameEvaluation::GameEnded(result) => {
            let text = match result {
                GameEndResult::EngineWon => {"you are check mate"}
                GameEndResult::EngineLost => {"engine is check mate"}
                GameEndResult::Draw(reason) => {
                    match reason {
                        DrawReason::StaleMate => {"stale mate"}
                        DrawReason::InsufficientMaterial => {"draw because of insufficient material"}
                        DrawReason::ThreeTimesRepetition => {"draw because of three-fold repetition"}
                        DrawReason::NoChangeIn50Moves => {"draw because of no progress in 50 moves"}
                    }
                }
            };
            get_eval_json_end_or_err("GameEnded", String::from(text))
        }
        GameEvaluation::MoveToPlay(chosen_move, eval) => {
            let new_game_config = format!("{} {}", game_config, chosen_move);
            let fen = new_game_config.as_str().parse::<Game>().unwrap().get_fen();
            move_to_play_to_json(chosen_move, eval, fen)
        }
        GameEvaluation::Err(msg) => {
            get_eval_json_end_or_err("Err", msg)
        }
    }
}

fn move_to_play_to_json(chosen_move: Move, eval: MoveEvaluation, fen: String) -> String {
    let eval_string = serde_json::to_string(&eval).unwrap();
    let result = GameEvaluationResultMoveToPlay {
        result_type: "MoveToPlay".to_string(),
        move_to_play: chosen_move.to_string(),
        eval: eval_string,
        fen
    };
    serde_json::to_string(&result).unwrap()
}

fn get_eval_json_end_or_err(
    result_type: &str,
    msg: String,
) -> String {
    let result = GameEvaluationResultEndOrErr {
        result_type: result_type.to_string(),
        msg,
    };
    serde_json::to_string(&result).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
struct GameEvaluationResultEndOrErr {
    result_type: String,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct GameEvaluationResultMoveToPlay {
    result_type: String,
    move_to_play: String,
    eval: String,
    fen: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum GameEvaluationResult {
    EndOrErr(GameEvaluationResultEndOrErr),
    MoveToPlay(GameEvaluationResultMoveToPlay),
}

fn get_current_max(game_eval_result_array_str: &str) -> Evaluation {
    let game_eval_results: Vec<GameEvaluationResult> = deserialize_game_eval_array(game_eval_result_array_str);
    if game_eval_results.is_empty() {
        return MIN_EVALUATION;
    }
    let max_eval = game_eval_results.iter().filter_map(|result| {
        match result {
            GameEvaluationResult::EndOrErr(_) => { None }
            GameEvaluationResult::MoveToPlay(move_to_play) => {
                serde_json::from_str::<Evaluation>(move_to_play.eval.as_str()).map(|eval| {
                    Some(eval)
                }).unwrap_or(None);
            }
        }
    }).max().unwrap_or(&MIN_EVALUATION).clone();
    max_eval
}

fn deserialize_game_eval_array(game_eval_result_array_str: &str) -> Vec<GameEvaluationResult> {
    console::log_1(&JsValue::from_str(format!("pick_move_to_play param: {}", game_eval_result_array_str).as_str()));

    let game_eval_results: Vec<GameEvaluationResult> = if game_eval_result_array_str.is_empty() {
        vec![]
    } else {
        game_eval_result_array_str.split('|').map(|result_str| {
            let result = match serde_json::from_str::<GameEvaluationResultMoveToPlay>(result_str) {
                Ok(move_to_play_result) => {
                    GameEvaluationResult::MoveToPlay(move_to_play_result)
                }
                Err(_) => {
                    match serde_json::from_str::<GameEvaluationResultEndOrErr>(result_str) {
                        Ok(end_or_err_result) => {
                            GameEvaluationResult::EndOrErr(end_or_err_result)
                        }
                        Err(_) => {
                            panic!("json was neither GameEvalMoveToPlay nor GameEvalEnd/Err: {}", result_str);
                        }
                    }
                }
            };
            result
        }).collect()
    };

    game_eval_results
}

pub fn evaluate_single_move(game_config: &str, next_move: Move, pruner: Pruner, current_max: Evaluation) -> PreliminaryGameEvaluation {
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
        current_max,
        eval_type,
    );

    PreliminaryGameEvaluation::MoveTested(next_move, evaluation)
}

#[derive(Debug, Clone)]
enum PreliminaryGameEvaluation {
    GameEnded(GameEndResult),
    MoveTested(Move, Evaluation),
    Err(String),
}

