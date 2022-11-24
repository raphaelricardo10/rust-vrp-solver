use std::{cmp, collections::HashSet};

use rand::{
    rngs::ThreadRng,
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
    individual::{Chromosome, Gene, GeneAddress, Individual},
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

    pub(crate) fn choose_random_chromosome<'a>(
        individual: &'a Individual,
    ) -> Option<(usize, &'a Chromosome)> {
        let mut rng = thread_rng();

        individual.chromosomes.iter().enumerate().choose(&mut rng)
    }

    pub(crate) fn choose_gene(individual: &Individual, rng: &mut ThreadRng) -> Option<GeneAddress> {
        let (chromosome_index, chromosome) = Self::choose_random_chromosome(individual)?;

        let address: GeneAddress = chromosome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .take(chromosome.stops.len() - 1)
            .choose(rng)
            .iter()
            .map(|(gene_index, _)| (chromosome_index, *gene_index))
            .last()?;

        Some(address)
    }

    pub(crate) fn choose_random_genes(
        individual: &Individual,
    ) -> Option<(GeneAddress, GeneAddress)> {
        let mut rng = thread_rng();

        let (chromosome_index, chromosome) = Self::choose_random_chromosome(individual)?;

        let addresses: Vec<GeneAddress> = chromosome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .choose_multiple(&mut rng, 2)
            .iter()
            .map(|(gene_index, gene)| (chromosome_index, *gene_index))
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

    fn generate_range(min: usize, max: usize, rng: &mut ThreadRng) -> (usize, usize) {
        let a = rng.gen_range(min..=max);
        let mut b = rng.gen_range(min..=max);

        while a == b {
            b = rng.gen_range(min..=max);
        }

        (cmp::min(a, b), cmp::max(a, b))
    }

    fn slice_individual_randomly(
        individual: &Individual,
        rng: &mut ThreadRng,
    ) -> Option<(GeneAddress, Vec<Gene>)> {
        let (chromosome_index, chromosome) = Self::choose_random_chromosome(individual)?;

        let max_size = chromosome.stops.len();

        let (lower_bound, upper_bound) =
            Self::generate_range(1, max_size, rng);

        let slice_address: GeneAddress = (chromosome_index, lower_bound);

        Some((
            slice_address,
            chromosome.stops[lower_bound..upper_bound].to_vec(),
        ))
    }

    pub(crate) fn make_offspring(
        parent1: Individual,
        parent2: Individual,
        rng: &mut ThreadRng,
        distance_service: &DistanceService,
    ) -> Option<Individual> {
        let (_, parent_slice): (GeneAddress, Vec<Gene>) =
            Self::slice_individual_randomly(&parent1, rng)?;

        let parent_slice_cost: f64 = parent_slice[..parent_slice.len() - 1]
            .iter()
            .enumerate()
            .skip(1)
            .map(|(gene_index, _)| {
                Path::from_stop_index(&parent_slice, gene_index, distance_service)
                    .unwrap()
                    .cost
            })
            .sum();

        let genes_set: HashSet<Gene> = parent2
            .chromosomes
            .iter()
            .flat_map(|chromosome| chromosome.stops.clone())
            .collect();

        let mut offspring_chromosomes: Vec<Chromosome> = Vec::new();

        for chromosome in parent2.chromosomes {
            let stops = chromosome.stops.clone();
            let mut offspring_chromosome: Chromosome = chromosome.clone();

            chromosome
                .stops[..chromosome.stops.len() - 1]
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(_, gene)| genes_set.contains(gene))
                .map(|(gene_index, _)| {
                    Path::from_stop_index(&stops, gene_index, distance_service).unwrap()
                })
                .for_each(|path| offspring_chromosome.remove_stop(path.current.index, path.cost));

            offspring_chromosomes.push(offspring_chromosome);
        }

        let mut offspring = Individual::new(offspring_chromosomes);

        let insertion_point: GeneAddress = Self::choose_gene(&offspring, rng)?;

        offspring.chromosomes[insertion_point.0].add_multiple_stops_at(
            parent_slice,
            insertion_point.1,
            parent_slice_cost,
        );

        Some(offspring)
    }

    pub(crate) fn crossover(&mut self) -> Option<()> {
        let parents = self.selection();

        let mut rng = thread_rng();

        let offspring1 = Self::make_offspring(
            parents[0].clone(),
            parents[1].clone(),
            &mut rng,
            &self.stop_swapper.distance_service,
        )?;

        let offspring2 = Self::make_offspring(
            parents[1].clone(),
            parents[0].clone(),
            &mut rng,
            &self.stop_swapper.distance_service,
        )?;

        Some(())
    }
}
