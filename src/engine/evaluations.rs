use std::cmp::Ordering;
use crate::Move;
use crate::base::EXPECTED_MAX_NUMBER_OF_MOVES;
use tinyvec::TinyVec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Evaluation {
    WinIn(u8),
    Numeric(f64),
    Draw(DrawReason),
    LooseIn(u8),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DrawReason {
    StaleMate,
    InsufficientMaterial,
    ThreeTimesRepetition,
    NoChangeIn50Moves,
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn rank(this: &Evaluation) -> usize {
            match this {
                Evaluation::LooseIn(_) => 1,
                Evaluation::Numeric(x) if x.is_sign_negative() => 2,
                Evaluation::Draw(_) => 3,
                Evaluation::Numeric(_) => 4,
                Evaluation::WinIn(_) => 5,
            }
        }

        let rank_order: Option<Ordering> = rank(self).partial_cmp(&rank(other));
        match rank_order {
            Some(Ordering::Equal) => {
                match self {
                    Evaluation::WinIn(self_win_in_nr) => {
                        if let Evaluation::WinIn(other_win_in_nr) = other {
                            other_win_in_nr.partial_cmp(self_win_in_nr)
                        } else {
                            None
                        }
                    },
                    Evaluation::Numeric(self_eval) => {
                        if let Evaluation::Numeric(other_eval) = other {
                            self_eval.partial_cmp(other_eval)
                        } else {
                            None
                        }
                    },
                    Evaluation::Draw(self_draw_reason) => {
                        if let Evaluation::Draw(other_draw_reason) = other {
                            Some(self_draw_reason.cmp(other_draw_reason))
                        } else {
                            None
                        }
                    },
                    Evaluation::LooseIn(self_loose_in_nr) => {
                        if let Evaluation::LooseIn(other_loose_in_nr) = other {
                            self_loose_in_nr.partial_cmp(other_loose_in_nr)
                        } else {
                            None
                        }
                    },
                }
            },
            _ => rank_order,
        }
    }
}

pub struct EvaluatedMove {
    pub a_move: Move,
    pub evaluation: Evaluation,
}

// impl Default for EvaluatedMove {
//     fn default() -> Self {
//         EvaluatedMove {
//             a_move: Move::default(),
//             evaluation: Evaluation::Draw(DrawReason::NoChangeIn50Moves)
//         }
//     }
// }
//
// pub struct EvaluatedMoveArray {
//     array: [EvaluatedMove; 80]
// }
//
// impl tinyvec::Array for EvaluatedMoveArray {
//     type Item = EvaluatedMove;
//     const CAPACITY: usize = EXPECTED_MAX_NUMBER_OF_MOVES;
//
//     fn as_slice(&self) -> &[Self::Item] {
//         &self.array
//     }
//
//     fn as_slice_mut(&mut self) -> &mut [Self::Item] {
//         &mut self.array
//     }
//
//     fn default() -> Self {
//         EvaluatedMoveArray {
//             array: [EvaluatedMove::default(); EXPECTED_MAX_NUMBER_OF_MOVES]
//         }
//     }
// }
//
// pub type EvaluatedMoves = TinyVec<EvaluatedMoveArray>;