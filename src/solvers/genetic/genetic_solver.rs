use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};

use crate::{
    domain::{route::Route, stop::Stop},
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    stop_swapper::{path::Path, StopSwapper},
};

use super::{
    individual::{GeneAddress, Individual},
    population::Population,
};

pub struct GeneticSolver {
    elite_size: usize,
    mutation_rate: f64,
    population_size: u32,
    population: Population,
    stop_swapper: StopSwapper,
}

impl GeneticSolver {
    pub fn new(
        stops: Vec<Stop>,
        distances: &DistanceMatrix,
        population_size: u32,
        elite_size: usize,
        mutation_rate: f64,
        mut route_service: RouteService,
    ) -> Self {
        let stop_swapper = StopSwapper::new(stops, distances);
        let population = Self::generate_random_population(population_size, &mut route_service);

        Self {
            elite_size,
            population,
            stop_swapper,
            mutation_rate,
            population_size,
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
            .get_k_bests(self.elite_size)
            .choose_multiple_weighted(&mut thread_rng(), 2, |individual| individual.fitness)
            .unwrap()
            .cloned()
            .collect()
    }

    pub(crate) fn choose_random_genes(
        individual: &Individual,
    ) -> Option<(GeneAddress, GeneAddress)> {
        let mut rng = thread_rng();

        let (chromossome_index, chromossome) =
            individual.chromosomes.iter().enumerate().choose(&mut rng)?;

        let addresses: Vec<GeneAddress> = chromossome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .choose_multiple(&mut rng, 2)
            .iter()
            .map(|(gene_index, gene)| (chromossome_index, *gene_index))
            .collect();

        Some((addresses[0], addresses[1]))
    }

    pub(crate) fn swap_random_genes(
        individual: &mut Individual,
        stop_swapper: &StopSwapper,
    ) -> Option<()> {
        let (address1, address2): (GeneAddress, GeneAddress) =
            Self::choose_random_genes(individual)?;

        let path1 = Path::from_stop_index(
            &individual.chromosomes.get(address1.0)?.stops,
            address1.1,
            &stop_swapper.distance_service,
        )?;

        let path2 = Path::from_stop_index(
            &individual.chromosomes.get(address2.0)?.stops,
            address2.1,
            &stop_swapper.distance_service,
        )?;

        let swap_cost = stop_swapper.calculate_swap_cost(&path1, &path2);

        individual.swap_genes(address1, address2, swap_cost);

        Some(())
    }

    pub(crate) fn mutation(&mut self) {
        let mut rng = thread_rng();

        let stop_swapper = &self.stop_swapper;

        self.population
            .individuals
            .iter_mut()
            .filter(|_| rng.gen_bool(self.mutation_rate))
            .for_each(|individual| {
                Self::swap_random_genes(individual, stop_swapper);
            });
    }
}
