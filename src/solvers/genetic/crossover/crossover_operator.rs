use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService, solvers::genetic::individual::Individual,
};

pub(crate) trait CrossoverOperator {
    fn run<R>(
        &self,
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized;

    fn max_of_tries(&self) -> u8;
}
