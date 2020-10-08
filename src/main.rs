use voidchess_engine_rs::{Position, Move, GameState};

fn main() {
    println!("Hello VoidChess");
    let match_state = GameState::classic();
    let match_state = match_state.do_move("e2-e4".parse::<Move>().unwrap());
    let match_state = match_state.do_move("d7-d5".parse::<Move>().unwrap());
    let match_state = match_state.do_move("e4-d5".parse::<Move>().unwrap());
    let match_state = match_state.do_move("c7-c5".parse::<Move>().unwrap());
    let match_state = match_state.do_move("d5-c6".parse::<Move>().unwrap());
    let match_state = match_state.do_move("c8-g4".parse::<Move>().unwrap());
    let match_state = match_state.do_move("g1-f3".parse::<Move>().unwrap());
    let match_state = match_state.do_move("b8-a6".parse::<Move>().unwrap());
    let match_state = match_state.do_move("f1-e2".parse::<Move>().unwrap());
    let match_state = match_state.do_move("d8-d6".parse::<Move>().unwrap());
    let match_state = match_state.do_move("e1-h1".parse::<Move>().unwrap());
    let match_state = match_state.do_move("e8-a8".parse::<Move>().unwrap());
    let match_state = match_state.do_move("f1-e1".parse::<Move>().unwrap());
    let match_state = match_state.do_move("g4-f3".parse::<Move>().unwrap());
    let match_state = match_state.do_move("a2-a3".parse::<Move>().unwrap());
    let match_state = match_state.do_move("c8-b8".parse::<Move>().unwrap());
    println!("game after a few moves: {}", match_state);
}