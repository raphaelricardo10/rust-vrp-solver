use rand::{seq::SliceRandom, thread_rng};

use crate::{domain::route::Route, services::route::route_service::RouteService};

use super::{individual::Individual, population::Population};

pub struct GeneticSolver {
    population_size: u32,
    population: Population,
}

impl GeneticSolver {
    pub fn new(population_size: u32, mut route_service: RouteService) -> Self {
        let population = Self::generate_random_population(population_size, &mut route_service);

        Self {
            population_size,
            population,
        }
    }

    pub(crate) fn generate_random_individual(route_service: &mut RouteService) -> Individual {
        let vehicle_ids: Vec<u32> = route_service
            .get_vehicles()
            .iter()
            .map(|vehicle| vehicle.id)
            .collect();

        while route_service.has_available_stop().unwrap() {
            for vehicle_id in vehicle_ids.iter() {
                let stop = route_service.get_random_stop(*vehicle_id).unwrap();

                route_service
                    .assign_stop_to_route(*vehicle_id, stop.id)
                    .unwrap();
            }
        }

        let routes: Vec<Route> = route_service.get_all_routes().values().cloned().collect();

        Individual::new(routes)
    }

    pub(crate) fn generate_random_population(
        population_size: u32,
        route_service: &mut RouteService,
    ) -> Population {
        let mut population = Population::default();

        for _ in 0..population_size {
            let individual = Self::generate_random_individual(route_service);
            population.individuals.push(individual);

            route_service.reset();
        }

        population
    }

    pub(crate) fn selection(&self) -> Vec<Individual> {
        self.population
            .get_k_bests(2)
            .choose_multiple_weighted(&mut thread_rng(), 2, |individual| individual.fitness)
            .unwrap()
            .cloned()
            .collect()
    }
}
