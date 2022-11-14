use crate::{services::distance::distance_service::DistanceMatrix, solvers::solver::Solver};
use rstest::rstest;

use crate::{
    domain::stop::Stop, solvers::grasp::solver::GraspSolver, tests::fixtures::VehicleFactory,
};

use super::fixtures::{distances, stops, vehicle_factory};

#[rstest]
fn grasp_solution_is_generated(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(2);

    let mut solver = GraspSolver::new(3, vehicles, &distances, stops);
    solver.solve();

    let solution_v1 = solver.get_solution().get(&0).unwrap();
    let solution_v2 = solver.get_solution().get(&1).unwrap();

    assert_ne!(solution_v1.len(), 0);
    assert_ne!(solution_v2.len(), 0);
}
