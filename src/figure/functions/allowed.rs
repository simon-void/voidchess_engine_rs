use crate::base::{Move, PawnPromotion, PromotionType};
use crate::Game;
use crate::game::{MoveResult, StoppedReason};

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
            match a_move.pawn_promo {
                PawnPromotion::Yes(promo_type) => {promo_type==PromotionType::Queen}
                PawnPromotion::No => {true}
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
    case("white ♔e1 ♖h1 ♚g8 ♝c4", 12),
    case("white ♔e1 ♖h1 ♚g8 ♝c3", 4),
    case("white ♔e1 ♖h1 ♚g8 ♝d2", 5),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_get_allowed_moves(game_config: &str, expected_nr_of_moves: usize) {
        let actual_nr_of_allowed_moves = get_allowed_moves(game_config).len();
        assert_eq!(actual_nr_of_allowed_moves, expected_nr_of_moves);
    }
}