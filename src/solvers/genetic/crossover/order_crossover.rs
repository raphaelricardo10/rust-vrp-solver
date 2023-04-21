use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService, solvers::genetic::individual::Individual,
};

use super::crossover_operator::CrossoverOperator;

#[derive(Clone)]
pub(crate) struct OrderCrossover {
    max_of_tries: u8,
}

impl CrossoverOperator for OrderCrossover {
    fn run<R>(
        &self,
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized,
    {
        parent1.crossover_with(parent2, rng, distance_service)
    }

    fn max_of_tries(&self) -> u8 {
        self.max_of_tries
    }
}

impl OrderCrossover {
    pub(crate) fn new(max_of_tries: u8) -> Self {
        Self { max_of_tries }
    }
}
