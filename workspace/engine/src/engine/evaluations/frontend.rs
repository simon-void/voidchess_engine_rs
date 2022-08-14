use super::*;
use crate::engine::evaluations::frontend::MoveEvaluation::*;
use crate::engine::evaluations::frontend::GameEvaluation::*;

#[derive(Debug, Clone)]
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
    EngineGetsCheckMatedIn(u8, f32),
}

impl PartialEq for GameEvaluation {
    fn eq(&self, other: &Self) -> bool {
        match self {
            GameEnded(game_end_result) => {
                if let GameEnded(other_game_end_result) = other {
                    game_end_result == other_game_end_result
                } else {
                    false
                }
            }
            MoveToPlay(self_move, move_eval) => {
                if let MoveToPlay(other_move, other_move_eval) = other {
                    self_move == other_move && move_eval == other_move_eval
                } else {
                    false
                }
            }
            Err(msg) => {
                if let Err(other_msg) = other {
                    msg == other_msg
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for GameEvaluation {}

impl PartialEq for MoveEvaluation {
    fn eq(&self, other: &Self) -> bool {
        // this is clearly a hack, but the non-hacky version failed for some reason
        let debug_self = format!("{:?}", self);
        let debug_other = format!("{:?}", other);
        debug_self == debug_other
    }
}

impl Eq for MoveEvaluation {}

impl MoveEvaluation {
    pub(crate) fn from(eval: &Evaluation) -> MoveEvaluation {
        match eval {
            Evaluation::WinIn(number_of_halfmoves) => { EngineCheckMatesIn((number_of_halfmoves-1)/2)}
            Evaluation::LoseIn(number_of_halfmoves, num_eval) => {
                let full_moves = number_of_halfmoves/2;
                EngineGetsCheckMatedIn(full_moves, *num_eval)
            }
            Evaluation::Draw(reason) => {Draw(*reason)}
            Evaluation::Numeric(value) => {Numeric(*value)}
        }
    }
}

impl Ord for MoveEvaluation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for MoveEvaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn rank(this: &MoveEvaluation) -> usize {
            match this {
                EngineGetsCheckMatedIn(_, _) => 1,
                Numeric(x) if x.is_sign_negative() => 2,
                Draw(_) => 3,
                Numeric(_) => 4,
                EngineCheckMatesIn(_) => 5,
            }
        }

        let rank_order: Option<Ordering> = rank(self).partial_cmp(&rank(other));
        match rank_order {
            Some(Ordering::Equal) => {
                match self {
                    MoveEvaluation::EngineCheckMatesIn(self_win_in_nr) => {
                        if let MoveEvaluation::EngineCheckMatesIn(other_win_in_nr) = other {
                            other_win_in_nr.partial_cmp(self_win_in_nr)
                        } else {
                            None
                        }
                    },
                    MoveEvaluation::Numeric(self_num_eval) => {
                        if let MoveEvaluation::Numeric(other_num_eval) = other {
                            self_num_eval.partial_cmp(other_num_eval)
                        } else {
                            None
                        }
                    }
                    MoveEvaluation::Draw(self_draw_reason) => {
                        if let MoveEvaluation::Draw(other_draw_reason) = other {
                            Some(other_draw_reason.cmp(self_draw_reason))
                        } else {
                            None
                        }
                    },
                    MoveEvaluation::EngineGetsCheckMatedIn(self_loose_in_nr, self_num_eval) => {
                        if let MoveEvaluation::EngineGetsCheckMatedIn(other_loose_in_nr, other_num_eval) = other {
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
