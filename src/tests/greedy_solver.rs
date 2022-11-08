use crate::{
    services::{distance::distance_service::DistanceMatrix, route::route_service::RouteService},
    solvers::solver::Solver,
};
use rstest::rstest;

use crate::{
    domain::stop::Stop, solvers::greedy::solver::GreedySolver, tests::fixtures::VehicleFactory,
};

use super::fixtures::{distances, stops, vehicle_factory};

#[rstest]
fn greedy_solution_is_correct_single_vehicle(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(1);

    let mut solver = GreedySolver::new(vehicles, distances, stops);
    solver.solve();

    let solution = solver.get_solution().get(&0).unwrap();

    assert_eq!(solution[0], 0);
    assert_eq!(solution[1], 2);
    assert_eq!(solution[2], 3);
    assert_eq!(solution[3], 1);
}

#[rstest]
fn greedy_solution_is_correct_multiple_vehicles(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(2);

    let mut solver = GreedySolver::new(vehicles, distances, stops);
    solver.solve();

    let solution_v1 = solver.get_solution().get(&0).unwrap();
    let solution_v2 = solver.get_solution().get(&1).unwrap();

    assert_eq!(solution_v1[0], 0);
    assert_eq!(solution_v1[1], 2);
    assert_eq!(solution_v1[2], 3);

    assert_eq!(solution_v2[0], 0);
    assert_eq!(solution_v2[1], 1);
}

#[rstest]
fn greedy_solution_total_distance_is_correct(
    distances: DistanceMatrix,
    stops: Vec<Stop>,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(2);

    let mut solver = GreedySolver::new(vehicles, distances, stops);
    solver.solve();

    assert_eq!(solver.solution_total_distance(), 10.0);
}

#[rstest]
fn cannot_get_infeasible_near_stops(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(1);
    let mut route_service = RouteService::new(vehicles, distances, stops);

    route_service.assign_stop_to_route(0, 0).unwrap();

    let stop = route_service.get_nearest_stop(0).unwrap();

    assert_ne!(stop.get_id(), 4)
}

#[rstest]
fn the_vehicle_returned_to_depot(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) {
    let vehicles = vehicle_factory(2);

    let mut solver = GreedySolver::new(vehicles, distances, stops);
    solver.solve();

    let solution = solver.get_solution().get(&0).unwrap();

    let last_stop = *solution.last().unwrap();

    assert_eq!(last_stop, 0);
}