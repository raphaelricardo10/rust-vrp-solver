use std::collections::HashMap;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    solvers::greedy::solver::GreedySolver,
};

#[test]
fn greedy_solution_is_correct() {
    let vehicle = Vehicle::new(0, 10);
    let mut vehicles = vec![vehicle];

    let mut stops: Vec<Stop> = Vec::new();
    stops.push(Stop::new(0, 0));
    stops.push(Stop::new(1, 0));
    stops.push(Stop::new(2, 0));
    stops.push(Stop::new(3, 0));

    let mut distances: DistanceMatrix = HashMap::new();
    distances.insert((0, 1), 2.0);
    distances.insert((0, 2), 1.0);
    distances.insert((0, 3), 3.0);

    distances.insert((1, 0), 2.0);
    distances.insert((1, 2), 5.0);
    distances.insert((1, 3), 3.0);

    distances.insert((2, 0), 1.0);
    distances.insert((2, 1), 5.0);
    distances.insert((2, 3), 2.0);

    distances.insert((3, 0), 3.0);
    distances.insert((3, 1), 3.0);
    distances.insert((3, 2), 2.0);

    let mut solver = GreedySolver::new(&mut vehicles, &distances, &stops);
    solver.solve();

    let solution = solver.get_solution();

    assert_eq!(solution[0], 0);
    assert_eq!(solution[1], 2);
    assert_eq!(solution[2], 3);
    assert_eq!(solution[3], 1);
}
