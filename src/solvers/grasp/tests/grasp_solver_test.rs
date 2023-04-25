use crate::services::distance::distance_service::DistanceMatrix;
use crate::solvers::grasp::grasp_solver::GraspSolverParameters;
use crate::solvers::solver::Solver;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rstest::rstest;

use crate::{domain::stop::Stop, solvers::grasp::grasp_solver::GraspSolver};

use crate::tests::fixtures::distances_fixture::distances;
use crate::tests::fixtures::stops_fixture::stops;
use crate::tests::fixtures::vehicles_fixture::{vehicle_factory, VehicleFactory};

#[rstest]
fn grasp_solution_is_correct(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let rng = ChaCha8Rng::seed_from_u64(0);
    let vehicles = vehicle_factory(2);

    let parameters = GraspSolverParameters {
        rcl_size: 3,
        max_improvement_times: 3,
    };

    let mut solver = GraspSolver::new(stops, vehicles, &distances, parameters, Box::new(rng));
    let solution = solver.solve();

    let solution_v1 = &solution.routes.get(&0).unwrap().stops;
    let solution_v2 = &solution.routes.get(&1).unwrap().stops;

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
    assert_eq!(solution.total_distance, 10.0);
}
