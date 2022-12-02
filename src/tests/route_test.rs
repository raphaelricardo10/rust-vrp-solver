use rstest::rstest;

use crate::{
    domain::{route::Route, stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::DistanceService,
};

use super::fixtures::routes_fixture::{route_factory, RouteFactory};
use super::fixtures::services_fixture::distance_service;
use super::fixtures::stops_fixture::full_stops;
use super::fixtures::stops_fixture::stops;

#[rstest]
fn route_distance_calculation(stops: Vec<Stop>, route_factory: RouteFactory) {
    let route = route_factory(stops[0..4].to_vec());

    assert_eq!(route.total_distance(), 9.0);
}

#[rstest]
fn route_cannot_overload_vehicle(full_stops: Vec<Stop>, distance_service: DistanceService) {
    let vehicle = Vehicle::new(0, 10);

    let mut route = Route::new(vehicle);

    route.add_stop(full_stops[0], 0.0).unwrap();

    let distance = distance_service
        .get_distance(&full_stops[0], &full_stops[1])
        .unwrap();

    if route.add_stop(full_stops[1], distance).is_ok() {
        panic!();
    }
}
