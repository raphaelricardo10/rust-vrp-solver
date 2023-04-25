use crate::solvers::solution::Solution;
use super::individual::Individual;

#[derive(Default)]
pub(crate) struct Population {
    pub(super) individuals: Vec<Individual>,
}

impl From<&[Solution]> for Population {
    fn from(solutions: &[Solution]) -> Self {
        Self {
            individuals: solutions
                .iter()
                .map(|solution| Individual::new(solution.routes.values().cloned().collect()))
                .collect(),
        }
    }
}

impl Population {
    #[allow(dead_code)]
    pub(super) fn new(individuals: Vec<Individual>) -> Self {
        Self { individuals }
    }

    pub(super) fn get_k_bests(&self, k: usize) -> &[Individual] {
        &self.individuals[..k]
    }
}
