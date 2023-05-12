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
    fn get_best_candidate(&self, sequence_id: P::SequenceId) -> Option<(P::CandidateId, P::Cost)>;
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
}

trait IterationAlgorithm<S, T>
where
    S: Solution,
    T: SequentialSolverParameters + CandidateChooser<S, T> + SequentialSolver<S, T> + ?Sized,
{
    fn run_iteration(&mut self);
}

impl<S, T> IterationAlgorithm<S, T> for T
where
    S: Solution,
    T: SequentialSolverParameters + CandidateChooser<S, T> + SequentialSolver<S, T> + ?Sized,
{
    fn run_iteration(&mut self) {
        let sequence_ids: Vec<T::SequenceId> = self.get_all_sequences().collect();

        for sequence_id in sequence_ids {
            if let Some((candidate_id, _)) = self.get_best_candidate(sequence_id) {
                self.choose_candidate(sequence_id, candidate_id);
            }
        }
    }
}

impl<S, T> Solver<S> for T
where
    S: Solution,
    T: SequentialSolverParameters
        + CandidateChooser<S, T>
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
