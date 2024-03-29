use std::fmt::Display;

use crate::solvers::{
    solution::Solution,
    solver::{Solver, SolverCallbacks},
};

pub trait SequentialSolverParameters {
    type CandidateId: Copy + Display;
    type SequenceId: Copy + Display;
    type Cost: PartialOrd + Display;
}

pub trait SolutionGetter<S>
where
    S: Solution,
{
    fn get_solution(&self) -> S;
}

pub trait CandidateChooser<P>
where
    P: SequentialSolverParameters + ?Sized,
{
    fn get_best_candidate(
        &self,
        candidates: Box<dyn Iterator<Item = (P::CandidateId, P::Cost)> + '_>,
    ) -> Option<P::CandidateId>;
}

pub trait SequentialSolver<P>
where
    P: SequentialSolverParameters + ?Sized,
{
    fn choose_candidate(&mut self, sequence_id: P::SequenceId, candidate_id: P::CandidateId);

    fn stop_condition_met(&self) -> bool;
    fn get_all_sequences(&self) -> Box<dyn Iterator<Item = P::SequenceId> + '_>;

    fn get_all_candidates(
        &self,
        sequence_id: P::SequenceId,
    ) -> Box<dyn Iterator<Item = (P::CandidateId, P::Cost)> + '_>;

    fn get_candidate_chooser(&self) -> &dyn CandidateChooser<P>;

    fn run_iteration(&mut self) {
        let sequence_ids: Vec<P::SequenceId> = self.get_all_sequences().collect();

        for sequence_id in sequence_ids {
            let candidates = self.get_all_candidates(sequence_id);
            let candidate_id = self.get_candidate_chooser().get_best_candidate(candidates);

            if let Some(candidate_id) = candidate_id {
                self.choose_candidate(sequence_id, candidate_id);
            }
        }
    }
}

impl<S, T> Solver<S> for T
where
    S: Solution,
    T: SequentialSolverParameters + SequentialSolver<T> + SolverCallbacks + SolutionGetter<S>,
{
    fn solve(&mut self) -> S {
        self.before_solving();

        while !self.stop_condition_met() {
            self.run_iteration();
        }

        self.after_solving();

        let solution = self.get_solution();

        self.on_exit();

        solution
    }
}
