use crate::services::distance::distance_service::DistanceMatrix;
use rstest::rstest;

use crate::{domain::stop::Stop, solvers::grasp::grasp_solver::GraspSolver};

use super::fixtures::distances_fixture::distances;
use super::fixtures::stops_fixture::stops;
use super::fixtures::vehicles_fixture::{vehicle_factory, VehicleFactory};

#[rstest]
fn grasp_solution_is_generated(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(2);

    let mut solver = GraspSolver::new(3, vehicles, &distances, 3, stops);
    solver.solve();

    let solution_v1 = solver.solution.result.get(&0).unwrap();
    let solution_v2 = solver.solution.result.get(&1).unwrap();

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
}
