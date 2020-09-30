use crate::base::{Colour, Position, Move};
use crate::figure::{Figure, FigureType, RookType};

static WHITE_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Colour::White,};
static WHITE_QUEEN_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::QueenSide), color: Colour::White,};
static WHITE_KING_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::KingSide), color: Colour::White,};
static WHITE_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Colour::White,};
static WHITE_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Colour::White,};
static WHITE_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Colour::White,};
static WHITE_KING: Figure = Figure {fig_type:FigureType::King, color: Colour::White,};

static BLACK_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Colour::Black,};
static BLACK_QUEEN_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::QueenSide), color: Colour::Black,};
static BLACK_KING_SIDE_ROOK: Figure = Figure {fig_type:FigureType::Rook(RookType::KingSide), color: Colour::Black,};
static BLACK_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Colour::Black,};
static BLACK_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Colour::Black,};
static BLACK_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Colour::Black,};
static BLACK_KING: Figure = Figure {fig_type:FigureType::King, color: Colour::Black,};

#[derive(Debug)]
pub struct MatchState {
    board: [[Option<Figure>; 8]; 8],
    next_turn_by: Colour,
    white_king_pos: Position,
    black_king_pos: Position,
    en_passant_intercept_pos: Option<Position>,
    is_white_queen_side_castling_possible: bool,
    is_white_king_side_castling_possible: bool,
    is_black_queen_side_castling_possible: bool,
    is_black_king_side_castling_possible: bool,
}

impl MatchState {
    pub fn new() -> MatchState {
        MatchState {
            board: [
                [
                    Some(WHITE_QUEEN_SIDE_ROOK),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_BISHOP),
                    Some(WHITE_QUEEN),
                    Some(WHITE_KING),
                    Some(WHITE_BISHOP),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_KING_SIDE_ROOK),
                ],
                [Some(WHITE_PAWN); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(BLACK_PAWN); 8],
                [
                    Some(BLACK_QUEEN_SIDE_ROOK),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_BISHOP),
                    Some(BLACK_QUEEN),
                    Some(BLACK_KING),
                    Some(BLACK_BISHOP),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_KING_SIDE_ROOK),
                ],
            ],
            next_turn_by: Colour::White,
            white_king_pos: "e1".parse::<Position>().unwrap(),
            black_king_pos: "e8".parse::<Position>().unwrap(),
            en_passant_intercept_pos: None,
            is_white_queen_side_castling_possible: true,
            is_white_king_side_castling_possible: true,
            is_black_queen_side_castling_possible: true,
            is_black_king_side_castling_possible: true,
        }
    }

    // fn after_move(&self, next_move: Move) -> MatchState {
    //
    // }
}
