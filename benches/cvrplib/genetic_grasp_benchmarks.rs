use criterion::{criterion_group, Criterion};
use rand::thread_rng;
use vrp_solver::{
    parsers::vrp_parser::VrpInputs,
    solvers::{
        genetic::{
            crossover::order_crossover::OrderCrossover, genetic_solver::GeneticSolverParameters,
        },
        grasp::vrp_grasp_solver::{GraspSolverParameters, VrpGraspSolver},
        solver::Solver,
        two_stage_genetic::two_stage_genetic_solver::{
            TwoStageGeneticSolver, TwoStageGeneticSolverParameters,
        },
    },
};

use crate::cvrplib::instance_repository::InstanceRepository;

pub fn genetic_grasp_benchmark(c: &mut Criterion) {
    const INSTANCES: [&str; 1] = ["A-n32-k5"];

    for instance in INSTANCES {
        let VrpInputs {
            stops,
            vehicles,
            distances,
        } = InstanceRepository::get_instance(instance);

        let rcl_size = (stops.len() as f32 * 0.3).round() as usize;

        let grasp_parameters = GraspSolverParameters {
            rcl_size,
            max_improvement_times: 10,
        };

        let crossover_operator = OrderCrossover::new(3);

        let genetic_parameters = TwoStageGeneticSolverParameters {
            population_size: 200,
            genetic_solver_parameters: GeneticSolverParameters {
                elite_size: 300,
                mutation_rate: 0.5,
                max_generations: 100,
                local_search_rate: 0.2,
            },
        };

        let mut grasp_solver = VrpGraspSolver::new(
            stops.clone(),
            vehicles,
            &distances,
            grasp_parameters,
            thread_rng(),
        );

        let mut genetic_solver = TwoStageGeneticSolver::new(
            stops,
            &distances,
            &mut grasp_solver,
            genetic_parameters,
            &crossover_operator,
            Box::new(thread_rng()),
        );

        c.bench_function(instance, |b| b.iter(|| genetic_solver.solve()));
    }
}

criterion_group!(genetic_grasp, genetic_grasp_benchmark);
