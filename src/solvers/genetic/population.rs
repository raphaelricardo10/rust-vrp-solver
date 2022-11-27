use rand::Rng;

use crate::services::route::route_service::RouteService;

use super::individual::Individual;

#[derive(Default)]
pub(crate) struct Population {
    pub(crate) individuals: Vec<Individual>,
}

impl Population {
    #[allow(dead_code)]
    pub(super) fn new(individuals: Vec<Individual>) -> Self {
        Self { individuals }
    }

    pub(super) fn get_k_bests(&self, k: usize) -> &[Individual] {
        &self.individuals[..k]
    }

    pub(crate) fn from_random<R>(size: u32, rng: &mut R, route_service: &mut RouteService) -> Self
    where
        R: Rng + ?Sized,
    {
        let mut population = Self::default();

        for _ in 0..size {
            let individual = Individual::from_random(rng, route_service);
            population.individuals.push(individual);

            route_service.reset();
        }

        population
    }
}
