use std::fmt;
use std::char;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    column: u8,
    row: u8,
}

impl Position {
    pub fn from_str(code: &str) -> Position {
        let mut char_iter = code.chars();
        Position {
            column: (char_iter.next().unwrap() as u8) - 97,
            row: (char_iter.next().unwrap() as u8) - 49,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", (self.column + 97) as char, (self.row+49) as char)
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    from: Position,
    to: Position,
    pawn_promo: Option<PromotionType>,
}

impl Move {
    fn from_str(code: &str) -> Move {
        Move {
            from: Position::from_str(&code[0..2]),
            to: Position::from_str(&code[3..5]),
            pawn_promo: PromotionType::from_str(&code[2..3])
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //TODO replace the separator with the proper PawnPromo character
        write!(f, "{}-{}", self.from, self.to)
    }
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Black, White,
}

#[derive(Debug, Copy, Clone)]
enum FigureType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
enum PromotionType {
    Rook,
    Knight,
    Bishop,
    Queen,
}

impl PromotionType {
    fn from_str(s: &str) -> Option<PromotionType> {
        match s {
            "-" => None,
            "Q" => Some(PromotionType::Queen),
            "R" => Some(PromotionType::Rook),
            "K" => Some(PromotionType::Knight),
            "B" => Some(PromotionType::Bishop),
            _ => panic!("unknown pawn promotion type"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Figure {
    fig_type: FigureType,
    color: Color,
}

static WHITE_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Color::White,};
static WHITE_ROOK: Figure = Figure {fig_type:FigureType::Rook, color: Color::White,};
static WHITE_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Color::White,};
static WHITE_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Color::White,};
static WHITE_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Color::White,};
static WHITE_KING: Figure = Figure {fig_type:FigureType::King, color: Color::White,};

static BLACK_PAWN: Figure = Figure {fig_type:FigureType::Pawn, color: Color::Black,};
static BLACK_ROOK: Figure = Figure {fig_type:FigureType::Rook, color: Color::Black,};
static BLACK_KNIGHT: Figure = Figure {fig_type:FigureType::Knight, color: Color::Black,};
static BLACK_BISHOP: Figure = Figure {fig_type:FigureType::Bishop, color: Color::Black,};
static BLACK_QUEEN: Figure = Figure {fig_type:FigureType::Queen, color: Color::Black,};
static BLACK_KING: Figure = Figure {fig_type:FigureType::King, color: Color::Black,};


#[derive(Debug, Copy, Clone)]
struct GameState {
    board: [[Option<Figure>; 8]; 8],
    white_king_pos: Position,
    black_king_pos: Position,
    en_passant_intercept_pos: Option<Position>,
    has_white_left_rook_moved: bool,
    has_white_right_rook_moved: bool,
    has_white_king_moved: bool,
    has_black_left_rook_moved: bool,
    has_black_right_rook_moved: bool,
    has_black_king_moved: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            board: [
                [
                    Some(WHITE_ROOK),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_BISHOP),
                    Some(WHITE_QUEEN),
                    Some(WHITE_KING),
                    Some(WHITE_BISHOP),
                    Some(WHITE_KNIGHT),
                    Some(WHITE_ROOK),
                ],
                [Some(WHITE_PAWN); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(BLACK_PAWN); 8],
                [
                    Some(BLACK_ROOK),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_BISHOP),
                    Some(BLACK_QUEEN),
                    Some(BLACK_KING),
                    Some(BLACK_BISHOP),
                    Some(BLACK_KNIGHT),
                    Some(BLACK_ROOK),
                ],
            ],
            white_king_pos: Position::from_str("e1"),
            black_king_pos: Position::from_str("e8"),
            en_passant_intercept_pos: None,
            has_white_left_rook_moved: true,
            has_white_right_rook_moved: true,
            has_white_king_moved: true,
            has_black_left_rook_moved: true,
            has_black_right_rook_moved: true,
            has_black_king_moved: true,
        }
    }
}
