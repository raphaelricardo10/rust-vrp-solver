use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

use crate::solvers::genetic::genetic_solver::GeneticSolverParameters;
use crate::solvers::genetic::population::Population;
use crate::{
    domain::stop::Stop,
    services::distance::distance_service::DistanceMatrix,
    solvers::genetic::{crossover::order_crossover::OrderCrossover, genetic_solver::GeneticSolver},
};

use crate::tests::fixtures::distances_fixture::distances;
use crate::tests::fixtures::services_fixture::{route_service_factory, RouteServiceFactory};
use crate::tests::fixtures::stops_fixture::stops;

#[rstest]
fn test_genetic_algorithm_can_generate_a_good_route(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    route_service_factory: RouteServiceFactory,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let mut route_service = route_service_factory(2);
    let population = Population::from((10, &mut rng, &mut route_service));
    let crossover_op = OrderCrossover::new(5);

    let parameters = GeneticSolverParameters {
        elite_size: 3,
        max_generations: 10,
        local_search_rate: 0.2,
        mutation_rate: 0.05,
    };

    let mut solver = GeneticSolver::new(
        stops,
        &distances,
        population,
        parameters,
        &crossover_op,
        &mut rng,
    );

    solver.solve();

    let solution_v1 = &solver.solution.routes.get(&0).unwrap().stops;
    let solution_v2 = &solver.solution.routes.get(&1).unwrap().stops;

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
}
