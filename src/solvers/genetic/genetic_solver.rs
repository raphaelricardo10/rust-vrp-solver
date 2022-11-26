use std::{cmp, collections::HashSet};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};

use crate::{
    domain::{route::Route, stop::Stop},
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::{RouteMap, RouteService},
    },
    solvers::solution::Solution,
    stop_swapper::{path::Path, StopSwapper},
};

use super::{
    individual::{Chromosome, Gene, GeneAddress, Individual},
    population::Population,
};

pub struct GeneticSolver {
    elite_size: usize,
    mutation_rate: f64,
    population: Population,
    stop_swapper: StopSwapper,
    max_generations: u32,
    current_generation: u32,
    pub solution: Solution,
    best: Individual,
    max_crossover_tries: u8,
}

impl GeneticSolver {
    #[allow(clippy::too_many_arguments)]
    pub fn new<R>(
        stops: Vec<Stop>,
        distances: &DistanceMatrix,
        population_size: u32,
        elite_size: usize,
        mutation_rate: f64,
        max_crossover_tries: u8,
        max_generations: u32,
        mut route_service: RouteService,
        rng: &mut R,
    ) -> Self
    where
        R: Rng + ?Sized,
    {
        let stop_swapper = StopSwapper::new(stops, distances);
        let population = Self::generate_random_population(population_size, &mut route_service, rng);

        Self {
            elite_size,
            population,
            stop_swapper,
            mutation_rate,
            max_generations,
            max_crossover_tries,
            best: Default::default(),
            solution: Default::default(),
            current_generation: Default::default(),
        }
    }

    pub(crate) fn generate_random_individual<R>(
        route_service: &mut RouteService,
        rng: &mut R,
    ) -> Individual
    where
        R: Rng + ?Sized,
    {
        let vehicle_ids: Vec<u32> = route_service
            .get_vehicles()
            .iter()
            .map(|vehicle| vehicle.id)
            .collect();

        route_service.assign_starting_points();

        while route_service.has_available_stop().unwrap() {
            for vehicle_id in vehicle_ids.iter() {
                let stop = match route_service.get_random_stop(*vehicle_id, rng) {
                    Some(stop) => stop,
                    None => continue,
                };

                route_service
                    .assign_stop_to_route(*vehicle_id, stop.id)
                    .unwrap();
            }
        }

        route_service.assign_stop_points();

        let routes: Vec<Route> = route_service.get_all_routes().values().cloned().collect();

        Individual::new(routes)
    }

    pub(crate) fn generate_random_population<R>(
        population_size: u32,
        route_service: &mut RouteService,
        rng: &mut R,
    ) -> Population
    where
        R: Rng + ?Sized,
    {
        let mut population = Population::default();

        for _ in 0..population_size {
            let individual = Self::generate_random_individual(route_service, rng);
            population.individuals.push(individual);

            route_service.reset();
        }

        population
    }

    pub(crate) fn selection(&self) -> Vec<(usize, Individual)> {
        self.population
            .get_k_bests(self.elite_size)
            .choose_multiple_weighted(&mut thread_rng(), 2, |individual| individual.fitness)
            .unwrap()
            .cloned()
            .enumerate()
            .collect()
    }

