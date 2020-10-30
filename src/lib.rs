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