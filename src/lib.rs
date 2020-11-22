use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::console;
use std::collections::HashMap;

mod figure;
mod base;
mod game;
mod engine;
mod js_interop;

pub use crate::game::{Game};
pub use crate::engine::evaluate;
pub use crate::figure::functions::allowed::get_allowed_moves;
pub use crate::engine::min_max::pruner::*;
use crate::base::{Move};
use crate::engine::evaluations::frontend::{GameEvaluation, GameEndResult, MoveEvaluation};
use crate::engine::evaluations::{DrawReason, EvaluatedMove, Evaluation, MIN_EVALUATION};
use crate::engine::{choose_next_move};
use crate::engine::min_max::evaluate_move;
use crate::js_interop::evaluate_single_move;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("wasm init"));

    Ok(())
}

static PRUNER: Pruner = PRUNER_1_2_3_4;

#[wasm_bindgen]
pub fn get_greeting_for(name: &str) -> JsValue {
    let greeting = format!("Hello, {}", name);
    JsValue::from_str(greeting.as_str())
}

#[wasm_bindgen]
pub fn get_concatenated_allowed_moves(game_config: &str) -> JsValue {
    let moves = get_allowed_moves(game_config);
    let moves_as_str: Vec<String> = moves.iter().map(|it|format!("{}", it)).collect();
    JsValue::from_str(format!("{:?}", moves_as_str).as_str())
}

#[wasm_bindgen]
pub fn get_fen(game_config: &str) -> JsValue {
    let fen_result = match game_config.parse::<Game>() {
        Ok(game) => {
            let fen = game.get_fen();
            FenResult {
                is_ok: true,
                value: fen,
            }
        }
        Err(err) => {
            let error_msg = format!("{:?}: {}", err.kind, err.msg);
            FenResult {
                is_ok: false,
                value: error_msg,
            }
        }
    };
    let json = serde_json::to_string(&fen_result).unwrap();
    JsValue::from_str(json.as_str())
}

#[wasm_bindgen]
pub fn evaluate_position_after(game_config: &str) -> JsValue {
    let evaluation = evaluate(game_config, PRUNER);
    let json = eval_to_json(evaluation, game_config);
    JsValue::from_str(json.as_str())
}

#[wasm_bindgen]
pub fn evaluate_move_after(game_config: &str, move_str: &str, game_eval_result_array_str: &str) -> JsValue {
    let current_max: Evaluation = get_current_max(game_eval_result_array_str);
    let json = match move_str.parse::<Move>() {
        Err(err) => {
            let err_msg = format!("{}", err);
            get_eval_json_end_or_err("Err", err_msg)
        }
        Ok(move_to_evaluate) => {
            let evaluation = evaluate_single_move(game_config, move_to_evaluate, PRUNER, current_max);
            eval_to_json(evaluation, game_config)
        }
    };

    JsValue::from_str(json.as_str())
}

#[wasm_bindgen]
pub fn pick_move_to_play(prelim_game_eval_result_array_str: &str) -> JsValue {

    console::log_1(&JsValue::from_str(format!("pick_move_to_play param: {}", prelim_game_eval_result_array_str).as_str()));

    let game_eval_results: Vec<GameEvaluationResult> = deserialize_game_eval_array(prelim_game_eval_result_array_str);

    let opt_game_eval_result_end_or_err = game_eval_results.iter().find(|result|{
        match result {
            GameEvaluationResult::EndOrErr(_) => {true}
            GameEvaluationResult::MoveToPlay(_) => {false}
        }
    });
    if let Some(game_eval_result_end_or_err) = opt_game_eval_result_end_or_err {
        let err_or_game_ended_json = match game_eval_result_end_or_err {
            GameEvaluationResult::EndOrErr(end_or_err) => {serde_json::to_string(end_or_err).unwrap()}
            GameEvaluationResult::MoveToPlay(_) => { get_eval_json_end_or_err("Err", "filtered by EndOrErr, still got MoveToPlay".to_string())}
        };
        return JsValue::from_str(err_or_game_ended_json.as_str());
    }
    // everything should be move to play
    let mut fen_by_move = HashMap::new();
    let moves_to_pick_from: Vec<EvaluatedMove> = game_eval_results.iter().map(|game_result|{
        match game_result {
            GameEvaluationResult::EndOrErr(_) => {panic!("only MoveToPlay expected at this point")}
            GameEvaluationResult::MoveToPlay(type4) => {
                let move_played:Move = type4.move_to_play.parse().unwrap();
                let eval: MoveEvaluation = serde_json::from_str(type4.eval.as_str()).unwrap();
                let fen = type4.fen.as_str();
                fen_by_move.insert(move_played, fen);
                EvaluatedMove {
                    a_move: move_played,
                    evaluation: eval,
                }
            }
        }
    }).collect();

    let next_move = choose_next_move(moves_to_pick_from);
    let fen = fen_by_move.get(&next_move.a_move).unwrap().to_string();
    let json = move_to_play_to_json(    next_move.a_move, next_move.evaluation, fen);

    console::log_1(&JsValue::from_str(format!("json of the picked gameEval: {}", json).as_str()));

    JsValue::from_str(json.as_str())
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialise_GameEvaluationResultMoveToPlay() {
        let move_to_play_result = GameEvaluationResultMoveToPlay {
            result_type: "MoveToPlay".to_string(),
            move_to_play: "b7-b6".to_string(),
            eval: "{\"Numeric\":-1.0050015}".to_string(),
            fen: "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string()
        };
        let serialized = serde_json::to_string(&move_to_play_result).unwrap();
        let deserialized: GameEvaluationResultMoveToPlay = serde_json::from_str(serialized.as_str()).unwrap();
        assert_eq!(
            deserialized,
            move_to_play_result,
        );
    }

    #[test]
    fn test_deserialise_GameEvaluationResultMoveToPlay() {
        let chosen_move = "a2-a4".parse::<Move>().unwrap();
        let eval = MoveEvaluation::Numeric(5.5);
        let fen = "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string();
        let json = move_to_play_to_json(chosen_move, eval, fen);
        let deserialized: GameEvaluationResultMoveToPlay = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(
            deserialized.move_to_play,
            "a2-a4",
        );
        assert_eq!(
            deserialized.fen,
            "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".to_string(),
        );
        let deserialized_move_eval: MoveEvaluation = serde_json::from_str(deserialized.eval.as_str()).unwrap();
        assert_eq!(
            deserialized_move_eval,
            MoveEvaluation::Numeric(5.5),
        );
    }
}
