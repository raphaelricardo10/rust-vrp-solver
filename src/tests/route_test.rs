use rstest::rstest;

use crate::domain::{
    route::{DistanceMatrix, Route},
    stop::Stop,
    vehicle::Vehicle,
};

use super::fixtures::{distances, full_stops, stops};

#[rstest]
fn route_distance_calculation(stops: Vec<Stop>, distances: DistanceMatrix) {
    let vehicle = Vehicle::new(0, 10);

    let mut route = Route::new(vehicle);

    route.add_stop(stops[0], 0.0).unwrap();
    route
        .add_stop(
            stops[1],
            *distances
                .get(&(stops[0].get_id(), stops[1].get_id()))
                .unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops[2],
            *distances
                .get(&(stops[1].get_id(), stops[2].get_id()))
                .unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops[3],
            *distances
                .get(&(stops[2].get_id(), stops[3].get_id()))
                .unwrap(),
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
