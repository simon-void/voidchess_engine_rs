use std::cmp::Ordering;
use crate::game::StoppedReason;
use crate::base::Move;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Evaluation {
    WinIn(u8),
    Numeric(f32),
    Draw(DrawReason),
    LoseIn(u8, f32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DrawReason {
    NoChangeIn50Moves,
    ThreeTimesRepetition,
    InsufficientMaterial,
    StaleMate,
}

pub fn sort_evaluations_best_first(eval1: &Evaluation, eval2: &Evaluation) -> Ordering {
    (*eval2).partial_cmp(eval1).unwrap()
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
                Evaluation::LoseIn(_, _) => 1,
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
                    Evaluation::LoseIn(self_loose_in_nr, self_num_eval) => {
                        if let Evaluation::LoseIn(other_loose_in_nr, other_num_eval) = other {
                            let loose_in_nr_order = self_loose_in_nr.partial_cmp(other_loose_in_nr);
                            if let Some(Ordering::Equal) = loose_in_nr_order {
                                self_num_eval.partial_cmp(other_num_eval)
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

/**
 * for testing
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoughEvaluation {
    WinIn(u8),
    PositiveNumeric, // original numeric evaluation was >= 0.0
    NegativeNumeric, // original numeric evaluation was < 0.0
    Draw(DrawReason),
    LoseIn(u8),
}

impl RoughEvaluation {
    pub(crate) fn of(eval: &Evaluation) -> RoughEvaluation {
        match eval {
            Evaluation::WinIn(depth) => RoughEvaluation::WinIn(*depth),
            Evaluation::LoseIn(depth, _) => RoughEvaluation::LoseIn(*depth),
            Evaluation::Draw(reason) => RoughEvaluation::Draw(*reason),
            Evaluation::Numeric(value) => {
                if *value>0.0 {
                    RoughEvaluation::PositiveNumeric
                } else {
                    RoughEvaluation::NegativeNumeric
                }
            }
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

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[test]
    fn test_sort_evaluations_best_first() {
        let e01 = Evaluation::WinIn(1);
        let e02 = Evaluation::WinIn(3);
        let e03 = Evaluation::Numeric(3.4);
        let e04 = Evaluation::Numeric(0.4);
        let e05 = Evaluation::Numeric(0.0);
        let e06 = Evaluation::Draw(DrawReason::StaleMate);
        let e07 = Evaluation::Draw(DrawReason::InsufficientMaterial);
        let e08 = Evaluation::Draw(DrawReason::ThreeTimesRepetition);
        let e09 = Evaluation::Draw(DrawReason::NoChangeIn50Moves);
        let e10 = Evaluation::Numeric(-0.2);
        let e11 = Evaluation::Numeric(-3.0);
        let e12 = Evaluation::LoseIn(4, -5.0);
        let e13 = Evaluation::LoseIn(3, 5.0);
        let e14 = Evaluation::LoseIn(3, 3.0);
        let e15 = Evaluation::LoseIn(3, -7.0);
        let e16 = Evaluation::LoseIn(1, 1.0);

         let mut actual_sorted_evaluations: Vec<Evaluation> = vec![
             e06, e02, e16, e04, e01, e07, e14, e08, e03, e09, e05, e15, e13, e11, e10, e12,
         ];
        actual_sorted_evaluations.sort_unstable_by(sort_evaluations_best_first);

        let expected_sorted_evaluations: Vec<Evaluation> = vec![
            e01, e02, e03, e04, e05, e06, e07, e08, e09, e10, e11, e12, e13, e14, e15, e16,
        ];

        assert_eq!(actual_sorted_evaluations, expected_sorted_evaluations);
    }
}
