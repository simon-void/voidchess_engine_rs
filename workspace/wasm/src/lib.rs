extern crate core;

use std::collections::HashMap;
use std::convert::{From, Into};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

use engine_core::*;

use crate::base::Move;
use crate::engine::{choose_next_move, evaluate_single_move};
pub use crate::engine::evaluate;
use crate::engine::evaluations::{DrawReason, EvaluatedMove};
use crate::engine::evaluations::frontend::{GameEndResult, GameEvaluation, MoveEvaluation};
pub use crate::engine::min_max::pruner::*;
pub use crate::figure::functions::allowed::get_allowed_moves;
pub use crate::game::Game;

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

static PRUNER: Pruner = PRUNER_1_1_3_3;

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

// fn get_result_json(is_ok: bool, value: String) -> String {
//     format!("{}{}{}{}{}", "{\"isOk\":", is_ok, ", \"value\":\"", value, "\"}")
// }

#[derive(Serialize, Deserialize, Debug)]
struct FenResult {
    is_ok: bool,
    value: String,
}

#[wasm_bindgen]
pub fn evaluate_position_after(game_config: &str) -> JsValue {
    let evaluation = evaluate(game_config, PRUNER);
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
    let eval_string = serde_json::to_string(&SerializableMoveEvaluation::from(eval)).unwrap();
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

#[wasm_bindgen]
pub fn evaluate_move_after(game_config: &str, move_str: &str) -> JsValue {
    let json = match move_str.parse::<Move>() {
        Err(err) => {
            let err_msg = format!("{}", err);
            get_eval_json_end_or_err("Err", err_msg)
        }
        Ok(move_to_evaluate) => {
            let evaluation = evaluate_single_move(game_config, move_to_evaluate, PRUNER);
            eval_to_json(evaluation, game_config)
        }
    };

    JsValue::from_str(json.as_str())
}

#[wasm_bindgen]
pub fn pick_move_to_play(game_eval_result_array_str: &str) -> JsValue {

    console::log_1(&JsValue::from_str(format!("pick_move_to_play param: {}", game_eval_result_array_str).as_str()));

    let game_eval_results: Vec<GameEvaluationResult> = game_eval_result_array_str.split('|').map(|result_str|{
        match serde_json::from_str::<GameEvaluationResultMoveToPlay>(result_str) {
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
        }
    }).collect();

    // let game_eval_results: Vec<GameEvaluationResult> = serde_json::from_str(game_eval_result_array_str).unwrap();
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
                let eval: MoveEvaluation = serde_json::from_str::<SerializableMoveEvaluation>(type4.eval.as_str()).unwrap().into();
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SerializableDrawReason {
    StaleMate,
    InsufficientMaterial,
    ThreeTimesRepetition,
    NoChangeIn50Moves,
}

impl From<DrawReason> for SerializableDrawReason {
    fn from(item: DrawReason) -> Self {
        match item {
            DrawReason::StaleMate => SerializableDrawReason::StaleMate,
            DrawReason::InsufficientMaterial => SerializableDrawReason::InsufficientMaterial,
            DrawReason::ThreeTimesRepetition => SerializableDrawReason::ThreeTimesRepetition,
            DrawReason::NoChangeIn50Moves => SerializableDrawReason::NoChangeIn50Moves,
        }
    }
}

impl From<SerializableDrawReason> for DrawReason {
    fn from(reason: SerializableDrawReason) -> Self {
        match reason {
            SerializableDrawReason::StaleMate => DrawReason::StaleMate,
            SerializableDrawReason::InsufficientMaterial => DrawReason::InsufficientMaterial,
            SerializableDrawReason::ThreeTimesRepetition => DrawReason::ThreeTimesRepetition,
            SerializableDrawReason::NoChangeIn50Moves => DrawReason::NoChangeIn50Moves,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SerializableMoveEvaluation {
    EngineCheckMatesIn(u8),
    Numeric(f32),
    Draw(SerializableDrawReason),
    EngineGetsCheckMatedIn(u8, f32),
}

impl From<MoveEvaluation> for SerializableMoveEvaluation {
    fn from(item: MoveEvaluation) -> Self {
        match item {
            MoveEvaluation::EngineCheckMatesIn(count) => SerializableMoveEvaluation::EngineCheckMatesIn(count),
            MoveEvaluation::Numeric(value) => SerializableMoveEvaluation::Numeric(value),
            MoveEvaluation::Draw(reason) => SerializableMoveEvaluation::Draw(reason.into()),
            MoveEvaluation::EngineGetsCheckMatedIn(count, value) => SerializableMoveEvaluation::EngineGetsCheckMatedIn(count, value),
        }
    }
}

impl From<SerializableMoveEvaluation> for MoveEvaluation {
    fn from(evaluation: SerializableMoveEvaluation) -> Self {
        match evaluation {
            SerializableMoveEvaluation::EngineCheckMatesIn(count) => MoveEvaluation::EngineCheckMatesIn(count),
            SerializableMoveEvaluation::Numeric(value) => MoveEvaluation::Numeric(value),
            SerializableMoveEvaluation::Draw(reason) => MoveEvaluation::Draw(reason.into()),
            SerializableMoveEvaluation::EngineGetsCheckMatedIn(count, value) => MoveEvaluation::EngineGetsCheckMatedIn(count, value),
        }
    }
}



//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_serialize_deserialize_GameEvaluationResultMoveToPlay() {
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
    #[allow(non_snake_case)]
    fn test_deserialize_GameEvaluationResultMoveToPlay() {
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
        let deserialized_move_eval: MoveEvaluation = serde_json::from_str::<SerializableMoveEvaluation>(deserialized.eval.as_str()).unwrap().into();
        assert_eq!(
            deserialized_move_eval,
            MoveEvaluation::Numeric(5.5),
        );
    }
}
