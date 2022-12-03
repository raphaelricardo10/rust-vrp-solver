use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

use crate::{
    domain::stop::Stop, services::distance::distance_service::DistanceMatrix,
    solvers::genetic::genetic_solver::GeneticSolver,
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

    let route_service = route_service_factory(2);
    let mut solver = GeneticSolver::new(
        stops,
        &distances,
        10,
        3,
        0.05,
        10,
        5,
        route_service,
        &mut rng,
    );

    solver.solve();

    let solution_v1 = solver.solution.result.get(&0).unwrap();
    let solution_v2 = solver.solution.result.get(&1).unwrap();

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
}