    pub(crate) fn choose_random_chromosome<'a, R>(
        individual: &'a Individual,
        rng: &mut R,
    ) -> Option<(usize, &'a Chromosome)>
    where
        R: Rng + ?Sized,
    {
        individual.chromosomes.iter().enumerate().choose(rng)
    }

    pub(crate) fn choose_gene<R>(individual: &Individual, rng: &mut R) -> Option<GeneAddress>
    where
        R: Rng + ?Sized,
    {
        let (chromosome_index, chromosome) = Self::choose_random_chromosome(individual, rng)?;

        if chromosome.stops.len() == 1 {
            return Some((chromosome_index, 0));
        }

        let (gene_index, _) = chromosome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .take(chromosome.stops.len() - 2)
            .choose(rng)?;

        Some((chromosome_index, gene_index))
    }

    pub(crate) fn choose_random_genes<R>(
        individual: &Individual,
        rng: &mut R,
    ) -> Option<(GeneAddress, GeneAddress)>
    where
        R: Rng + ?Sized,
    {
        let (chromosome_index, chromosome) = individual
            .chromosomes
            .iter()
            .enumerate()
            .filter(|(_, chromosome)| chromosome.stops.len() > 3)
            .choose(rng)?;

        let addresses: Vec<GeneAddress> = chromosome
            .stops
            .iter()
            .enumerate()
            .skip(1)
            .take(chromosome.stops.len() - 2)
            .choose_multiple(rng, 2)
            .iter()
            .map(|(gene_index, _)| (chromosome_index, *gene_index))
            .collect();

        Some((addresses[0], addresses[1]))
    }

    pub(crate) fn swap_random_genes<R>(
        individual: &mut Individual,
        stop_swapper: &StopSwapper,
        rng: &mut R,
    ) -> Option<()>
    where
        R: Rng + ?Sized,
    {
        let (address1, address2): (GeneAddress, GeneAddress) =
            Self::choose_random_genes(individual, rng)?;

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
                Self::swap_random_genes(individual, stop_swapper, &mut thread_rng());
            });
    }

    fn generate_range<R>(min: usize, max: usize, rng: &mut R) -> (usize, usize)
    where
        R: Rng + ?Sized,
    {
        let a = rng.gen_range(min..=max);
        let mut b = rng.gen_range(min..=max);

        while a == b {
            b = rng.gen_range(min..=max);
        }

        (cmp::min(a, b), cmp::max(a, b))
    }

    fn choose_parent_slice_chromosome<'a, R>(parent: &'a Individual, rng: &mut R) -> Option<(usize, &'a Chromosome)>
    where
        R: Rng + ?Sized,
    {
        parent.chromosomes.iter().enumerate().filter(|(_, chromosome)| chromosome.stops.len() > 3).choose(rng)
    }

    fn slice_individual_randomly<R>(
        individual: &Individual,
        rng: &mut R,
    ) -> Option<(GeneAddress, Vec<Gene>)>
    where
        R: Rng + ?Sized,
    {
        let (chromosome_index, chromosome) = Self::choose_parent_slice_chromosome(individual, rng)?;

        let max_size = chromosome.stops.len() - 1;

        let (lower_bound, upper_bound) = Self::generate_range(1, max_size, rng);

        let slice_address: GeneAddress = (chromosome_index, lower_bound);

        Some((
            slice_address,
            chromosome.stops[lower_bound..upper_bound].to_vec(),
        ))
    }

    pub(crate) fn calculate_slice_cost(slice: &[Gene], distance_service: &DistanceService) -> f64 {
        slice
            .windows(2)
            .map(|window| {
                distance_service
                    .get_distance(&window[0], &window[1])
                    .unwrap()
            })
            .sum()
    }

    pub(crate) fn drop_gene_duplicates<'a>(
        chromosome: &'a Chromosome,
        compare_set: &'a HashSet<Gene>,
    ) -> Vec<&'a Gene> {
        chromosome
            .stops
            .iter()
            .filter(|gene| !compare_set.contains(gene))
            .collect()
    }

    pub(crate) fn make_offspring_chromosome(
        parent1_slice: &HashSet<Gene>,
        parent2_chromosome: Chromosome,
        distance_service: &DistanceService,
    ) -> Chromosome {
        let mut offspring_chromosome = Chromosome::new(parent2_chromosome.vehicle);

        offspring_chromosome
            .add_stop(parent2_chromosome.stops[0], 0.0)
            .unwrap();

        let unrepeated_genes: Vec<&Gene> =
            Self::drop_gene_duplicates(&parent2_chromosome, parent1_slice);

        if unrepeated_genes.len() == 2 {
            return offspring_chromosome;
        }

        unrepeated_genes
            .windows(2)
            .map(|window| {
                (
                    window[1],
                    distance_service.get_distance(window[0], window[1]).unwrap(),
                )
            })
            .for_each(|(gene, distance)| {
                offspring_chromosome.add_stop(*gene, distance).unwrap();
            });

        offspring_chromosome
    }

    pub(crate) fn insert_parent_slice_in_offspring(
        offspring: &mut Individual,
        insertion_point: GeneAddress,
        parent_slice: Vec<Gene>,
        parent_slice_cost: f64,
        distance_service: &DistanceService,
    ) -> Option<()> {
        let distance_before = match offspring.chromosomes[insertion_point.0].stops.len() == 1 {
            true => 0.0,
            false => distance_service.get_distance(
                &offspring.chromosomes[insertion_point.0].stops[insertion_point.1 - 1],
                parent_slice.first()?,
            )?,
        };

        let distance_after = distance_service.get_distance(
            parent_slice.last()?,
            &offspring.chromosomes[insertion_point.0].stops[insertion_point.1],
        )?;

        offspring.chromosomes[insertion_point.0].add_multiple_stops_at(
            parent_slice,
            insertion_point.1,
            parent_slice_cost + distance_before + distance_after,
        );

        offspring.update_fitness();

        Some(())
    }

    pub(crate) fn make_offspring<R>(
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized,
    {
        let (_, parent_slice): (GeneAddress, Vec<Gene>) =
            Self::slice_individual_randomly(&parent1, rng)?;

        let parent_slice_cost = Self::calculate_slice_cost(&parent_slice, distance_service);

        let genes_set: HashSet<Gene> = HashSet::from_iter(parent_slice.iter().cloned());

        let mut offspring_chromosomes: Vec<Chromosome> = Vec::new();

        for chromosome in parent2.chromosomes {
            offspring_chromosomes.push(Self::make_offspring_chromosome(
                &genes_set,
                chromosome,
                distance_service,
            ));
        }

        let mut offspring = Individual::new(offspring_chromosomes);
        let insertion_point: GeneAddress = Self::choose_gene(&offspring, rng).unwrap();

        Self::insert_parent_slice_in_offspring(
            &mut offspring,
            insertion_point,
            parent_slice,
            parent_slice_cost,
            distance_service,
        )?;

        Some(offspring)
    }

    fn is_offspring_better_than_parents(
        offspring: &Individual,
        parent1: &Individual,
        parent2: &Individual,
    ) -> bool {
        if offspring.fitness > parent1.fitness {
            return false;
        }

        if offspring.fitness > parent2.fitness {
            return false;
        }

        true
    }

    pub(crate) fn make_better_offspring<R>(
        parent1: Individual,
        parent2: Individual,
        rng: &mut R,
        number_of_tries: u8,
        distance_service: &DistanceService,
    ) -> Option<Individual>
    where
        R: Rng + ?Sized,
    {
        for _ in 0..number_of_tries {
            let offspring =
                Self::make_offspring(parent1.clone(), parent2.clone(), rng, distance_service)?;

            if Self::is_offspring_better_than_parents(&offspring, &parent1, &parent2) {
                return Some(offspring);
            }
        }

        None
    }

    pub(crate) fn crossover<R>(
        &mut self,
        parent1: &Individual,
        parent2: &Individual,
        rng: &mut R,
    ) -> Option<(Individual, Individual)>
    where
        R: Rng + ?Sized,
    {
        let offspring1 = Self::make_better_offspring(
            parent1.clone(),
            parent2.clone(),
            rng,
            self.max_crossover_tries,
            &self.stop_swapper.distance_service,
        )?;

        let offspring2 = Self::make_better_offspring(
            parent1.clone(),
            parent2.clone(),
            rng,
            self.max_crossover_tries,
            &self.stop_swapper.distance_service,
        )?;

        Some((offspring1, offspring2))
    }

    fn stop_condition_met(&self) -> bool {
        self.current_generation >= self.max_generations
    }

    fn should_update_best(&self, individual: &Individual) -> bool {
        individual.fitness < self.best.fitness
    }

    pub fn solve<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        while !self.stop_condition_met() {
            loop {
                let parents = self.selection();

                let (parent1_index, parent1) = &parents[0];
                let (parent2_index, parent2) = &parents[1];

                let (offspring1, offspring2) = match self.crossover(parent1, parent2, rng) {
                    Some(offsprings) => offsprings,
                    None => break,
                };

                if self.should_update_best(&offspring1) {
                    self.best = offspring1.clone();
                }

                if self.should_update_best(&offspring2) {
                    self.best = offspring2.clone();
                }

                self.population.individuals[*parent1_index] = offspring1;
                self.population.individuals[*parent2_index] = offspring2;
            }

            self.mutation();

            self.current_generation += 1;
        }

        let route_map: RouteMap = self
            .best
            .chromosomes
            .iter()
            .cloned()
            .map(|chromosome| (chromosome.vehicle.id, chromosome))
            .collect();

        self.solution = Solution::new(&route_map, self.best.fitness);
    }
}
