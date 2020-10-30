use wasm_bindgen::prelude::*;

mod figure;
mod base;
mod game;
mod engine;

pub use crate::game::{Game};
pub use crate::engine::evaluate;
pub use crate::figure::functions::allowed::get_allowed_moves;
pub use crate::engine::min_max::pruner::*;


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