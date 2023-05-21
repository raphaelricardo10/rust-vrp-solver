use std::rc::Rc;

use rand::{seq::SliceRandom, Rng};

use crate::{
    domain::stop::Stop,
    local_search::two_opt::TwoOptSearcher,
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteMap,
    },
    solvers::{solver::Solver, vrp_solution::VrpSolution},
    stop_swapper::StopSwapper,
};

use super::{
    crossover::{crossover_operator::CrossoverOperator, offspring::Offspring},
    individual::Individual,
    population::Population,
};

pub struct GeneticSolverParameters {
    pub elite_size: usize,
    pub mutation_rate: f32,
    pub max_generations: u32,
    pub local_search_rate: f32,
}

pub struct GeneticSolver<'a, R: Rng + ?Sized> {
    parameters: GeneticSolverParameters,
    population: Population,
    stop_swapper: StopSwapper,
    current_generation: u32,
    initial_population: Population,
    pub solution: VrpSolution,
    best: Individual,
    crossover_op: &'a dyn CrossoverOperator<R>,
    local_search: TwoOptSearcher,
    distance_service: Rc<DistanceService>,
    rng: Box<R>,
}

impl<'a, R: Rng + ?Sized> Solver<VrpSolution> for GeneticSolver<'a, R> {
    fn solve(&mut self) -> VrpSolution {
        while !self.stop_condition_met() {
            let parents = self.selection();

            let (parent1_index, parent1) = &parents[0];
            let (parent2_index, parent2) = &parents[1];

            let (offspring1, offspring2) = match self.crossover(parent1, parent2) {
                Some(offsprings) => offsprings,
                None => continue,
            };

            self.population.individuals[*parent1_index] = offspring1;
            self.population.individuals[*parent2_index] = offspring2;

            self.mutation();
            self.apply_local_search();

            let best_in_generation = self
                .population
                .individuals
                .iter()
                .min_by(|individual_1, individual_2| {
                    individual_1.fitness.total_cmp(&individual_2.fitness)
                })
                .unwrap()
                .clone();

            if best_in_generation.fitness < self.best.fitness {
                self.best = best_in_generation;
            }

            self.current_generation += 1;
        }

        let route_map: RouteMap = self
            .best
            .chromosomes
            .iter()
            .cloned()
            .map(|chromosome| (chromosome.vehicle.id, chromosome))
            .collect();

        self.population = self.initial_population.clone();

        VrpSolution::new(&route_map, self.best.fitness)
    }
}

impl<'a, R: Rng + ?Sized> GeneticSolver<'a, R> {
    pub(crate) fn new(
        stops: Vec<Stop>,
        distances: &DistanceMatrix,
        population: Population,
        parameters: GeneticSolverParameters,
        crossover_op: &'a dyn CrossoverOperator<R>,
        rng: Box<R>,
    ) -> Self {
        let distance_service = Rc::new(DistanceService::new(stops, distances));

        Self {
            rng,
            parameters,
            crossover_op,
            population: population.clone(),
            initial_population: population,
            best: Default::default(),
            solution: Default::default(),
            current_generation: Default::default(),
            distance_service: distance_service.clone(),
            local_search: TwoOptSearcher::new(distance_service.clone()),
            stop_swapper: StopSwapper { distance_service },
        }
    }

    pub(crate) fn update_population(&mut self, population: Population) {
        self.population = population;
    }

    pub(super) fn selection(&mut self) -> Vec<(usize, Individual)> {
        self.population
            .get_k_bests(self.parameters.elite_size)
            .choose_multiple_weighted(&mut self.rng, 2, |individual| individual.fitness)
            .unwrap_or_else(|err| match err {
                rand::distributions::WeightedError::NoItem => {
                    panic!("the candidate list should not be empty")
                }
                rand::distributions::WeightedError::InvalidWeight => {
                    panic!("the weight value should be between 0 and f32::MAX")
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

        for individual in self.population.individuals.iter_mut() {
            if self.rng.gen_bool(self.parameters.mutation_rate.into()) {
                individual.swap_random_genes(stop_swapper, &mut self.rng);
            }
        }
    }

    pub(super) fn crossover(
        &mut self,
        parent1: &Individual,
        parent2: &Individual,
    ) -> Option<(Individual, Individual)> {
        let mut offspring1 = Offspring::new(parent1.clone(), parent2.clone(), self.crossover_op);
        let mut offspring2 = Offspring::new(parent2.clone(), parent1.clone(), self.crossover_op);

        offspring1.try_to_evolve(&mut self.rng, &self.distance_service)?;
        offspring2.try_to_evolve(&mut self.rng, &self.distance_service)?;

        Some((offspring1.individual, offspring2.individual))
    }

    pub(super) fn apply_local_search(&mut self) {
        for individual in self.population.individuals.iter_mut() {
            if self.rng.gen_bool(self.parameters.local_search_rate as f64) {
                for chromosome in individual.chromosomes.iter_mut() {
                    self.local_search.run(chromosome);
                }
                individual.update_fitness();
            }
        }
    }

    fn stop_condition_met(&self) -> bool {
        self.current_generation >= self.parameters.max_generations
    }
}
