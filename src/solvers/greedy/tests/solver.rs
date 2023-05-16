use std::collections::{BTreeMap, BTreeSet};

use crate::solvers::{
    greedy::greedy_candidate_chooser::GreedyCandidateChooser,
    sequential::sequential_solver::{
        CandidateChooser, SequentialSolver, SequentialSolverParameters, SolutionGetter,
    },
    solver::SolverCallbacks,
};

use super::solution::TestSolution;

pub(super) struct TestGreedySolver {
    pub(super) candidates: BTreeSet<u32>,
    pub(super) sequences: BTreeSet<u32>,
    pub(super) costs: BTreeMap<u32, u32>,
    pub(super) solution: TestSolution,
    pub(super) candidate_chooser: GreedyCandidateChooser,
}

impl SolverCallbacks for TestGreedySolver {}

impl TestGreedySolver {
    #[allow(dead_code)]
    pub(super) fn new(candidates: &[u32], sequences: &[u32], costs: &[(u32, u32)]) -> Self {
        Self {
            candidates: candidates.iter().copied().collect(),
            sequences: sequences.iter().copied().collect(),
            costs: costs.iter().copied().collect(),
            solution: Default::default(),
            candidate_chooser: GreedyCandidateChooser {},
        }
    }
}

impl SequentialSolverParameters for TestGreedySolver {
    type SequenceId = u32;
    type CandidateId = u32;
    type Cost = u32;
}

impl SolutionGetter<TestSolution> for TestGreedySolver {
    fn get_solution(&self) -> TestSolution {
        self.solution.clone()
    }
}

impl SequentialSolver<TestGreedySolver> for TestGreedySolver {
    fn choose_candidate(&mut self, sequence_id: u32, candidate_id: u32) {
        self.solution.insert(
            sequence_id,
            candidate_id,
            *self.costs.get(&candidate_id).unwrap(),
        );

        match self.candidates.remove(&candidate_id) {
            true => (),
            false => panic!("the candidate {} should be available", candidate_id),
        };
    }

    fn get_all_sequences(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.sequences.iter().copied())
    }

    fn get_all_candidates(&self, _: u32) -> Box<dyn Iterator<Item = (u32, u32)> + '_> {
        Box::new(
            self.candidates
                .iter()
                .map(|candidate_id| (*candidate_id, *self.costs.get(candidate_id).unwrap())),
        )
    }

    fn stop_condition_met(&self) -> bool {
        self.candidates.is_empty()
    }

    fn get_candidate_chooser(&self) -> &dyn CandidateChooser<TestGreedySolver> {
        &self.candidate_chooser
    }
}
