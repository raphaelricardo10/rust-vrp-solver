use crate::solvers::{
    sequential_solver::{CandidateChooser, SequentialSolver, SequentialSolverParameters},
    solution::Solution,
    solver::SolverCallbacks,
};

#[derive(Clone, Copy)]
pub struct GreedySolver;

impl<S, T> CandidateChooser<S, T> for GreedySolver
where
    S: Solution,
    T: SequentialSolverParameters
        + SequentialSolver<S, T>
        + SolverCallbacks
        + ?Sized,
{
    fn get_best_candidate(
        &self,
        candidates: Box<
            dyn Iterator<
                Item = (
                    <T as SequentialSolverParameters>::CandidateId,
                    <T as SequentialSolverParameters>::Cost,
                ),
            > + '_,
        >,
    ) -> Option<<T as SequentialSolverParameters>::CandidateId> {
        let chosen = candidates.min_by(
            |(first_candidate_id, first_candidate_cost), (second_candidate_id, second_candidate_cost)| {
                first_candidate_cost
                    .partial_cmp(second_candidate_cost)
                    .unwrap_or_else(|| {
                        panic!("it should be possible to compare the costs {} and {}, from candidates {} and {} respectively",
                        first_candidate_cost, second_candidate_cost, first_candidate_id, second_candidate_id
                    )})
            },
        )?;

        Some(chosen.0)
    }
}
