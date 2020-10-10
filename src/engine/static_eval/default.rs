use crate::base::{Color, Direction, Position};
use crate::figure::FigureType;
use crate::game::GameState;

pub fn default_static_eval_for_white(game_state: &GameState) -> f64 {
    let (white_figures, black_figures) = game_state.board.get_white_and_black_figures();
    let white_value = get_value(game_state, white_figures, Color::White);
    let black_value = get_value(game_state, black_figures, Color::Black);

    white_value - black_value
}

fn get_value(game_state: &GameState, figures: [Option<(FigureType, Position)>; 16], color: Color) -> f64 {
    let mut value = 0.0;
    let (backward_left, _, backward_right) = Direction::forward_directions(color.toggle());
    for opt_fig_data in figures.iter() {
        match opt_fig_data  {
            Some((fig_type, pos)) => {
                let fig_value = match fig_type {
                    FigureType::Pawn => get_pawn_value(game_state, pos, color, backward_left, backward_right),
                    FigureType::Rook(_) => 5.0,
                    FigureType::Knight => 3.0,
                    FigureType::Bishop => 3.01,
                    FigureType::Queen => 9.0,
                    FigureType::King => 0.0,
                };
                value = value + fig_value;
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
    pawn_pos: &Position,
    color: Color,
    backward_left: Direction,
    backward_right: Direction,
) -> f64 {
    fn is_protected(
        game_state: &GameState,
        pawn_pos: &Position,
        color: Color,
        backward_left: Direction,
        backward_right: Direction,
    ) -> bool {
        let opt_left_backward_figure = pawn_pos.step(backward_left).map(|pos|{game_state.board.get_figure(pos)}).flatten();
        if let Some(figure) = opt_left_backward_figure {
            if figure.fig_type == FigureType::Pawn && figure.color == color {
                return true;
            }
        }
        let opt_right_backward_figure = pawn_pos.step(backward_right).map(|pos|{game_state.board.get_figure(pos)}).flatten();
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
    } as f64;
    let steps_value = if is_protected(game_state, pawn_pos, color, backward_left, backward_right) {
        0.2
    } else {
        0.15
    };

    1.0 + (steps_value * steps_taken)
}