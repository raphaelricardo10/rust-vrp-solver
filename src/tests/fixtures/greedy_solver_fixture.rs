use crate::solvers::greedy::greedy_solver::GreedySolver;
use rstest::fixture;

use crate::{domain::stop::Stop, services::distance::distance_service::DistanceMatrix};

use super::{
    distances_fixture::distances, stops_fixture::stops, vehicles_fixture::vehicle_factory,
    vehicles_fixture::VehicleFactory,
};

pub type GreedySolverFactory = Box<dyn Fn(u32) -> GreedySolver>;

#[fixture]
pub fn greedy_solver_factory(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) -> GreedySolverFactory {
    let wrapper = move |number_of_vehicles: u32| -> GreedySolver {
        let vehicles = vehicle_factory(number_of_vehicles);

        GreedySolver::new(vehicles, &distances, stops.clone())
    };

    Box::new(wrapper)
}
