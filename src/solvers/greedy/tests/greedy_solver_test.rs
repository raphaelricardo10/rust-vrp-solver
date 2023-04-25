use crate::{solvers::solver::Solver, tests::fixtures::greedy_solver_fixture::GreedySolverFactory};
use rstest::rstest;

use crate::tests::fixtures::greedy_solver_fixture::greedy_solver_factory;

#[rstest]
fn greedy_solution_is_correct_single_vehicle(greedy_solver_factory: GreedySolverFactory) {
    let mut solver = greedy_solver_factory(1);

    let solution = solver.solve();
    let stops = &solution.routes.get(&0).unwrap().stops;

    assert_eq!(stops[0].id, 0);
    assert_eq!(stops[1].id, 2);
    assert_eq!(stops[2].id, 3);
    assert_eq!(stops[3].id, 1);
}

#[rstest]
fn greedy_solution_is_correct_multiple_vehicles(greedy_solver_factory: GreedySolverFactory) {
    let mut solver = greedy_solver_factory(2);

    let solution = solver.solve();

    let stops_v1 = &solution.routes.get(&0).unwrap().stops;
    let stops_v2 = &solution.routes.get(&1).unwrap().stops;

    assert_eq!(stops_v1[0].id, 0);
    assert_eq!(stops_v1[1].id, 2);
    assert_eq!(stops_v1[2].id, 3);

    assert_eq!(stops_v2[0].id, 0);
    assert_eq!(stops_v2[1].id, 1);
}

#[rstest]
fn greedy_solution_total_distance_is_correct(greedy_solver_factory: GreedySolverFactory) {
    let mut solver = greedy_solver_factory(2);

    let solution = solver.solve();

    assert_eq!(solution.total_distance, 10.0);
}

#[rstest]
fn the_vehicle_returned_to_depot(greedy_solver_factory: GreedySolverFactory) {
    let mut solver = greedy_solver_factory(1);

    let solution = solver.solve();
    let stops = &solution.routes.get(&0).unwrap().stops;

    let last_stop = *stops.last().unwrap();

    assert_eq!(last_stop.id, 0);
}
