use tinyvec::*;
use crate::figure::{FigureType};
use crate::{Position, Move, MatchState};
use crate::game::{Board, FieldContent};
use crate::base::{Color, STRAIGHT_DIRECTIONS, DIAGONAL_DIRECTIONS, ALL_DIRECTIONS, Direction, PawnPromotion, PromotionType, MoveArray};

#[cfg(test)]
// mod tests {
//     use tinyvec::*;
//     use crate::*;
//     use crate::game::*;
//     use crate::base::*;

    #[test]
    fn testing_works() {
        let mut move_collection: TinyVec<MoveArray> = tiny_vec!();
        for_reachable_knight_moves(
            Color::White,
            "b1".parse::<Position>().unwrap(),
            &Board::classic(),
            &mut move_collection,
        );
        assert_eq!(move_collection.len(), 2);
    }
// }

pub fn for_reachable_moves(
    fig_type: FigureType,
    pos: Position,
    match_state: &MatchState,
    move_collector: &mut TinyVec<MoveArray>,
) {
    match fig_type {
        FigureType::Pawn => for_reachable_pawn_moves(
            match_state.turn_by,
            pos,
            &match_state.board,
            match_state.en_passant_intercept_pos,
            move_collector
        ),
        FigureType::Rook(_) => for_reachable_rook_moves(
            match_state.turn_by,
            pos,
            &match_state.board,
            move_collector
        ),
        FigureType::Knight => for_reachable_knight_moves(
            match_state.turn_by,
            pos,
            &match_state.board,
            move_collector
        ),
        FigureType::Bishop => for_reachable_bishop_moves(
            match_state.turn_by,
            pos,
            &match_state.board,
            move_collector
        ),
        FigureType::Queen => for_reachable_queen_moves(
            match_state.turn_by,
            pos,
            &match_state.board,
            move_collector
        ),
        FigureType::King => {
            let is_queen_side_castling_possible: bool;
            let is_king_side_castling_possible: bool;
            match match_state.turn_by {
                Color::White => {
                    is_queen_side_castling_possible = match_state.is_white_queen_side_castling_possible;
                    is_king_side_castling_possible = match_state.is_white_king_side_castling_possible;
                },
                Color::Black => {
                    is_queen_side_castling_possible = match_state.is_black_queen_side_castling_possible;
                    is_king_side_castling_possible = match_state.is_black_king_side_castling_possible;
                },
            }
            for_reachable_king_moves(
                match_state.turn_by,
                pos,
                &match_state.board,
                is_queen_side_castling_possible,
                is_king_side_castling_possible,
                move_collector
            )
        },
    };
}

fn for_reachable_pawn_moves(
    color: Color,
    pawn_pos: Position,
    board: &Board,
    opt_en_passant_intercept_pos: Option<Position>,
    move_collector: &mut TinyVec<MoveArray>,
) {
    fn move_collector_diagonal_pawn_move(
        color: Color,
        pawn_pos: Position,
        diagonal_forward_pos: Position,
        board: &Board,
        opt_en_passant_intercept_pos: Option<Position>,
        move_collector: &mut TinyVec<MoveArray>,
    ) {
        match board.get_content_type(diagonal_forward_pos, color) {
            FieldContent::OpponentFigure => move_collector_pawn_moves(pawn_pos, diagonal_forward_pos, move_collector),
            FieldContent::Empty => {
                if let Some(en_passant_intercept_pos) = opt_en_passant_intercept_pos {
                    if en_passant_intercept_pos==diagonal_forward_pos {
                        move_collector.push(Move::new(pawn_pos, diagonal_forward_pos));
                    }
                }
            }
            _ => {}
        };
    }

    fn move_collector_pawn_moves(
        pawn_pos_from: Position,
        pawn_pos_to: Position,
        move_collector: &mut TinyVec<MoveArray>,
    ) {
        if pawn_pos_to.row==0 || pawn_pos_to.row==7 {
            [
                PawnPromotion::Yes(PromotionType::Queen),
                PawnPromotion::Yes(PromotionType::Knight),
            ].iter().for_each(|pawn_promo|{
                move_collector.push(Move{
                    from: pawn_pos_from,
                    to: pawn_pos_to,
                    pawn_promo: *pawn_promo
                });
            });
        } else {
            move_collector.push(Move{
                from: pawn_pos_from,
                to: pawn_pos_to,
                pawn_promo: PawnPromotion::No
            });
        };
    }

    let (forward_left, forward, forward_right) = Direction::forward_directions(color);
    if let Some(forward_left_pos) = pawn_pos.step(forward_left) {
        move_collector_diagonal_pawn_move(color, pawn_pos, forward_left_pos, board, opt_en_passant_intercept_pos, move_collector);
    }
    if let Some(forward_right_pos) = pawn_pos.step(forward_right) {
        move_collector_diagonal_pawn_move(color, pawn_pos, forward_right_pos, board, opt_en_passant_intercept_pos, move_collector);
    }
    if let Some(forward_pos) = pawn_pos.step(forward) {
        if let FieldContent::Empty = board.get_content_type(forward_pos, color) {
            move_collector_pawn_moves(pawn_pos, forward_pos, move_collector);

            // check for two-jump option
            if (pawn_pos.row==1 && color==Color::White) || (pawn_pos.row==6 && color==Color::Black) {
                let double_forward_pos = forward_pos.step(forward).unwrap();
                if let FieldContent::Empty = board.get_content_type(double_forward_pos, color) {
                    move_collector_pawn_moves(pawn_pos, double_forward_pos, move_collector);
                }
            }
        }
    }
}

