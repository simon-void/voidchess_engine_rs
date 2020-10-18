use super::*;
use crate::engine::evaluations::frontend::MoveEvaluation::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameEvaluation {
    GameEnded(GameEndResult),
    MoveToPlay(Move, MoveEvaluation),
    Err(String),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameEndResult {
    EngineWon,
    EngineLost,
    Draw(DrawReason),
}

/**
 * Different from Evaluation, here the u8 in WinIn and LoseIn are full moves ahead, not half moves.
 * Also while a EngineCheckMatesIn(0) (=CheckMate) is possible, a EngineGetsCheckMatedIn(0) isn't,
 * since either the Engine already lost, in which case GameEnded(EngineLost) will be returned or
 * the opponent still has to play the mating move, so EngineGetsCheckMatedIn(1) is appropriate.
 */
#[derive(Debug, Copy, Clone)]
pub enum MoveEvaluation {
    EngineCheckMatesIn(u8),
    Numeric(f32),
    Draw(DrawReason),
    EngineGetsCheckMatedIn(u8),
}

impl PartialEq for MoveEvaluation {
    fn eq(&self, other: &Self) -> bool {
        match self {
            EngineCheckMatesIn(nr_of_fullmoves) => {
                if let EngineGetsCheckMatedIn(other_nr_of_fullmoves) = other {
                    nr_of_fullmoves == other_nr_of_fullmoves
                } else {
                    false
                }
            }
            Numeric(value) => {
                if let Numeric(other_value) = other {
                    value.partial_cmp(other_value).unwrap() == Ordering::Equal
                } else {
                    false
                }
            }
            Draw(reason) => {
                if let Draw(other_reason) = other {
                    reason == other_reason
                } else {
                    false
                }
            }
            EngineGetsCheckMatedIn(nr_of_fullmoves) => {
                if let EngineGetsCheckMatedIn(other_nr_of_fullmoves) = other {
                    nr_of_fullmoves == other_nr_of_fullmoves
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for MoveEvaluation {}

impl MoveEvaluation {
    pub(crate) fn from(eval: &Evaluation) -> MoveEvaluation {
        match eval {
            Evaluation::WinIn(number_of_halfmoves) => { EngineCheckMatesIn(number_of_halfmoves/2)}
            Evaluation::LoseIn(number_of_halfmoves, _) => {
                let full_moves = number_of_halfmoves/2;
                if full_moves == 0 {
                    panic!("EngineGetsCheckMatedIn(0) has to be guarded against! Return GameEvaluation::GameEnded(GameEndResult::EngineLost) instead.");
                }
                EngineGetsCheckMatedIn(full_moves)
            }
            Evaluation::Draw(reason) => {Draw(*reason)}
            Evaluation::Numeric(value) => {Numeric(*value)}
        }
    }
}

// impl PartialEq for MoveEvaluation {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             MoveEvaluation::WinIn(win_in_full_moves) => {
//                 if let MoveEvaluation::WinIn(other_win_in_full_moves) = other {
//                     win_in_full_moves == other_win_in_full_moves
//                 } else {
//                     false
//                 }
//             }
//             MoveEvaluation::Numeric(value) => {
//                 if let MoveEvaluation::WinIn(other_win_in_full_moves) = other {
//                     win_in_full_moves == other_win_in_full_moves
//                 } else {
//                     false
//                 }}
//             MoveEvaluation::Draw(reason) => {
//                 if let MoveEvaluation::WinIn(other_win_in_full_moves) = other {
//                     win_in_full_moves == other_win_in_full_moves
//                 } else {
//                     false
//                 }}
//             MoveEvaluation::LoseIn(win_in_full_moves) => {
//                 if let MoveEvaluation::WinIn(other_win_in_full_moves) = other {
//                     win_in_full_moves == other_win_in_full_moves
//                 } else {
//                     false
//                 }}
//         }
//     }
// }
//
// impl Eq for MoveEvaluation {}