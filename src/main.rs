use voidchess_engine_rs::Position;
use voidchess_engine_rs::Move;

fn main() {
    println!("Hello VoidChess");
    println!("Position should be e4: {}", "e4".parse::<Position>().unwrap());
    println!("Move should be a1-h8: {}", "a1-h8".parse::<Move>().unwrap());
}