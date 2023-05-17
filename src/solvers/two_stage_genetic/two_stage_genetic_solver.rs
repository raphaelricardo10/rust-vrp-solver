use rand::Rng;

use crate::{
    domain::stop::Stop,
    services::distance::distance_service::DistanceMatrix,
    solvers::{
        genetic::{
            crossover::crossover_operator::CrossoverOperator,
            genetic_solver::{GeneticSolver, GeneticSolverParameters},
            population::Population,
        },
        solver::Solver,
        vrp_solution::VrpSolution,
    },
};

pub struct TwoStageGeneticSolverParameters {
    pub population_size: u32,
    pub genetic_solver_parameters: GeneticSolverParameters,
}

pub struct TwoStageGeneticSolver<'a, R: Rng + ?Sized> {
    population_size: u32,
    first_stage_solver: &'a mut dyn Solver<VrpSolution>,
    genetic_solver: GeneticSolver<'a, R>,
}

impl<'a, R: Rng + ?Sized> Solver<VrpSolution> for TwoStageGeneticSolver<'a, R> {
    fn solve(&mut self) -> VrpSolution {
        let solutions = self.generate_initial_solutions();
        let population = Population::from(solutions.as_slice());
        self.genetic_solver.update_population(population);
        self.genetic_solver.solve()
    }
}

impl<'a, R: Rng + ?Sized> TwoStageGeneticSolver<'a, R> {
    pub fn new(
        stops: Vec<Stop>,
        distances: &DistanceMatrix,
        first_stage_solver: &'a mut dyn Solver<VrpSolution>,
        parameters: TwoStageGeneticSolverParameters,
        crossover_op: &'a dyn CrossoverOperator<R>,
        rng: Box<R>,
    ) -> Self {
        Self {
            first_stage_solver,
            population_size: parameters.population_size,
            genetic_solver: GeneticSolver::new(
                stops,
                distances,
                Default::default(),
                parameters.genetic_solver_parameters,
                crossover_op,
                rng,
            ),
        }
    }

    fn generate_initial_solutions(&mut self) -> Vec<VrpSolution> {
        (0..self.population_size)
            .map(|_| self.first_stage_solver.solve())
            .collect()
    }
}
