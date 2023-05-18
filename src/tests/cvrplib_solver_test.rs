use rand::{SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;
use rstest::{fixture, rstest};

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    parsers::{
        cvrplib::cvrplib_parser::CvrpLibParser,
        vrp_parser::{VrpInputs, VrpParser},
    },
    services::distance::distance_service::DistanceMatrix,
    solvers::{
        genetic::{
            crossover::order_crossover::OrderCrossover, genetic_solver::GeneticSolverParameters,
        },
        grasp::vrp_grasp_solver::{GraspSolverParameters, VrpGraspSolver},
        greedy::vrp_greedy_solver::VrpGreedySolver,
        solver::Solver,
        two_stage_genetic::two_stage_genetic_solver::{
            TwoStageGeneticSolver, TwoStageGeneticSolverParameters,
        },
    },
};

#[fixture]
fn vrp_inputs() -> VrpInputs {
    CvrpLibParser::from_file("./src/parsers/cvrplib/tests/A-n32-k5.vrp", 5).parse()
}

#[fixture]
fn stops(vrp_inputs: VrpInputs) -> Vec<Stop> {
    vrp_inputs.stops
}

#[fixture]
fn vehicles(vrp_inputs: VrpInputs) -> Vec<Vehicle> {
    vrp_inputs.vehicles
}

#[fixture]
fn distances(vrp_inputs: VrpInputs) -> DistanceMatrix {
    vrp_inputs.distances
}

#[rstest]
fn test_can_solve_with_greedy_solver(
    stops: Vec<Stop>,
    vehicles: Vec<Vehicle>,
    distances: DistanceMatrix,
) {
    let mut solver = VrpGreedySolver::new(vehicles, &distances, stops);

    let solution = solver.solve();
    assert_eq!(solution.total_distance, 1235.28748)
}

#[rstest]
fn test_can_solve_with_grasp_solver(
    stops: Vec<Stop>,
    vehicles: Vec<Vehicle>,
    distances: DistanceMatrix,
) {
    let rng = ChaCha8Rng::seed_from_u64(0);

    let parameters = GraspSolverParameters {
        rcl_size: 10,
        max_improvement_times: 3,
    };

    let mut solver = VrpGraspSolver::new(stops, vehicles, &distances, parameters, rng);
    let solution = solver.solve();

    const POSSIBLE_RESULTS: [f32; 2] = [1687.5743, 1763.0903];

    assert!(POSSIBLE_RESULTS.contains(&solution.total_distance));
}

#[rstest]
fn test_can_solve_with_grasp_genetic_solver(
    stops: Vec<Stop>,
    vehicles: Vec<Vehicle>,
    distances: DistanceMatrix,
) {
    let rng = ChaCha8Rng::seed_from_u64(0);

    let grasp_parameters = GraspSolverParameters {
        rcl_size: 10,
        max_improvement_times: 3,
    };

    let mut grasp_solver =
        VrpGraspSolver::new(stops.clone(), vehicles, &distances, grasp_parameters, rng.clone());

    let genetic_parameters = TwoStageGeneticSolverParameters {
        population_size: 1000,
        genetic_solver_parameters: GeneticSolverParameters {
            elite_size: 100,
            local_search_rate: 0.2,
            mutation_rate: 0.05,
            max_generations: 5000,
        },
    };

    let crossover_operator = OrderCrossover::new(255);

    let mut genetic_solver = TwoStageGeneticSolver::new(
        stops,
        &distances,
        &mut grasp_solver,
        genetic_parameters,
        &crossover_operator,
        Box::new(thread_rng()),
    );

    let solution = genetic_solver.solve();
    assert_ne!(solution.total_distance, f32::MAX);
}
