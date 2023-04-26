use crate::solvers::greedy::vrp_greedy_solver::VrpGreedySolver;
use rstest::fixture;

use crate::{domain::stop::Stop, services::distance::distance_service::DistanceMatrix};

use super::{
    distances_fixture::distances, stops_fixture::stops, vehicles_fixture::vehicle_factory,
    vehicles_fixture::VehicleFactory,
};

pub type GreedySolverFactory = Box<dyn Fn(u32) -> VrpGreedySolver>;

#[fixture]
pub fn greedy_solver_factory(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) -> GreedySolverFactory {
    let wrapper = move |number_of_vehicles: u32| -> VrpGreedySolver {
        let vehicles = vehicle_factory(number_of_vehicles);

        VrpGreedySolver::new(vehicles, &distances, stops.clone())
    };

    Box::new(wrapper)
}
