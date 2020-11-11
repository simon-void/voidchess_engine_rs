use wasm_bindgen::prelude::*;

mod figure;
mod base;
mod game;
mod engine;

pub use crate::game::{Game};
pub use crate::engine::evaluate;
pub use crate::figure::functions::allowed::get_allowed_moves;
pub use crate::engine::min_max::pruner::*;
use crate::base::ChessError;
use crate::engine::evaluations::frontend::{GameEvaluation, GameEndResult};
use crate::engine::evaluations::DrawReason;


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
    let json = match game_config.parse::<Game>() {
        Ok(game) => {
            let fen = game.get_fen();
            get_result_json(true, fen)
        }
        Err(err) => {
            let error_msg = format!("{:?}: {}", err.kind, err.msg);
            get_result_json(false, error_msg)
        }
    };
    JsValue::from_str(json.as_str())
}

fn get_result_json(is_ok: bool, value: String) -> String {
    format!("{}{}{}{}{}", "{\"isOk\":", is_ok, ", \"value\":\"", value, "\"}")
}

#[wasm_bindgen]
pub fn evaluate_position_after(game_config: &str) -> JsValue {
    let evaluation = evaluate(game_config, PRUNER_2_4_6);
    let json = eval_to_json(evaluation, game_config);
    JsValue::from_str(json.as_str())
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
            get_eval_json2("GameEnded", "msg", String::from(text))
        }
        GameEvaluation::MoveToPlay(chosen_move, eval) => {
            let eval_string = format!("{:?}", eval);
            let new_game_config = format!("{} {}", game_config, chosen_move);
            let fen = new_game_config.as_str().parse::<Game>().unwrap().get_fen();
            get_eval_json4(
                "MoveToPlay",
                "move", chosen_move.to_string(),
                "eval", eval_string,
                "fen", fen
            )
        }
        GameEvaluation::Err(msg) => {
            get_eval_json2("Err", "msg", msg)
        }
    }
}

fn get_eval_json4(
    value1: &str,
    key2: &str,
    value2: String,
    key3: &str,
    value3: String,
    key4: &str,
    value4: String,
) -> String {
    format!("{}\"type\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\",\"{}\":\"{}\"{}", "{", value1, key2, value2, key3, value3, key4, value4, "}")
}

fn get_eval_json2(
    value1: &str,
    key2: &str,
    value2: String,
) -> String {
    format!("{}\"type\":\"{}\",\"{}\":\"{}\"{}", "{", value1, key2, value2, "}")
}