use crate::figure::{FigureType};
use crate::game::{Board};
use crate::base::{Color, STRAIGHT_DIRECTIONS, DIAGONAL_DIRECTIONS, ALL_DIRECTIONS, Position};

pub fn count_reachable_moves(
    fig_type: FigureType,
    color: Color,
    pos: Position,
    board: &Board,
) -> usize {
    match fig_type {
        FigureType::Pawn => 0,
        FigureType::Rook => count_reachable_rook_moves(
            color,
            pos,
            board,
        ),
        FigureType::Knight => count_reachable_knight_moves(
            color,
            pos,
            board,
        ),
        FigureType::Bishop => count_reachable_bishop_moves(
            color,
            pos,
            board,
        ),
        FigureType::Queen => count_reachable_queen_moves(
            color,
            pos,
            board,
        ),
        FigureType::King => 0,
    }
}

fn count_reachable_rook_moves(
    color: Color,
    rook_pos: Position,
    board: &Board,
) -> usize {
    let mut counter: usize = 0;
    STRAIGHT_DIRECTIONS.iter().for_each(|&direction|{
        counter += rook_pos.count_reachable_directed_positions(color, direction, board);
    });
    counter
}

fn count_reachable_knight_moves(
    color: Color,
    knight_pos: Position,
    board: &Board,
) -> usize {
    knight_pos.count_reachable_knight_positions(color, board)
}

fn count_reachable_bishop_moves(
    color: Color,
    bishop_pos: Position,
    board: &Board,
) -> usize {
    let mut counter: usize = 0;
    DIAGONAL_DIRECTIONS.iter().for_each(|&direction|{
        counter += bishop_pos.count_reachable_directed_positions(color, direction, board);
    });
    counter
}

fn count_reachable_queen_moves(
    color: Color,
    queen_pos: Position,
    board: &Board,
) -> usize {
    let mut counter: usize = 0;
    ALL_DIRECTIONS.iter().for_each(|&direction|{
        counter += queen_pos.count_reachable_directed_positions(color, direction, board);
    });
    counter
}

//------------------------------Tests------------------------

// #[cfg(test)]
// mod tests {
//     use super::*;
// }