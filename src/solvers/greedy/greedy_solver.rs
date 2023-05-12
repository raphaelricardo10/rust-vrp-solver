use crate::solvers::{
    sequential_solver::{CandidateChooser, SequentialSolver, SequentialSolverParameters},
    solution::Solution,
    solver::SolverCallbacks,
};

pub trait GreedySolver {}

impl<S, T> CandidateChooser<S, T> for T
where
    S: Solution,
    T: SequentialSolverParameters
        + GreedySolver
        + SequentialSolver<S, T>
        + SolverCallbacks
        + ?Sized,
{
    fn get_best_candidate(&self, sequence_id: T::SequenceId) -> Option<(T::CandidateId, T::Cost)> {
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
}
