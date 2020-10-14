use crate::base::{Position, Color, Direction};
use crate::game::Board;
use crate::figure::FigureType;

pub fn is_king_in_check(king_pos: Position, color: Color, board: &Board) -> bool {
    let (forward_left, forward, forward_right) = Direction::forward_directions(color);

    is_king_straight_attackable(king_pos, color, forward, board) ||
        is_king_straight_attackable(king_pos, color, Direction::Left, board) ||
        is_king_straight_attackable(king_pos, color, Direction::Right, board) ||
        is_king_forward_diagonal_attackable(king_pos, color, forward_left, board) ||
        is_king_forward_diagonal_attackable(king_pos, color, forward_right, board) ||
        is_king_attackable_by_knight(king_pos, color, board) ||
        (
            (!king_pos.is_on_ground_row(color)) && (
                is_king_straight_attackable(king_pos, color, forward.reverse(), board) ||
                    is_king_backward_diagonal_attackable(king_pos, color, forward_left.reverse(), board) ||
                    is_king_backward_diagonal_attackable(king_pos, color, forward_right.reverse(), board)
            )
        )
}

pub fn is_king_straight_attackable(king_pos: Position, color: Color, dir: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, dir, board) {
        return match opponent_fig_type {
            FigureType::Rook | FigureType::Queen => true,
            FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

pub fn is_king_forward_diagonal_attackable(king_pos: Position, color: Color, diagonal: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, diagonal, board) {
        return match opponent_fig_type {
            FigureType::Bishop | FigureType::Queen => true,
            FigureType::Pawn | FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

fn is_king_backward_diagonal_attackable(king_pos: Position, color: Color, diagonal: Direction, board: &Board) -> bool {
    // since the king is trying to castling, he has to stand on the ground row
    if let Some((opponent_fig_type, is_attacker_next_to_self)) = get_first_opposite_color_figure_type_in_direction(king_pos, color, diagonal, board) {
        return match opponent_fig_type {
            FigureType::Bishop | FigureType::Queen => true,
            FigureType::King if is_attacker_next_to_self => true,
            _ => false,
        };
    }
    false
}

pub fn is_king_attackable_by_knight(king_pos: Position, color: Color, board: &Board) -> bool {
    for possible_knight_attacker_pos in king_pos.reachable_knight_positions(color, board) {
        if let Some(figure) = board.get_figure(possible_knight_attacker_pos) {
            // different color is already guaranteed by reachable_knight_positions
            if figure.fig_type == FigureType::Knight {
                return true;
            }
        }
    }
    false
}

/*
 * returns the optional figure_type and if that figure is right next to self
 */
fn get_first_opposite_color_figure_type_in_direction(
    king_pos: Position,
    king_color: Color,
    direction: Direction,
    board: &Board,
) -> Option<(FigureType, bool)> {
    let mut old_pos = king_pos;
    let mut is_attacker_next_to_self = true;
    loop {
        let new_pos = match old_pos.step(direction) {
            Some(pos) => {
                pos
            }
            None => { return None; }
        };
        if let Some(figure) = board.get_figure(new_pos) {
            return if figure.color == king_color {
                None
            } else {
                Some((figure.fig_type, is_attacker_next_to_self))
            };
        }
        old_pos = new_pos;
        is_attacker_next_to_self = false;
    }
}
