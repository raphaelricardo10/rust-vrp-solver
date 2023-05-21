use super::individual::Individual;
use crate::solvers::vrp_solution::VrpSolution;

#[derive(Default, Clone)]
pub(crate) struct Population {
    pub(super) individuals: Vec<Individual>,
}

impl From<&[VrpSolution]> for Population {
    fn from(solutions: &[VrpSolution]) -> Self {
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

    pub(super) fn get_k_bests(&mut self, k: usize) -> &[Individual] {
        self.individuals.sort_by(|individual1, individual2| {
            individual1
                .fitness
                .partial_cmp(&individual2.fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        &self.individuals[..k]
    }
}
