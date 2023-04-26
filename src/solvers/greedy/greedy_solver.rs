use std::fmt::Display;

use crate::solvers::{solution::Solution, solver::Solver};

pub(super) trait GreedySolver<R, S, T>
where
    R: Copy,
    S: Display,
    T: PartialOrd + Display,
{
    fn before_solving_callback(&mut self) {}
    fn after_solving_callback(&mut self) {}
    fn choose_candidate(&mut self, sequence_id: R, candidate_id: S);

    fn get_solution(&self) -> Solution;
    fn get_all_sequences(&self) -> Box<dyn Iterator<Item = R> + '_>;
    fn get_all_candidates(&self, sequence_id: R) -> Box<dyn Iterator<Item = (S, T)> + '_>;
    fn stop_condition_met(&self) -> bool;

    fn get_best_candidate(&self, sequence_id: R) -> Option<(S, T)> {
        self.get_all_candidates(sequence_id).min_by(
            |(first_candidate_id, first_candidate_cost), (second_candidate_id, second_candidate_cost)| {
                first_candidate_cost
                    .partial_cmp(second_candidate_cost)
                    .unwrap_or_else(|| {
                        panic!("it should be possible to compare the costs {} and {}, from candidates {} and {} respectively",
                        first_candidate_cost, second_candidate_cost, first_candidate_id, second_candidate_id
                    )})
            },
        )
    }

    fn run_iteration(&mut self) {
        let sequence_ids: Vec<R> = self.get_all_sequences().collect();

        for sequence_id in sequence_ids {
            if let Some((candidate_id, _)) = self.get_best_candidate(sequence_id) {
                self.choose_candidate(sequence_id, candidate_id);
            }
        }
    }
}

impl<T> Solver for T
where
    T: GreedySolver<u32, u32, f32>,
{
    fn solve(&mut self) -> Solution {
        self.before_solving_callback();

        while !self.stop_condition_met() {
            self.run_iteration();
        }

        self.after_solving_callback();

        self.get_solution()
    }
}
