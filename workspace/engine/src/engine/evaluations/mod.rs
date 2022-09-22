use std::cmp::Ordering;
use crate::base::Move;
use crate::engine::evaluations::frontend::MoveEvaluation;

pub mod frontend;
pub(crate) mod testing;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Evaluation {
    WinIn(u8),
    Numeric(f32),
    Draw(DrawReason),
    LoseIn(u8, f32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DrawReason {
    StaleMate,
    InsufficientMaterial,
    ThreeTimesRepetition,
    NoChangeIn50Moves,
}

pub const MIN_EVALUATION: Evaluation = Evaluation::LoseIn(0, f32::MIN);
pub const MAX_EVALUATION: Evaluation = Evaluation::WinIn(0);

impl Eq for Evaluation {}

impl Ord for Evaluation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
                            Some(other_draw_reason.cmp(self_draw_reason))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EvaluatedMove {
    pub a_move: Move,
    pub evaluation: MoveEvaluation,
}

//------------------------------Tests------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    //♔♕♗♘♖♙♚♛♝♞♜♟

    #[test]
    fn test_sort_evaluations() {
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
        actual_sorted_evaluations.sort_unstable_by(
            Evaluation::cmp
        );

        let expected_sorted_evaluations: Vec<Evaluation> = vec![
            e16, e15, e14, e13, e12, e11, e10, e09, e08, e07, e06, e05, e04, e03, e02, e01,
        ];

        assert_eq!(actual_sorted_evaluations, expected_sorted_evaluations);
    }

    #[rstest(
    larger_eval, smaller_eval,
    case(Evaluation::WinIn(1), Evaluation::WinIn(3)),
    case(Evaluation::WinIn(1), Evaluation::LoseIn(3, 0.0)),
    ::trace //This leads to the arguments being printed in front of the test result.
    )]
    fn test_larger_than(
        larger_eval: Evaluation,
        smaller_eval: Evaluation,
    ) {
        assert!(
            larger_eval > smaller_eval,
            "{:?} is supposed to be larger than {:?}",
            larger_eval,
            smaller_eval,
        );
    }
}
