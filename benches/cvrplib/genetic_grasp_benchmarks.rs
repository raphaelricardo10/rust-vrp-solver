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
        } = InstanceRepository::get_instance(instance, 5);

        let rcl_size = (stops.len() as f32 * 0.3).round() as usize;

        let grasp_parameters = GraspSolverParameters {
            rcl_size,
            max_improvement_times: 10,
        };

        let crossover_operator = OrderCrossover::new(50);

        let mut grasp_solver = VrpGraspSolver::new(
            stops.clone(),
            vehicles,
            &distances,
            grasp_parameters,
            thread_rng(),
        );

        c.bench_function(instance, |b| {
            let genetic_parameters: TwoStageGeneticSolverParameters =
                TwoStageGeneticSolverParameters {
                    population_size: 1000,
                    genetic_solver_parameters: GeneticSolverParameters {
                        elite_size: 300,
                        mutation_rate: 0.00,
                        max_generations: 1000,
                        local_search_rate: 0.2,
                    },
                };

            let mut genetic_solver = TwoStageGeneticSolver::new(
                stops.clone(),
                &distances,
                &mut grasp_solver,
                genetic_parameters,
                &crossover_operator,
                Box::new(thread_rng()),
            );

            b.iter(|| {
                let solution = genetic_solver.solve();
                println!("Solution: {:?}", solution.total_distance);
                solution
            })
        });
    }
}

criterion_group!(genetic_grasp, genetic_grasp_benchmark);
