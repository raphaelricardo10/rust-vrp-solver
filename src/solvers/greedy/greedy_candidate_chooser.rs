use crate::solvers::{
    sequential::sequential_solver::{
        CandidateChooser, SequentialSolver, SequentialSolverParameters,
    },
    solver::SolverCallbacks,
};

#[derive(Clone, Copy)]
pub struct GreedyCandidateChooser;

impl<P> CandidateChooser<P> for GreedyCandidateChooser
where
    P: SequentialSolverParameters + SequentialSolver<P> + SolverCallbacks + ?Sized,
{
    fn get_best_candidate(
        &self,
        candidates: Box<dyn Iterator<Item = (P::CandidateId, P::Cost)> + '_>,
    ) -> Option<<P as SequentialSolverParameters>::CandidateId> {
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
