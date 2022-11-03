use rstest::rstest;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop},
    services::route_service::RouteService,
    tests::fixtures::VehicleFactory,
};

use super::fixtures::{distances, stops, vehicle_factory};

#[rstest]
fn route_service_started(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let mut vehicles = vehicle_factory(2);

    let route_service = RouteService::new(&mut vehicles, &distances, &stops);

    assert_eq!(route_service.get_available_stops().len(), 4);
    assert_eq!(route_service.get_all_routes().len(), 2);
    assert_eq!(route_service.get_vehicles().len(), 2);
}

#[rstest]
fn can_assign_stop_to_route(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let mut vehicles = vehicle_factory(1);

    let mut route_service = RouteService::new(&mut vehicles, &distances, &stops);

    route_service.assign_stop_to_route(0, 0);

    assert_eq!(route_service.get_route(0).get_stops().len(), 1)
}

#[rstest]
fn can_get_nearest_stop(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let mut vehicles = vehicle_factory(1);

    let mut route_service = RouteService::new(&mut vehicles, &distances, &stops);
    route_service.assign_stop_to_route(0, 0);

    assert_eq!(route_service.get_nearest_stop(0).unwrap().get_id(), 2);
}
