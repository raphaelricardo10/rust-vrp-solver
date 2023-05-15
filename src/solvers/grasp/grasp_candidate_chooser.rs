use std::{cell::RefCell, cmp::min, ops::DerefMut};

use rand::{seq::SliceRandom, Rng};

use crate::solvers::sequential_solver::{CandidateChooser, SequentialSolverParameters};

pub struct GraspCandidateChooser<R: Rng + ?Sized> {
    pub rcl_size: usize,
    pub rng: RefCell<R>,
}

impl<P, R> CandidateChooser<P> for GraspCandidateChooser<R>
where
    P: SequentialSolverParameters,
    R: Rng + ?Sized,
{
    fn get_best_candidate(
        &self,
        candidates: Box<dyn Iterator<Item = (P::CandidateId, P::Cost)> + '_>,
    ) -> Option<P::CandidateId> {
        let mut candidates: Vec<(P::CandidateId, P::Cost)> = candidates.collect();

        candidates.sort_by(
            |(first_candidate_id, first_candidate_cost), (second_candidate_id, second_candidate_cost)| {
                first_candidate_cost
                    .partial_cmp(second_candidate_cost)
                    .unwrap_or_else(|| {
                        panic!("it should be possible to compare the costs {} and {}, from candidates {} and {} respectively",
                        first_candidate_cost, second_candidate_cost, first_candidate_id, second_candidate_id
                    )})
            },
        );

        let rcl_size = min(self.rcl_size, candidates.len());
        let mut rng = self.rng.borrow_mut();

        Some(candidates[0..rcl_size].choose(rng.deref_mut())?.0)
    }
}
