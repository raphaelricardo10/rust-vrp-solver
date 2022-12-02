use super::{distances_fixture::distances, stops_fixture::stops};
use crate::services::route::route_service::RouteService;
use crate::tests::fixtures::vehicles_fixture::vehicle_factory;

use rstest::fixture;

use crate::{
    domain::stop::Stop,
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

use super::vehicles_fixture::VehicleFactory;

pub type RouteServiceFactory = Box<dyn Fn(u32) -> RouteService>;

#[fixture]
pub fn distance_service(distances: DistanceMatrix, stops: Vec<Stop>) -> DistanceService {
    DistanceService::new(stops, &distances)
}

#[fixture]
pub fn route_service_factory(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) -> RouteServiceFactory {
    let wrapper = move |number_of_vehicles| -> RouteService {
        let vehicles = vehicle_factory(number_of_vehicles);

        RouteService::new(vehicles, &distances, stops[..stops.len()].to_vec())
    };

    Box::new(wrapper)
}
