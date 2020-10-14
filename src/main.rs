use voidchess_engine_rs::{Game};

fn main() {
    println!("Hello VoidChess");
    let moves = "e2-e4 d7-d5 e4-d5 c7-c5 d5-c6 c8-g4 g1-f3 b8-a6 f1-e2 d8-d6 e1-h1 e8-a8 f1-e1 g4-f3 a2-a3 c8-b8";
    match moves.parse::<Game>() {
        Ok(game) => {print!("game after a few moves:\n{}", game);}
        Err(chess_error) => {println!("chess error: {:?}", chess_error)}
    }

    // ♔♕♗♘♖♙♚♛♝♞♜♟
    let manual_config = "white ♙h2 ♙a4 ♘b7 ♔e3 ♖d4 ♕f4 ♚e6 ♝h7 ♛b2";
    match manual_config.parse::<Game>() {
        Ok(game) => {print!("configured game:\n{}", game);}
        Err(chess_error) => {println!("chess error: {:?}", chess_error)}
    }

    // use std::mem;
    // println!("size of pos: {}", mem::size_of::<Position>())
}