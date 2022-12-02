use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    domain::stop::Stop,
    services::{
        distance::distance_service::DistanceMatrix,
        route::route_service::{RouteMap, RouteService},
    },
    solvers::solution::Solution,
    stop_swapper::StopSwapper,
};

use super::{
    crossover::{order_crossover::OrderCrossover, offspring::Offspring}, individual::Individual, population::Population,
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
        mut route_service: RouteService,
        rng: &'a mut R,
    ) -> Self {
        let stop_swapper = StopSwapper::new(stops, distances);
        let crossover_op = OrderCrossover::new(max_crossover_tries);
        let population = Population::from_random(population_size, rng, &mut route_service);

        Self {
            rng,
            elite_size,
            population,
            crossover_op,
            stop_swapper,
            mutation_rate,
            max_generations,
            best: Default::default(),
            solution: Default::default(),
            current_generation: Default::default(),
        }
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

    pub(crate) fn mutation(&mut self) {
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
