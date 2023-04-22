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
        solution::Solution,
        solver::Solver,
    },
};

pub struct TwoStageGeneticSolver<'a, R: Rng + ?Sized> {
    first_stage_solver: &'a dyn Solver,
    genetic_solver: GeneticSolver<'a, R>,
}

pub struct TwoStageGeneticSolverParameters {
    population_size: u32,
    genetic_solver_parameters: GeneticSolverParameters,
}

impl<'a, R: Rng + ?Sized> Solver for TwoStageGeneticSolver<'a, R> {
    fn solve(&mut self) {
        self.genetic_solver.solve();
    }

    fn get_solution(&self) -> &Solution {
        &self.genetic_solver.solution
    }

    fn reset_solution(&mut self) {
        self.genetic_solver.solution = Default::default();
    }
}

impl<'a, R: Rng + ?Sized> TwoStageGeneticSolver<'a, R> {
    pub(crate) fn new(
        stops: Vec<Stop>,
        distances: &DistanceMatrix,
        first_stage_solver: &'a mut dyn Solver,
        parameters: TwoStageGeneticSolverParameters,
        crossover_op: &'a dyn CrossoverOperator<R>,
        rng: &'a mut R,
    ) -> Self {
        first_stage_solver.solve();
        Self {
            first_stage_solver,
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
}
