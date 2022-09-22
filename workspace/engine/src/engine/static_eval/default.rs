use crate::base::{Color, Direction, Position};
use crate::figure::FigureType;
use crate::game::{FiguresWithPosArray, GameState};

const VALUE_OF_AREA: f32 = 0.015;

pub fn default_static_eval_for_white(game_state: &GameState) -> f32 {
    let (white_figures, black_figures) = game_state.board.get_white_and_black_figures();
    let white_value = get_value(game_state, white_figures, Color::White);
    let black_value = get_value(game_state, black_figures, Color::Black);

    let rules_area_diff_for_white = game_state.count_reachable_moves_diff_for_white();

    (white_value - black_value) + (rules_area_diff_for_white as f32 * VALUE_OF_AREA)
}

fn get_value(game_state: &GameState, figures: FiguresWithPosArray, color: Color) -> f32 {
    let mut value = 0.0;
    let (backward_left, _, backward_right) = Direction::forward_directions(color.toggle());
    for opt_fig_data in figures.iter() {
        match opt_fig_data  {
            Some((fig_type, pos)) => {
                let fig_value = match fig_type {
                    FigureType::Pawn => get_pawn_value(game_state, *pos, color, backward_left, backward_right),
                    FigureType::Rook => 5.0,
                    FigureType::Knight => 3.0,
                    FigureType::Bishop => 3.01,
                    FigureType::Queen => 9.0,
                    FigureType::King => 0.0,
                };
                value += fig_value;
            },
            None => {
                break;
            }
        }
    }
    value
}

fn get_pawn_value(
    game_state: &GameState,
    pawn_pos: Position,
    color: Color,
    backward_left: Direction,
    backward_right: Direction,
) -> f32 {
    fn is_protected(
        game_state: &GameState,
        pawn_pos: Position,
        color: Color,
        backward_left: Direction,
        backward_right: Direction,
    ) -> bool {
        let opt_left_backward_figure = pawn_pos.step(backward_left).and_then(|pos|{game_state.board.get_figure(pos)});
        if let Some(figure) = opt_left_backward_figure {
            if figure.fig_type == FigureType::Pawn && figure.color == color {
                return true;
            }
        }
        let opt_right_backward_figure = pawn_pos.step(backward_right).and_then(|pos|{game_state.board.get_figure(pos)});
        if let Some(figure) = opt_right_backward_figure {
            if figure.fig_type == FigureType::Pawn && figure.color == color {
                return true;
            }
        }
        false
    }

    let steps_taken = match color {
        Color::White => pawn_pos.row - 1,
        Color::Black => 6 -pawn_pos.row,
    } as f32;
    let steps_value = if is_protected(game_state, pawn_pos, color, backward_left, backward_right) {
        0.2
    } else {
        0.15
    };

    1.0 + (steps_value * steps_taken)
}