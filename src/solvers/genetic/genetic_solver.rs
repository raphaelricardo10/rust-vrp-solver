use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    domain::stop::Stop,
    local_search::two_opt::TwoOptSearcher,
    services::{
        distance::distance_service::DistanceMatrix,
        route::route_service::{RouteMap, RouteService},
    },
    solvers::solution::Solution,
    stop_swapper::StopSwapper,
};

use super::{
    crossover::{offspring::Offspring, order_crossover::OrderCrossover},
    individual::Individual,
    population::Population,
};

pub struct GeneticSolver<'a, R: Rng + ?Sized> {
    elite_size: usize,
    mutation_rate: f64,
    population: Population,
    stop_swapper: StopSwapper,
    max_generations: u32,
    current_generation: u32,
    pub solution: Solution,
    best: Individual,
    crossover_op: OrderCrossover,
    local_search: TwoOptSearcher,
    local_search_rate: f64,
    rng: &'a mut R,
}

impl<'a, R: Rng + ?Sized> GeneticSolver<'a, R> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        stops: Vec<Stop>,
        distances: &DistanceMatrix,
        population_size: u32,
        elite_size: usize,
        mutation_rate: f64,
        max_crossover_tries: u8,
        max_generations: u32,
        local_search_rate: f64,
        mut route_service: RouteService,
        rng: &'a mut R,
    ) -> Self {
        let stop_swapper = StopSwapper::new(stops.clone(), distances);
        let local_search = TwoOptSearcher::new(stops, distances);
        let crossover_op = OrderCrossover::new(max_crossover_tries);
        let population = Population::from_random(population_size, rng, &mut route_service);

        Self {
            rng,
            elite_size,
            population,
            crossover_op,
            stop_swapper,
            local_search,
            mutation_rate,
            max_generations,
            local_search_rate,
            best: Default::default(),
            solution: Default::default(),
            current_generation: Default::default(),
        }
    }

    pub(super) fn selection(&self) -> Vec<(usize, Individual)> {
        self.population
            .get_k_bests(self.elite_size)
            .choose_multiple_weighted(&mut thread_rng(), 2, |individual| individual.fitness)
            .unwrap_or_else(|err| match err {
                rand::distributions::WeightedError::NoItem => {
                    panic!("the candidate list should not be empty")
                }
                rand::distributions::WeightedError::InvalidWeight => {
                    panic!("the weight value should be between 0 and f64::MAX")
                }
                rand::distributions::WeightedError::AllWeightsZero => {
                    panic!("at least one weight must be a non-zero value")
                }
                rand::distributions::WeightedError::TooMany => {
                    panic!("the quantity of weights must be between 0 and u32::MAX")
                }
            })
            .cloned()
            .enumerate()
            .collect()
    }

    pub(super) fn mutation(&mut self) {
        let stop_swapper = &self.stop_swapper;

        let mutated_individuals: Vec<&mut Individual> = self
            .population
            .individuals
            .iter_mut()
            .filter(|_| self.rng.gen_bool(self.mutation_rate))
            .collect();

        for individual in mutated_individuals {
            individual.swap_random_genes(stop_swapper, self.rng);
        }
    }

    pub(super) fn crossover(
        &mut self,
        parent1: &Individual,
        parent2: &Individual,
    ) -> Option<(Individual, Individual)> {
        let mut offspring1 =
            Offspring::new(parent1.clone(), parent2.clone(), self.crossover_op.clone());
        let mut offspring2 =
            Offspring::new(parent2.clone(), parent1.clone(), self.crossover_op.clone());

        offspring1.try_to_evolve(self.rng, &self.stop_swapper.distance_service)?;
        offspring2.try_to_evolve(self.rng, &self.stop_swapper.distance_service)?;

        Some((offspring1.individual, offspring2.individual))
    }

    pub(super) fn apply_local_search(&mut self) {
        let selected_individuals: Vec<&mut Individual> = self
            .population
            .individuals
            .iter_mut()
            .filter(|_| self.rng.gen_bool(self.local_search_rate))
            .collect();

        for individual in selected_individuals {
            for chromosome in individual.chromosomes.iter_mut() {
                self.local_search.run(chromosome);
            }
        }
    }

    fn stop_condition_met(&self) -> bool {
        self.current_generation >= self.max_generations
    }

    fn should_update_best(&self, individual: &Individual) -> bool {
        individual.fitness < self.best.fitness
    }

    pub fn solve(&mut self) {
        while !self.stop_condition_met() {
            loop {
                let parents = self.selection();

                let (parent1_index, parent1) = &parents[0];
                let (parent2_index, parent2) = &parents[1];

                let (offspring1, offspring2) = match self.crossover(parent1, parent2) {
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
            self.apply_local_search();

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
