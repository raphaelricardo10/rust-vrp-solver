use std::marker::PhantomData;

use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService, solvers::genetic::individual::Individual,
};

use super::crossover_operator::CrossoverOperator;

pub(crate) struct Offspring<'a, R: Rng + ?Sized, T: CrossoverOperator<R> + ?Sized> {
    crossover_op: &'a T,
    pub(super) parent1: Individual,
    pub(super) parent2: Individual,
    pub(crate) individual: Individual,
    _t: PhantomData<(&'a T, R)>,
}

impl<'a, R: Rng + ?Sized, T: CrossoverOperator<R> + ?Sized> Offspring<'a, R, T> {
    pub(crate) fn new(parent1: Individual, parent2: Individual, crossover_op: &'a T) -> Self {
        Self {
            parent1,
            parent2,
            crossover_op,
            individual: Default::default(),
            _t: PhantomData,
        }
    }

    pub(crate) fn try_to_evolve(
        &mut self,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<()> {
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
