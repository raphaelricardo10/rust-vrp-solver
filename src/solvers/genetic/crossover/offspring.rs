use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService, solvers::genetic::individual::Individual,
};

use super::crossover_operator::CrossoverOperator;

pub(crate) struct Offspring<T: CrossoverOperator<T>> {
    crossover_op: T,
    pub(super) parent1: Individual,
    pub(super) parent2: Individual,
    pub(crate) individual: Individual,
}

impl<T: CrossoverOperator<T> + Clone> Offspring<T> {
    pub(crate) fn new(parent1: Individual, parent2: Individual, crossover_op: T) -> Self {
        Self {
            parent1,
            parent2,
            crossover_op,
            individual: Default::default(),
        }
    }

    pub(crate) fn try_to_evolve<R>(
        &mut self,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<()>
    where
        R: Rng + ?Sized,
    {
        for _ in 0..self.crossover_op.max_of_tries() {
            self.individual = self.crossover_op.run(
                self.parent1.clone(),
                self.parent2.clone(),
                rng,
                distance_service,
            )?;

            if self.has_evolved() {
                return Some(());
            }
        }

        None
    }

    pub(crate) fn has_evolved(&self) -> bool {
        if self.individual.fitness >= self.parent1.fitness {
            return false;
        }

        if self.individual.fitness >= self.parent2.fitness {
            return false;
        }

        true
    }
}
