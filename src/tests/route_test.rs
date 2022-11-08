use rstest::rstest;

use crate::{
    domain::{route::Route, stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

use super::fixtures::{distance_service, distances, full_stops, stops};

#[rstest]
fn route_distance_calculation(stops: Vec<Stop>, distance_service: DistanceService) {
    let vehicle = Vehicle::new(0, 10);

    let mut route = Route::new(vehicle);

    route.add_stop(stops[0], 0.0).unwrap();
    route
        .add_stop(
            stops[1],
            distance_service.get_distance(&stops[0], &stops[1]).unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops[2],
            distance_service.get_distance(&stops[1], &stops[2]).unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops[3],
            distance_service.get_distance(&stops[2], &stops[3]).unwrap(),
        )
        .unwrap();

    assert_eq!(route.total_distance(), 9.0);
}

#[rstest]
fn route_cannot_overload_vehicle(full_stops: Vec<Stop>, distances: DistanceMatrix) {
    let vehicle = Vehicle::new(0, 10);

    let mut route = Route::new(vehicle);

    route.add_stop(full_stops[0], 0.0).unwrap();
    let distance = *distances
        .get(&(full_stops[0].get_id(), full_stops[1].get_id()))
        .unwrap();

    match route.add_stop(full_stops[1], distance) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}
