use rand::Rng;

use crate::{services::route::route_service::RouteService, solvers::solution::Solution};

use super::individual::Individual;

#[derive(Default)]
pub(crate) struct Population {
    pub(super) individuals: Vec<Individual>,
}

pub(crate) type RandomPopulationGeneratorParams<'a, 'b, R> = (u32, &'a mut R, &'b mut RouteService);

impl<'a, 'b, R: Rng + ?Sized> From<RandomPopulationGeneratorParams<'a, 'b, R>> for Population {
    fn from((size, rng, route_service): RandomPopulationGeneratorParams<R>) -> Self {
        let mut population = Self::default();

        for _ in 0..size {
            let individual = Individual::from((&mut *rng, &mut *route_service));
            population.individuals.push(individual);

            route_service.reset();
        }

        population
    }
}

impl From<&[Solution]> for Population {
    fn from(solutions: &[Solution]) -> Self {
        Self {
            individuals: solutions
                .iter()
                .map(|solution| {
                    Individual::new(solution.routes.values().cloned().collect())
                })
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
