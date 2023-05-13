use std::fmt::Display;

use super::{
    solution::Solution,
    solver::{Solver, SolverCallbacks},
};

pub trait SequentialSolverParameters {
    type CandidateId: Display;
    type SequenceId: Copy + Display;
    type Cost: PartialOrd + Display;
}

pub trait CandidateChooser<S, P>
where
    S: Solution,
    P: SequentialSolverParameters + ?Sized,
{
    fn get_best_candidate(
        &self,
        candidates: Box<dyn Iterator<Item = (P::CandidateId, P::Cost)> + '_>,
    ) -> Option<P::CandidateId>;
}

pub trait SequentialSolver<S, P>
where
    S: Solution,
    P: SequentialSolverParameters + ?Sized,
{
    fn choose_candidate(&mut self, sequence_id: P::SequenceId, candidate_id: P::CandidateId);

    fn get_solution(&self) -> S;
    fn stop_condition_met(&self) -> bool;
    fn get_all_sequences(&self) -> Box<dyn Iterator<Item = P::SequenceId> + '_>;

    fn get_all_candidates(
        &self,
        sequence_id: P::SequenceId,
    ) -> Box<dyn Iterator<Item = (P::CandidateId, P::Cost)> + '_>;

    fn get_candidate_chooser(&self) -> &dyn CandidateChooser<S, P>;

    fn run_iteration(&mut self) {
        let sequence_ids: Vec<P::SequenceId> = self.get_all_sequences().collect();

        for sequence_id in sequence_ids {
            let candidates = self.get_all_candidates(sequence_id);
            match self.get_candidate_chooser().get_best_candidate(candidates) {
                Some(candidate_id) => self.choose_candidate(sequence_id, candidate_id),
                None => (),
            }
        }
    }
}

impl<S, T> Solver<S> for T
where
    S: Solution,
    T: SequentialSolverParameters
        + SequentialSolver<S, T>
        + SolverCallbacks,
{
    fn solve(&mut self) -> S {
        self.before_solving();

        while !self.stop_condition_met() {
            self.run_iteration();
        }

        self.after_solving();

        self.get_solution()
    }
}
