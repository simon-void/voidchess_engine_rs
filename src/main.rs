use voidchess_engine_rs::{Position, Move};

fn main() {
    println!("Hello VoidChess");
    println!("Position should be e4: {}", "e4".parse::<Position>().unwrap());
    println!("Move should be a1-h8: {}", "a1-h8".parse::<Move>().unwrap());
    println!("Move should be a7Qa8: {}", "a7Qa8".parse::<Move>().unwrap());
}