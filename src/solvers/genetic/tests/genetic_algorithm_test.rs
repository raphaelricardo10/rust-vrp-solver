use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

use crate::solvers::genetic::genetic_solver::GeneticSolverParameters;
use crate::solvers::random::random_solver::RandomSolver;
use crate::solvers::solver::Solver;
use crate::solvers::two_stage_genetic::two_stage_genetic_solver::{
    TwoStageGeneticSolver, TwoStageGeneticSolverParameters,
};
use crate::tests::fixtures::vehicles_fixture::{vehicle_factory, VehicleFactory};
use crate::{
    domain::stop::Stop, services::distance::distance_service::DistanceMatrix,
    solvers::genetic::crossover::order_crossover::OrderCrossover,
};

use crate::tests::fixtures::distances_fixture::distances;
use crate::tests::fixtures::stops_fixture::stops;

#[rstest]
fn test_genetic_algorithm_can_generate_a_good_route(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(2);

    let mut rng1 = ChaCha8Rng::seed_from_u64(0);
    let mut rng2 = ChaCha8Rng::seed_from_u64(0);

    let mut random_solver = RandomSolver::new(stops.clone(), vehicles, &distances, &mut rng1);

    let parameters = TwoStageGeneticSolverParameters {
        population_size: 10,
        genetic_solver_parameters: GeneticSolverParameters {
            elite_size: 3,
            max_generations: 10,
            local_search_rate: 0.2,
            mutation_rate: 0.05,
        },
    };

    let crossover_op = OrderCrossover::new(5);

    let mut genetic_solver = TwoStageGeneticSolver::new(
        stops,
        &distances,
        &mut random_solver,
        parameters,
        &crossover_op,
        &mut rng2,
    );

    genetic_solver.solve();

    let solution_v1 = &genetic_solver.get_solution().routes.get(&0).unwrap().stops;
    let solution_v2 = &genetic_solver.get_solution().routes.get(&1).unwrap().stops;

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
}
