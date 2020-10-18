use super::*;

/**
 * Different from Evaluation, here the u8 in WinIn and LoseIn are full moves ahead, not half moves
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
    pub(crate) fn from(eval: &Evaluation) -> RoughEvaluation {
        match eval {
            Evaluation::WinIn(depth) => RoughEvaluation::WinIn(*depth/2),
            Evaluation::LoseIn(depth, _) => RoughEvaluation::LoseIn(*depth/2),
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