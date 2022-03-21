use crate::game::MoveStats;

pub static PRUNER_L0: Pruner = Pruner { half_step_depth: 0, half_step_depth_after_pawn_moved: 0, half_step_depth_after_check: 0, half_step_depth_after_figure_caught: 0 };
pub static PRUNER_L1: Pruner = Pruner { half_step_depth: 2, half_step_depth_after_pawn_moved: 2, half_step_depth_after_check: 2, half_step_depth_after_figure_caught: 2 };
pub static PRUNER_L2: Pruner = Pruner { half_step_depth: 4, half_step_depth_after_pawn_moved: 4, half_step_depth_after_check: 4, half_step_depth_after_figure_caught: 4 };
pub static PRUNER_L3: Pruner = Pruner { half_step_depth: 6, half_step_depth_after_pawn_moved: 6, half_step_depth_after_check: 6, half_step_depth_after_figure_caught: 6 };
pub static PRUNER_1_1_3_3: Pruner = Pruner { half_step_depth: 2, half_step_depth_after_pawn_moved: 3, half_step_depth_after_check: 6, half_step_depth_after_figure_caught: 6 };
pub static PRUNER_1_2_3_4: Pruner = Pruner { half_step_depth: 2, half_step_depth_after_pawn_moved: 4, half_step_depth_after_check: 6, half_step_depth_after_figure_caught: 8 };
pub static PRUNER_2_4_5_8: Pruner = Pruner { half_step_depth: 4, half_step_depth_after_pawn_moved: 8, half_step_depth_after_check: 10, half_step_depth_after_figure_caught: 16 };
pub static PRUNER_3_4_4_6: Pruner = Pruner { half_step_depth: 6, half_step_depth_after_pawn_moved: 8, half_step_depth_after_check: 8, half_step_depth_after_figure_caught: 12 };

#[derive(Debug, Copy, Clone)]
pub struct Pruner {
    half_step_depth: usize,
    half_step_depth_after_pawn_moved: usize,
    half_step_depth_after_check: usize,
    half_step_depth_after_figure_caught: usize,
}

impl Pruner {
    pub fn new(
        step_depth: usize,
        step_depth_after_pawn_moved: usize,
        step_depth_after_chess_moved: usize,
        step_depth_after_figure_caught: usize,
    ) -> Pruner {
        if step_depth > step_depth_after_pawn_moved {
            panic!(
                "step_depth [{}] mustn't be larger than step_depth_after_pawn_moved [{}]",
                step_depth, step_depth_after_pawn_moved
            );
        }
        if step_depth_after_pawn_moved > step_depth_after_figure_caught {
            panic!(
                "step_depth_after_pawn_moved [{}] mustn't be larger than step_depth_after_figure_caught [{}]",
                step_depth_after_pawn_moved, step_depth_after_figure_caught
            );
        }
        Pruner {
            half_step_depth: step_depth * 2,
            half_step_depth_after_pawn_moved: step_depth_after_pawn_moved * 2,
            half_step_depth_after_check: step_depth_after_chess_moved * 2,
            half_step_depth_after_figure_caught: step_depth_after_figure_caught * 2,
        }
    }
    pub fn should_stop_min_max_ing(
        &self,
        current_half_step: usize,
        new_stats: MoveStats,
        old_stats: MoveStats,
        is_check: bool,
        was_check: bool,
    ) -> bool {

        let should_continue = (current_half_step < self.half_step_depth) ||
            ((new_stats.did_move_pawn || old_stats.did_move_pawn) && current_half_step < self.half_step_depth_after_pawn_moved) ||
            ((is_check || was_check) && current_half_step < self.half_step_depth_after_check) ||
            (new_stats.did_catch_figure && current_half_step < self.half_step_depth_after_figure_caught);
        !should_continue
    }
}