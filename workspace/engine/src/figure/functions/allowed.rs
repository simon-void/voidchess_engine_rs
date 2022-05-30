use crate::base::{Move, MoveType, PromotionType};
use crate::game::{Game, MoveResult, StoppedReason};

pub fn get_allowed_moves(game_config: &str) -> Vec<Move> {
    let mut movable_moves: Vec<Move> = vec!();
    let game = match game_config.parse::<Game>() {
        Err(_) => {
            return movable_moves;
        }
        Ok(game) => {game}
    };

    game.get_reachable_moves().iter().filter(
        |&&a_move| is_not_bound(&game, a_move)
    ).filter(
        |&&a_move| {
            if let MoveType::PawnPromotion(promo_type) = a_move.move_type {
                promo_type==PromotionType::Queen
            } else {
                true
            }
        }
    ).for_each(
        |&a_move| movable_moves.push(a_move)
    );

    movable_moves
}

fn is_not_bound(game: &Game, a_move: Move) -> bool {
    match game.play(a_move) {
        MoveResult::Ongoing(_, _) => {true}
        MoveResult::Stopped(reason, _) => {
            reason!=StoppedReason::KingInCheckAfterMove
        }
    }
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config, expected_nr_of_moves,
    case("", 20),
    case("white ♔a1 ♚h8", 0),
    case("white ♔a1 ♖h1 ♚h8", 0),
    case("white ♔a1 ♖g1 ♚h8", 16),
    case("white ♔e1 ♖h1 ♚g8", 15),
    case("white ♔e1 ♖a1 ♚g8", 16),
    case("white ♔e1 ♖h1 ♚g8 ♝c4", 12),
    case("white ♔e1 ♖h1 ♚g8 ♝c3", 4),
    case("white ♔e1 ♖h1 ♚g8 ♝d2", 5),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_allowed_moves_count(game_config: &str, expected_nr_of_moves: usize) {
        let moves = get_allowed_moves(game_config);
        let actual_nr_of_allowed_moves = moves.len();
        if actual_nr_of_allowed_moves != expected_nr_of_moves {
            println!("moves: {:?}", moves)
        }
        assert_eq!(actual_nr_of_allowed_moves, expected_nr_of_moves);
    }

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config, expected_move_str,
    case("white ♔e1 ♙a7 ♚g8", "a7Qa8"),
    case("white ♔e1 ♖a1 ♚g8", "e1Cc1"),
    case("white ♔e1 ♖h1 ♚g8", "e1cg1"),
    case("a2-a4 a7-a6 a4-a5 b7-b5", "a5eb6"),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_allowed_moves_contains(game_config: &str, expected_move_str: &str) {
        let expected_move = expected_move_str.parse::<Move>().unwrap();
        let moves = get_allowed_moves(game_config);
        let actual_contains = moves.contains(&expected_move);
        if !actual_contains {
            println!("moves: {:?}", moves)
        }
        assert_eq!(actual_contains, true);
    }

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[rstest(
    game_config, unexpected_move_str,
    case("white ♔e1 ♙a7 ♚g8", "a7-a8"),
    case("white ♔e1 ♙a7 ♚g8", "a7Ra8"),
    case("white ♔e1 ♙a7 ♚g8", "a7Ka8"),
    case("white ♔e1 ♙a7 ♚g8", "a7Ba8"),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_allowed_moves_doesnt_contain(game_config: &str, unexpected_move_str: &str) {
        let unexpected_move = unexpected_move_str.parse::<Move>().unwrap();
        let moves = get_allowed_moves(game_config);
        let actual_contains = moves.contains(&unexpected_move);
        if actual_contains {
            println!("moves: {:?}", moves)
        }
        assert_eq!(actual_contains, false);
    }
}