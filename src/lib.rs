use std::fmt;
use std::char;
use std::str;
use std::ops::Range;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    column: u8,
    row: u8,
}

impl str::FromStr for Position {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut char_iter = code.chars();
        let column = (char_iter.next().unwrap() as u8) - 97;
        let row = (char_iter.next().unwrap() as u8) - 49;
        let allowed_range: Range<u8> = 0..8;

        if !(allowed_range.contains(&column) & &allowed_range.contains(&row)) {
            panic!("illegal value for Position: {}", code);
        }

        Ok(Position {
            column,
            row,
        })
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", (self.column + 97) as char, (self.row+49) as char)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    from: Position,
    to: Position,
    pawn_promo: PawnPromotion,
}

impl str::FromStr for Move {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Ok(Move {
            from: Position::from_str(&code[0..2]).unwrap(),
            to: Position::from_str(&code[3..5]).unwrap(),
            pawn_promo: PawnPromotion::from_str(&code[2..3]).unwrap()
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.from, self.pawn_promo, self.to)
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PromotionType {
    Rook,
    Knight,
    Bishop,
    Queen,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PawnPromotion {
    Yes(PromotionType),
    No,
}

impl str::FromStr for PawnPromotion {
    type Err = ();

    fn from_str(s: &str) -> Result<PawnPromotion, Self::Err> {
        match s {
            "-" => Ok(PawnPromotion::No),
            "Q" => Ok(PawnPromotion::Yes(PromotionType::Queen)),
            "R" => Ok(PawnPromotion::Yes(PromotionType::Rook)),
            "K" => Ok(PawnPromotion::Yes(PromotionType::Knight)),
            "B" => Ok(PawnPromotion::Yes(PromotionType::Bishop)),
            _ => panic!("unknown pawn promotion type"),
        }
    }
}

impl fmt::Display for PawnPromotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            PawnPromotion::No => "-",
            PawnPromotion::Yes(PromotionType::Queen) => "Q",
            PawnPromotion::Yes(PromotionType::Rook) => "R",
            PawnPromotion::Yes(PromotionType::Knight) => "K",
            PawnPromotion::Yes(PromotionType::Bishop) => "B",
        };
        write!(f, "{}", code)
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


#[derive(Debug)]
struct GameState {
    board: [[Option<Figure>; 8]; 8],
    next_turn_by: Color,
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
            next_turn_by: Color::White,
            white_king_pos: "e1".parse::<Position>().unwrap(),
            black_king_pos: "e8".parse::<Position>().unwrap(),
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
