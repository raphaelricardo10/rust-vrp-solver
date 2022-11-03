use rstest::rstest;

use crate::domain::{
    route::{DistanceMatrix, Route},
    stop::Stop,
    vehicle::Vehicle,
};

use super::fixtures::{distances, full_stops, stops};

#[rstest]
fn route_distance_calculation(stops: Vec<Stop>, distances: DistanceMatrix) {
    let mut vehicle = Vehicle::new(0, 10);

    let mut route = Route::new(&mut vehicle, &distances);

    route.add_stop(&stops[0]).unwrap();
    route.add_stop(&stops[1]).unwrap();
    route.add_stop(&stops[2]).unwrap();
    route.add_stop(&stops[3]).unwrap();

    assert_eq!(route.total_distance(), 9.0);
}

#[rstest]
fn route_cannot_overload_vehicle(full_stops: Vec<Stop>, distances: DistanceMatrix) {
    let mut vehicle = Vehicle::new(0, 10);

    let mut route = Route::new(&mut vehicle, &distances);

    route.add_stop(&full_stops[0]).unwrap();

    match route.add_stop(&full_stops[1]) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}
