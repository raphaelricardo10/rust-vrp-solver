use std::fmt::Display;

use crate::solvers::{solver::Solver, vrp_solution::VrpSolution};

pub(super) trait GreedySolver {
    type SequenceId: Copy;
    type CandidateId: Display;
    type Cost: PartialOrd + Display;

    fn before_solving_callback(&mut self) {}
    fn after_solving_callback(&mut self) {}
    fn choose_candidate(&mut self, sequence_id: Self::SequenceId, candidate_id: Self::CandidateId);

    fn get_solution(&self) -> VrpSolution;
    fn get_all_sequences(&self) -> Box<dyn Iterator<Item = Self::SequenceId> + '_>;
    fn get_all_candidates(
        &self,
        sequence_id: Self::SequenceId,
    ) -> Box<dyn Iterator<Item = (Self::CandidateId, Self::Cost)> + '_>;
    fn stop_condition_met(&self) -> bool;

    fn get_best_candidate(
        &self,
        sequence_id: Self::SequenceId,
    ) -> Option<(Self::CandidateId, Self::Cost)> {
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
        let sequence_ids: Vec<Self::SequenceId> = self.get_all_sequences().collect();

        for sequence_id in sequence_ids {
            if let Some((candidate_id, _)) = self.get_best_candidate(sequence_id) {
                self.choose_candidate(sequence_id, candidate_id);
            }
        }
    }
}

impl<T: GreedySolver> Solver<VrpSolution> for T {

    fn solve(&mut self) -> VrpSolution {
        self.before_solving_callback();

        while !self.stop_condition_met() {
            self.run_iteration();
        }

        self.after_solving_callback();

        self.get_solution()
    }
}
