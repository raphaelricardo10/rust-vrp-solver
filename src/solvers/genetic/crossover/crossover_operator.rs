use rand::Rng;

use crate::{
    services::distance::distance_service::DistanceService, solvers::genetic::individual::Individual,
};

pub trait CrossoverOperator<R: Rng + ?Sized> {
    fn run(
        &self,
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>;

    fn max_of_tries(&self) -> u8;
}
