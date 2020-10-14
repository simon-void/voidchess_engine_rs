use std::cmp::Ordering;
use crate::game::StoppedReason;
use crate::base::Move;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Evaluation {
    WinIn(u8),
    Numeric(f32),
    Draw(DrawReason),
    LooseIn(u8, f32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DrawReason {
    StaleMate,
    InsufficientMaterial,
    ThreeTimesRepetition,
    NoChangeIn50Moves,
}

impl DrawReason {
    pub fn from(reason: StoppedReason) -> DrawReason {
        match reason {
            StoppedReason::InsufficientMaterial => DrawReason::InsufficientMaterial,
            StoppedReason::ThreeTimesRepetition => DrawReason::ThreeTimesRepetition,
            StoppedReason::NoChangeIn50Moves => DrawReason::ThreeTimesRepetition,
        }
    }
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn rank(this: &Evaluation) -> usize {
            match this {
                Evaluation::LooseIn(_, _) => 1,
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
                    Evaluation::Numeric(self_num_eval) => {
                        if let Evaluation::Numeric(other_num_eval) = other {
                            self_num_eval.partial_cmp(other_num_eval)
                        } else {
                            None
                        }
                    }
                    Evaluation::Draw(self_draw_reason) => {
                        if let Evaluation::Draw(other_draw_reason) = other {
                            Some(self_draw_reason.cmp(other_draw_reason))
                        } else {
                            None
                        }
                    },
                    Evaluation::LooseIn(self_loose_in_nr, self_num_eval) => {
                        if let Evaluation::LooseIn(other_loose_in_nr, other_num_eval) = other {
                            let loose_in_nr_order = self_loose_in_nr.partial_cmp(other_loose_in_nr);
                            if let Some(Ordering::Equal) = loose_in_nr_order {
                                self_loose_in_nr.partial_cmp(other_loose_in_nr)
                            } else {
                                loose_in_nr_order
                            }
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

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::*;
    // use crate::engine::evaluations::*;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    // #[test]
    // fn test_Evaluation_order() {
    //      let evaluations: Vec<Evaluation> = vec![
    //      ];
    // }
}