fn for_reachable_rook_moves(
    color: Color,
    rook_pos: Position,
    board: &Board,
    move_collector: &mut TinyVec<MoveArray>,
) {
    STRAIGHT_DIRECTIONS.iter().for_each(|&direction|{
        for pos_to in rook_pos.reachable_directed_positions(color, direction, board) {
            move_collector.push(Move::new(rook_pos, pos_to));
        }
    });
}

fn for_reachable_knight_moves(
    color: Color,
    knight_pos: Position,
    board: &Board,
    move_collector: &mut TinyVec<MoveArray>,
) {
    for pos_to in knight_pos.reachable_knight_positions(color, board) {
        move_collector.push(Move::new(knight_pos, pos_to));
    }
}

fn for_reachable_bishop_moves(
    color: Color,
    bishop_pos: Position,
    board: &Board,
    move_collector: &mut TinyVec<MoveArray>,
) {
    DIAGONAL_DIRECTIONS.iter().for_each(|&direction|{
        for pos_to in bishop_pos.reachable_directed_positions(color, direction, board) {
            move_collector.push(Move::new(bishop_pos, pos_to));
        }
    });
}

fn for_reachable_queen_moves(
    color: Color,
    queen_pos: Position,
    board: &Board,
    move_collector: &mut TinyVec<MoveArray>,
) {
    ALL_DIRECTIONS.iter().for_each(|&direction|{
        for pos_to in queen_pos.reachable_directed_positions(color, direction, board) {
            move_collector.push(Move::new(queen_pos, pos_to));
        }
    });
}

fn for_reachable_king_moves(
    color: Color,
    king_pos: Position,
    board: &Board,
    is_queen_side_castling_possible: bool,
    is_king_side_castling_possible: bool,
    move_collector: &mut TinyVec<MoveArray>,
) {
    ALL_DIRECTIONS.iter().for_each(|&direction|{
        if let Some(pos_to) = king_pos.step(direction) {
            match board.get_figure(pos_to) {
                Some(figure) => if figure.color != color {
                    move_collector.push(Move::new(king_pos, pos_to))
                }
                None => move_collector.push(Move::new(king_pos, pos_to))
            }
        }
    });
    if is_queen_side_castling_possible {
        if let Some(rook_pos) = check_if_queen_side_castling_possible(color, king_pos, board) {
            move_collector.push(Move::new(king_pos, rook_pos))
        }
    }
    if is_king_side_castling_possible {
        if let Some(rook_pos) = check_if_king_side_castling_possible(color, king_pos, board) {
            move_collector.push(Move::new(king_pos, rook_pos))
        }
    }
}

fn check_if_queen_side_castling_possible(
    color: Color,
    king_pos: Position,
    board: &Board,
) -> Option<Position> {
    None
}


fn check_if_king_side_castling_possible(
    color: Color,
    king_pos: Position,
    board: &Board,
) -> Option<Position> {
    None
}
