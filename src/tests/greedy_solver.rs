use rstest::{fixture, rstest};
use std::collections::HashMap;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    solvers::greedy::solver::GreedySolver,
};

type VehicleFactory = fn (number: u32) -> Vec<Vehicle>;

#[fixture]
fn distances() -> DistanceMatrix {
    HashMap::from([
        ((0, 1), 2.0),
        ((0, 2), 1.0),
        ((0, 3), 3.0),
        ((1, 0), 2.0),
        ((1, 2), 5.0),
        ((1, 3), 3.0),
        ((2, 0), 1.0),
        ((2, 1), 5.0),
        ((2, 3), 2.0),
        ((3, 0), 3.0),
        ((3, 1), 3.0),
        ((3, 2), 2.0),
    ])
}

#[fixture]
fn stops() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 0),
        Stop::new(1, 0),
        Stop::new(2, 0),
        Stop::new(3, 0),
    ])
}

#[fixture]
fn vehicle_factory() -> VehicleFactory {
    fn wrapper(number: u32) -> Vec<Vehicle> {
        let mut vehicles = Vec::new();
    
        for i in 0..number {
            vehicles.push(Vehicle::new(i, 10));
        }

        vehicles
    }

    wrapper
}

#[rstest]
fn greedy_solution_is_correct_single_vehicle(distances: DistanceMatrix, stops: Vec<Stop>, vehicle_factory: VehicleFactory) {
    let mut vehicles = vehicle_factory(1);

    let mut solver = GreedySolver::new(&mut vehicles, &distances, &stops);
    solver.solve();

    let solution = solver.get_solution().get(&0).unwrap();

    assert_eq!(solution[0], 0);
    assert_eq!(solution[1], 2);
    assert_eq!(solution[2], 3);
    assert_eq!(solution[3], 1);
}

#[rstest]
fn greedy_solution_is_correct_multiple_vehicles(distances: DistanceMatrix, stops: Vec<Stop>, vehicle_factory: VehicleFactory) {
    let mut vehicles = vehicle_factory(2);

    let mut solver = GreedySolver::new(&mut vehicles, &distances, &stops);
    solver.solve();

    let solution_v1 = solver.get_solution().get(&0).unwrap();
    let solution_v2 = solver.get_solution().get(&1).unwrap();

    assert_eq!(solution_v1[0], 0);
    assert_eq!(solution_v1[1], 2);
    assert_eq!(solution_v1[2], 3);

    assert_eq!(solution_v2[0], 0);
    assert_eq!(solution_v2[1], 1);
}
